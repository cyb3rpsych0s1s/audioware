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
    cache::cache,
    engine::{
        Engine,
        mutes::cache::{publish_entries, reclaim_entries, with_entries},
        queue,
    },
    utils::{fails, warns},
};

cache!(crate::EventName, crate::EventHookTypes);

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
        with_entries(|x| {
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
        with_entries(|x| {
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
        with_entries(|x| {
            next = x.to_vec();
        });
        let mut should_publish = false;
        for pending in self.pending_mutes.drain(..) {
            match pending {
                ReplacementNotification::Mute(event_name) => {
                    let Some(idx) = next.iter().rposition(|x| x.0 == event_name) else {
                        next.push((event_name, EventHookTypes::all()));
                        should_publish = true;
                        continue;
                    };
                    let mute = next.get_mut(idx).unwrap();
                    if !mute.1.is_all() {
                        mute.1.set(EventHookTypes::all(), true);
                        should_publish = true;
                    }
                }
                ReplacementNotification::MuteSpecific(event_name, event_hook_types) => {
                    let Some(idx) = next.iter().rposition(|x| x.0 == event_name) else {
                        next.push((event_name, event_hook_types));
                        should_publish = true;
                        continue;
                    };
                    let mute = next.get_mut(idx).unwrap();
                    if !mute.1.contains(event_hook_types) {
                        mute.1.set(event_hook_types, true);
                        should_publish = true;
                    }
                }
                ReplacementNotification::Unmute(event_name) => {
                    let before = next.len();
                    next.retain(|x| x.0 != event_name);
                    let after = next.len();
                    should_publish = before != after;
                }
                ReplacementNotification::UnmuteSpecific(event_name, event_hook_types) => {
                    let before = next.len();
                    next.retain_mut(|x| {
                        if x.0 != event_name {
                            true
                        } else if x.1.intersection(event_hook_types).is_empty() {
                            false
                        } else {
                            x.1.set(event_hook_types, false);
                            should_publish = true;
                            true
                        }
                    });
                    let after = next.len();
                    should_publish = before != after;
                }
            }
        }
        if should_publish {
            publish_entries(next);
        }
    }
    pub fn is_specific_muted(event_name: EventName, event_type: EventHookTypes) -> bool {
        let mut muted = false;
        with_entries(|x| {
            muted = x
                .iter()
                .any(|x| x.0 == event_name && x.1.contains(event_type));
        });
        muted
    }
    pub fn reclaim_mutes(&mut self) {
        reclaim_entries();
    }
}
