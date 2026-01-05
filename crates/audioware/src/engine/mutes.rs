use std::{
    cell::{Cell, UnsafeCell},
    collections::VecDeque,
    sync::{
        OnceLock,
        atomic::{AtomicPtr, AtomicU64, Ordering},
    },
};

use bitflags::Flags;
use kira::backend::Backend;
use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable},
};

use crate::{
    EventHookTypes, EventName,
    abi::lifecycle::{Lifecycle, ReplacementNotification},
    engine::{Engine, queue},
    utils::{fails, warns},
};

#[repr(C, align(64))]
struct Header {
    ptr: *const (EventName, EventHookTypes),
    len: usize,
    _pad: [u64; 6],
}

struct Retired {
    list: UnsafeCell<VecDeque<(*mut Header, u64)>>,
}

/// Safety: single-writer invariant
unsafe impl Sync for Header {}
/// Safety: single-writer invariant
unsafe impl Send for Header {}
/// Safety: single-writer invariant
unsafe impl Sync for Retired {}
/// Safety: single-writer invariant
unsafe impl Send for Retired {}

static CURRENT: AtomicPtr<Header> = AtomicPtr::new(std::ptr::null_mut());
static GENERATION: AtomicU64 = AtomicU64::new(0);

static RETIRED: OnceLock<Retired> = OnceLock::new();

thread_local! {
    static TLS_PTR: Cell<*const (EventName, EventHookTypes)> = const { Cell::new(std::ptr::null_mut()) };
    static TLS_LEN: Cell<usize> = const { Cell::new(0) };
    static TLS_GEN: Cell<u64> = const { Cell::new(0) };
}

fn retired() -> &'static Retired {
    RETIRED.get_or_init(|| Retired {
        list: UnsafeCell::new(VecDeque::new()),
    })
}

pub(crate) fn with_muted<F: FnOnce(&[(EventName, EventHookTypes)])>(f: F) {
    TLS_GEN.with(|g| {
        let generation = GENERATION.load(Ordering::Acquire);
        if g.get() != generation {
            let h = CURRENT.load(Ordering::Acquire);
            if !h.is_null() {
                unsafe {
                    TLS_PTR.set((*h).ptr);
                    TLS_LEN.set((*h).len);
                    g.set(generation);
                }
            }
        }
        let ptr = TLS_PTR.get();
        let len = TLS_LEN.get();
        if !ptr.is_null() {
            unsafe { f(std::slice::from_raw_parts(ptr, len)) }
        }
    });
}

pub(crate) fn publish_muted(mut data: Vec<(EventName, EventHookTypes)>) {
    if data.capacity() < data.len() * 2 {
        data.reserve(data.len());
    }

    let ptr = data.as_ptr();
    let len = data.len();
    std::mem::forget(data);
    let header = Box::into_raw(Box::new(Header {
        ptr,
        len,
        _pad: [0; 6],
    }));
    let prev = CURRENT.swap(header, Ordering::Release);
    let generation = GENERATION.fetch_add(1, Ordering::Release) + 1;
    if !prev.is_null() {
        unsafe {
            let list = &mut *retired().list.get();
            list.push_back((prev, generation));
        }
    }
}

pub(crate) fn reclaim_muted() {
    let min_gen = GENERATION.load(Ordering::Acquire);
    unsafe {
        let list = &mut *retired().list.get();
        while let Some(&(hdr, generation)) = list.front() {
            if generation + 2 < min_gen {
                let h = Box::from_raw(hdr);
                let _ = Vec::from_raw_parts(h.ptr as *mut u64, h.len, h.len);
                list.pop_front();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioEventManager {
    base: IScriptable,
}

unsafe impl ScriptClass for AudioEventManager {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AudioEventManager";
}

pub trait Mute {
    type Name;
    fn mute(&self, event_name: Self::Name);
    fn unmute(&self, event_name: Self::Name);
    fn mute_specific(&self, event_name: Self::Name, event_type: EventHookTypes);
    fn unmute_specific(&self, event_name: Self::Name, event_type: EventHookTypes);
    fn is_muted(&self, event_name: Self::Name) -> bool;
    fn is_specific_muted(&self, event_name: Self::Name, event_type: EventHookTypes) -> bool;
}

impl Mute for AudioEventManager {
    type Name = CName;
    #[inline]
    fn mute(&self, event_name: CName) {
        let Ok(event_name) = event_name.try_into() else {
            warns!("mute: invalid event name ({event_name})");
            return;
        };
        queue::notify(Lifecycle::Replacement(ReplacementNotification::Mute(
            event_name,
        )));
    }

    #[inline]
    fn is_muted(&self, event_name: CName) -> bool {
        let Ok(event_name) = event_name.try_into() else {
            return false;
        };
        let mut muted = false;
        with_muted(|x| {
            muted = x.iter().any(|x| x.0 == event_name);
        });
        muted
    }

    #[inline]
    fn mute_specific(&self, event_name: CName, event_type: EventHookTypes) {
        if event_type.contains_unknown_bits() {
            fails!("mute specific: invalid event type flag(s)");
            return;
        }
        let Ok(event_name) = event_name.try_into() else {
            warns!("mute specific: invalid event name ({event_name})");
            return;
        };
        queue::notify(Lifecycle::Replacement(
            ReplacementNotification::MuteSpecific(event_name, event_type),
        ));
    }

    #[inline]
    fn is_specific_muted(&self, event_name: CName, event_type: EventHookTypes) -> bool {
        if event_type.contains_unknown_bits() {
            fails!("is specific muted: invalid event type flag(s)");
            return false;
        }
        let Ok(event_name) = event_name.try_into() else {
            return false;
        };
        let mut muted = false;
        with_muted(|x| {
            muted = x
                .iter()
                .any(|x| x.0 == event_name && x.1.contains(event_type));
        });
        muted
    }

    #[inline]
    fn unmute(&self, event_name: CName) {
        let Ok(event_name) = event_name.try_into() else {
            warns!("unmute: invalid event name ({event_name})");
            return;
        };
        queue::notify(Lifecycle::Replacement(ReplacementNotification::Unmute(
            event_name,
        )));
    }

    #[inline]
    fn unmute_specific(&self, event_name: CName, event_type: EventHookTypes) {
        if event_type.contains_unknown_bits() {
            fails!("unmute specific: invalid event type flag(s)");
            return;
        }
        let Ok(event_name) = event_name.try_into() else {
            warns!("unmute specific: invalid event name ({event_name})");
            return;
        };
        queue::notify(Lifecycle::Replacement(
            ReplacementNotification::UnmuteSpecific(event_name, event_type),
        ));
    }
}

impl<B: Backend> Engine<B> {
    pub fn update_mutes(&mut self) {
        if self.pending_mutes.is_empty() {
            return;
        }
        let mut next = vec![];
        with_muted(|x| {
            next = x.to_vec();
        });
        for pending in self.pending_mutes.drain(..) {
            match pending {
                ReplacementNotification::Mute(event_name) => {
                    let Some(idx) = next.iter().rposition(|x| x.0 == event_name) else {
                        next.push((event_name, EventHookTypes::all()));
                        continue;
                    };
                    next.get_mut(idx)
                        .unwrap()
                        .1
                        .set(EventHookTypes::all(), true);
                }
                ReplacementNotification::MuteSpecific(event_name, event_hook_types) => {
                    let Some(idx) = next.iter().rposition(|x| x.0 == event_name) else {
                        next.push((event_name, EventHookTypes::all()));
                        continue;
                    };
                    next.get_mut(idx).unwrap().1.set(event_hook_types, true);
                }
                ReplacementNotification::Unmute(event_name) => next.retain(|x| x.0 != event_name),
                ReplacementNotification::UnmuteSpecific(event_name, event_hook_types) => next
                    .retain_mut(|x| {
                        if x.0 != event_name {
                            true
                        } else if x.1.intersection(event_hook_types).is_empty() {
                            false
                        } else {
                            x.1.set(event_hook_types, false);
                            true
                        }
                    }),
            }
        }
        publish_muted(next);
    }
    pub fn is_specific_muted(event_name: EventName, event_type: EventHookTypes) -> bool {
        let mut muted = false;
        with_muted(|x| {
            muted = x
                .iter()
                .any(|x| x.0 == event_name && x.1.contains(event_type));
        });
        muted
    }
}
