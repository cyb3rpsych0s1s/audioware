use std::{
    cell::{Cell, UnsafeCell},
    collections::VecDeque,
    sync::{
        LazyLock, OnceLock,
        atomic::{AtomicPtr, AtomicU64, AtomicUsize, Ordering},
    },
};

use kira::backend::Backend;
use red4ext_rs::{
    InvokeError, RttiSystem, ScriptClass, ScriptClassOps,
    types::{CName, Function, IScriptable, Ref, TaggedType, WeakRef},
};

use crate::{
    AddContainerStreamingPrefetchEvent, AnyTarget, AudioEventCallbackHandler,
    AudioEventCallbackSystem, ClassName, EngineSoundEvent, EventActionType, EventName,
    FunctionName, PlayEvent, PlayExternalEvent, PlayOneShotEvent,
    RemoveContainerStreamingPrefetchEvent, SetAppearanceNameEvent, SetEntityNameEvent,
    SetGlobalParameterEvent, SetParameterEvent, SetSwitchEvent, StopSoundEvent, StopTaggedEvent,
    TagEvent, UntagEvent,
    abi::callback::{Callback, FireCallback},
    engine::{Engine, queue},
    utils::{fails, lifecycle, warns},
};

#[repr(C, align(64))]
struct Header {
    ptr: *const (Key, AudioEventCallback),
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

static CURRENT_CB: AtomicPtr<Header> = AtomicPtr::new(std::ptr::null_mut());
static GENERATION_CB: AtomicU64 = AtomicU64::new(0);

static RETIRED_CB: OnceLock<Retired> = OnceLock::new();

thread_local! {
    static TLS_PTR_CB: Cell<*const (Key, AudioEventCallback)> = const { Cell::new(std::ptr::null_mut()) };
    static TLS_LEN_CB: Cell<usize> = const { Cell::new(0) };
    static TLS_GEN_CB: Cell<u64> = const { Cell::new(0) };
}

fn retired() -> &'static Retired {
    RETIRED_CB.get_or_init(|| Retired {
        list: UnsafeCell::new(VecDeque::new()),
    })
}

#[inline]
pub(crate) fn with_callbacks<F: FnOnce(&[(Key, AudioEventCallback)])>(f: F) {
    TLS_GEN_CB.with(|g| {
        let generation = GENERATION_CB.load(Ordering::SeqCst);
        if g.get() != generation {
            let h = CURRENT_CB.load(Ordering::SeqCst);
            if !h.is_null() {
                unsafe {
                    TLS_PTR_CB.set((*h).ptr);
                    TLS_LEN_CB.set((*h).len);
                    g.set(generation);
                }
            }
        }
        let ptr = TLS_PTR_CB.get();
        let len = TLS_LEN_CB.get();
        if !ptr.is_null() {
            unsafe { f(std::slice::from_raw_parts(ptr, len)) }
        }
    });
}

pub(crate) fn publish_callbacks(mut data: Vec<(Key, AudioEventCallback)>) {
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
    let prev = CURRENT_CB.swap(header, Ordering::SeqCst);
    let generation = GENERATION_CB.fetch_add(1, Ordering::SeqCst) + 1;
    if !prev.is_null() {
        unsafe {
            let list = &mut *retired().list.get();
            list.push_back((prev, generation));
        }
    }
}

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
pub(crate) fn reclaim_callbacks() {
    let min_gen = GENERATION_CB.load(Ordering::SeqCst);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(usize);

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cb:{}", self.0)
    }
}

pub trait Dispatch {
    fn dispatch(&self, fire: FireCallback);
}

pub trait Call {
    fn call<T: ScriptClassOps>(&self, ctor: impl FnOnce(&mut T) + Copy);
}

impl AudioEventCallbackSystem {
    #[inline]
    pub fn any_callback(event_name: EventName, event_type: EventActionType) -> bool {
        let mut exists = false;
        with_callbacks(|x| exists = x.iter().any(|x| x.1.matches(event_name, event_type)));
        exists
    }
    #[inline]
    pub fn is_registered(handler_id: usize) -> bool {
        let mut exists = false;
        with_callbacks(|x| exists = x.iter().any(|x| x.0 == Key(handler_id)));
        exists
    }
    pub fn dispatch<T: Into<FireCallback>>(event: T) {
        queue::forward(Callback::FireCallbacks(event.into()));
    }
    pub fn register_callback(
        &self,
        event_name: CName,
        target: Ref<IScriptable>,
        function_name: CName,
    ) -> Ref<AudioEventCallbackHandler> {
        let Ok(event_name) = EventName::try_from(event_name) else {
            warns!("event_name is invalid ({})", event_name.as_str());
            return Ref::default();
        };
        let Ok(function_name) = FunctionName::try_from(function_name) else {
            warns!("function_name is invalid ({})", function_name.as_str());
            return Ref::default();
        };
        if target.is_null() {
            warns!("target is null");
            return Ref::default();
        }
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        queue::forward(Callback::RegisterFunction {
            event_name,
            target: target.downgrade().into(),
            function_name,
            id,
        });
        Ref::<AudioEventCallbackHandler>::new_with(|x| {
            x.id.set(id);
        })
        .unwrap_or_default()
    }
    pub fn register_static_callback(
        &self,
        event_name: CName,
        class_name: CName,
        function_name: CName,
    ) -> Ref<AudioEventCallbackHandler> {
        let Ok(event_name) = EventName::try_from(event_name) else {
            warns!("event_name is invalid ({})", event_name.as_str());
            return Ref::default();
        };
        let Ok(class_name) = ClassName::try_from(class_name) else {
            warns!("class_name is invalid ({})", class_name.as_str());
            return Ref::default();
        };
        let Ok(function_name) = FunctionName::try_from(function_name) else {
            warns!("function_name is invalid ({})", function_name.as_str());
            return Ref::default();
        };
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        queue::forward(Callback::RegisterStaticFunction {
            event_name,
            class_name,
            function_name,
            id,
        });
        Ref::<AudioEventCallbackHandler>::new_with(|x| {
            x.id.set(id);
        })
        .unwrap_or_default()
    }
}

impl Dispatch for CallbackFunction {
    fn dispatch(&self, fire: FireCallback) {
        match self {
            CallbackFunction::Member(x) => x.dispatch(fire),
            CallbackFunction::Static(x) => x.dispatch(fire),
        }
    }
}

#[derive(Clone)]
pub struct AudioEventCallback {
    callback: CallbackFunction,
    targets: Vec<AnyTarget>,
    event_name: EventName,
    sticky: bool,
}

impl Dispatch for AudioEventCallback {
    fn dispatch(&self, fire: FireCallback) {
        self.callback.dispatch(fire);
    }
}

#[derive(Clone)]
pub enum CallbackFunction {
    Member(MemberFunc),
    Static(StaticFunc),
}

#[derive(Clone)]
pub struct MemberFunc {
    target: WeakRef<IScriptable>,
    function_name: FunctionName,
}

#[derive(Clone)]
pub struct StaticFunc {
    class_name: ClassName,
    function_name: FunctionName,
}

impl<T> Dispatch for T
where
    T: Call,
{
    fn dispatch(&self, fire: FireCallback) {
        match &fire {
            FireCallback::Play(f) => self.call(|x: &mut PlayEvent| x.hydrate(f)),
            FireCallback::PlayOneShot(f) => self.call(|x: &mut PlayOneShotEvent| x.hydrate(f)),
            FireCallback::PlayExternal(f) => self.call(|x: &mut PlayExternalEvent| x.hydrate(f)),
            FireCallback::SetGlobalParameter(f) => {
                self.call(|x: &mut SetGlobalParameterEvent| x.hydrate(f))
            }
            FireCallback::SetParameter(f) => self.call(|x: &mut SetParameterEvent| x.hydrate(f)),
            FireCallback::SetSwitch(f) => self.call(|x: &mut SetSwitchEvent| x.hydrate(f)),
            FireCallback::SetAppearanceName(f) => {
                self.call(|x: &mut SetAppearanceNameEvent| x.hydrate(f))
            }
            FireCallback::SetEntityName(f) => self.call(|x: &mut SetEntityNameEvent| x.hydrate(f)),
            FireCallback::Stop(f) => self.call(|x: &mut StopSoundEvent| x.hydrate(f)),
            FireCallback::StopTagged(f) => self.call(|x: &mut StopTaggedEvent| x.hydrate(f)),
            FireCallback::Tag(f) => self.call(|x: &mut TagEvent| x.hydrate(f)),
            FireCallback::Untag(f) => self.call(|x: &mut UntagEvent| x.hydrate(f)),
            FireCallback::AddContainerStreamingPrefetch(f) => {
                self.call(|x: &mut AddContainerStreamingPrefetchEvent| x.hydrate(f))
            }
            FireCallback::RemoveContainerStreamingPrefetch(f) => {
                self.call(|x: &mut RemoveContainerStreamingPrefetchEvent| x.hydrate(f))
            }
        }
    }
}

impl Call for MemberFunc {
    fn call<T: ScriptClassOps>(&self, ctor: impl FnOnce(&mut T) + Copy) {
        let rtti = RttiSystem::get();
        if let Some(target) = self.target.clone().upgrade()
            && let Some(class_name) = unsafe { target.instance() }.map(|x| x.class().name())
        {
            if let Some(class) = rtti.get_class(class_name) {
                if let Some(func) = class.methods().iter().map(|x| x.as_function()).find(|x| {
                    x.name() == *self.function_name && x.flags().is_event() && x.params().len() == 1
                }) {
                    inner_call::<T>(unsafe { target.fields() }, func, ctor);
                }
            } else {
                warns!("could not find callback class");
            }
        }
    }
}

impl Call for StaticFunc {
    fn call<T: ScriptClassOps>(&self, ctor: impl FnOnce(&mut T) + Copy) {
        let rtti = RttiSystem::get();
        let full_name = format!(
            "{}::{}",
            (*self.class_name).as_str(),
            (*self.function_name).as_str()
        );
        if let Some(func) = rtti.get_global_functions().iter().find(|x| {
            x.name().as_str() == full_name && x.flags().is_event() && x.params().len() == 1
        }) {
            inner_call::<T>(None, func, ctor);
        } else {
            warns!("could not find static callback {full_name}");
        }
    }
}

fn inner_call<T: ScriptClass>(
    receiver: Option<&IScriptable>,
    func: &Function,
    ctor: impl FnOnce(&mut T) + Copy,
) {
    let Some(event) = T::new_ref_with(|x| {
        ctor(x);
    }) else {
        return;
    };
    let Some(param) = func.params().first() else {
        fails!("callback expects a single event param");
        return;
    };
    let TaggedType::Ref(param_tag) = param.type_().tagged() else {
        fails!("callback event param should be a ref");
        return;
    };
    let Some(param_class) = param_tag.pointee().as_class() else {
        fails!("callback event param ref should point to a class");
        return;
    };
    if let Err(e) = match param_class.name().as_str() {
        x if x == T::NAME => func.execute::<_, ()>(receiver, (event,)),
        EngineSoundEvent::NAME => event
            .cast::<EngineSoundEvent>()
            .map(|x| func.execute::<_, ()>(receiver, (x,)))
            .unwrap_or(Err(InvokeError::ArgMismatch {
                function: func.name().as_str(),
                expected: EngineSoundEvent::NAME,
                index: 0,
            })),
        _ => Err(InvokeError::ArgMismatch {
            function: func.name().as_str(),
            expected: T::NAME,
            index: 0,
        }),
    } {
        fails!("couldn't execute callback: {e}");
    }
}

impl<B: Backend> Dispatch for Engine<B> {
    fn dispatch(&self, fire: FireCallback) {
        with_callbacks(|x| {
            x.iter()
                .filter(|x| x.1.matches_filters(&fire))
                .for_each(|x| {
                    x.1.dispatch(fire.clone());
                });
        });
    }
}

impl AudioEventCallback {
    pub fn matches(&self, event_name: EventName, event_type: EventActionType) -> bool {
        self.event_name == event_name
            && (self
                .targets
                .iter()
                .filter(|t| matches!(t, AnyTarget::Type(_)))
                .count()
                == 0
                || self.targets.iter().any(|t| match t {
                    AnyTarget::Type(x) => *x == event_type,
                    _ => false,
                }))
    }
    fn matches_filters(&self, other: &FireCallback) -> bool {
        if self.event_name != other.event_name() {
            return false;
        }
        if self.targets.is_empty() {
            return true;
        }
        for target in self.targets.iter() {
            match target {
                AnyTarget::Hook(x) if other.event_hook_type() == *x => return true,
                AnyTarget::Type(x) if other.event_type() == *x => return true,
                AnyTarget::Wwise(x) if other.wwise_id() == *x => return true,
                AnyTarget::Id(x) => {
                    if let Some(entity_id) = other.entity_id()
                        && entity_id == *x
                    {
                        return true;
                    }
                }
                AnyTarget::EmitterName(x) => {
                    if let Some(emitter_name) = other.emitter_name()
                        && emitter_name == *x
                    {
                        return true;
                    }
                }
                _ => {}
            };
        }
        false
    }
}

impl<B: Backend> Engine<B> {
    pub fn update_callbacks(&mut self) {
        if self.pending_callbacks.is_empty() {
            return;
        }
        let mut next = vec![];
        with_callbacks(|x| {
            next = x.to_vec();
        });
        let mut should_publish = false;
        for pending in self.pending_callbacks.drain(..) {
            match pending {
                Callback::RegisterFunction {
                    event_name,
                    target,
                    function_name,
                    id,
                } => {
                    if next.iter().any(|x| x.0 == Key(id)) {
                        warns!(
                            "{id} callback already registered for {event_name} => {function_name}"
                        );
                        continue;
                    }
                    next.push((
                        Key(id),
                        AudioEventCallback {
                            callback: CallbackFunction::Member(MemberFunc {
                                target: target.0,
                                function_name,
                            }),
                            targets: Vec::with_capacity(5),
                            event_name,
                            sticky: false,
                        },
                    ));
                    should_publish = true;
                    lifecycle!("{id} registered new callback for {event_name} => {function_name}",);
                }
                Callback::RegisterStaticFunction {
                    event_name,
                    class_name,
                    function_name,
                    id,
                } => {
                    if next.iter().any(|x| x.0 == Key(id)) {
                        warns!(
                            "{id} static callback already registered for {event_name} => {function_name}"
                        );
                        continue;
                    }
                    next.push((
                        Key(id),
                        AudioEventCallback {
                            callback: CallbackFunction::Static(StaticFunc {
                                class_name,
                                function_name,
                            }),
                            targets: Vec::with_capacity(5),
                            event_name,
                            sticky: false,
                        },
                    ));
                    should_publish = true;
                    lifecycle!(
                        "{id} registered new static callback for {event_name} => {function_name}",
                    );
                }
                Callback::FireCallbacks(_) => continue,
                Callback::Unregister { id } => {
                    let Some(idx) = next.iter().position(|x| x.0 == Key(id)) else {
                        continue;
                    };
                    next.remove(idx);
                    should_publish = true;
                }
                Callback::Filter { id, target, add } => match add {
                    true => {
                        let Some(x) = next
                            .iter_mut()
                            .rfind(|x| x.0 == Key(id) && !x.1.targets.contains(&target))
                        else {
                            continue;
                        };
                        x.1.targets.push(target);
                        should_publish = true;
                    }
                    false => {
                        next.retain_mut(|x| {
                            if x.0 != Key(id) {
                                true
                            } else if let Some(idx) = x.1.targets.iter().rposition(|x| *x == target)
                            {
                                x.1.targets.remove(idx);
                                should_publish = true;
                                !x.1.targets.is_empty()
                            } else {
                                true
                            }
                        });
                    }
                },
                Callback::SetLifetime { id, sticky } => {
                    if let Some(x) = next.iter_mut().rfind(|x| x.0 == Key(id))
                        && x.1.sticky != sticky
                    {
                        x.1.sticky = sticky;
                        should_publish = true;
                    }
                }
            }
        }
        if should_publish {
            publish_callbacks(next);
        }
    }
    pub fn reset_callbacks(&mut self) {
        let mut next = vec![];
        with_callbacks(|x| {
            next = x.to_vec();
        });
        let before = next.len();
        next.retain(|x| x.1.sticky);
        let after = next.len();
        if before != after {
            publish_callbacks(next);
        }
    }
}
