use std::{sync::LazyLock, time::Duration};

use bitflags::Flags;
use dashmap::DashMap;
use parking_lot::RwLock;
use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable},
};

use crate::{
    EventHookTypes, EventName,
    abi::lifecycle::{Lifecycle, ReplacementNotification},
    engine::queue,
    utils::{fails, warns},
};

const ALLOWED_CONTENTION: Duration = Duration::from_millis(20);

type Mutes = LazyLock<RwLock<DashMap<EventName, EventHookTypes>>>;

static MUTES: Mutes = LazyLock::new(|| RwLock::new(DashMap::with_capacity(1024)));

pub struct Replacements;

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
        Replacements.is_muted(event_name)
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
        Replacements.is_specific_muted(event_name, event_type)
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

impl Mute for Replacements {
    type Name = EventName;
    fn mute(&self, event_name: EventName) {
        if let Some(set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            set.entry(event_name)
                .and_modify(|x| *x = EventHookTypes::all())
                .or_insert(EventHookTypes::all());
        } else {
            warns!("write contention on muted event names store (mute: {event_name})");
        }
    }

    fn is_muted(&self, event_name: EventName) -> bool {
        if let Some(set) = MUTES.try_read_for(ALLOWED_CONTENTION) {
            for x in set.iter() {
                if *x.key() == event_name {
                    return true;
                }
            }
            return false;
        }
        warns!("read contention on muted event names store ({event_name})");
        false
    }

    fn mute_specific(&self, event_name: EventName, event_type: EventHookTypes) {
        if let Some(ref mut set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            set.entry(event_name)
                .and_modify(|x| x.set(event_type, true))
                .or_insert(event_type);
        } else {
            warns!(
                "write contention on muted event names store (mute: {event_name}, {event_type})"
            );
        }
    }

    fn is_specific_muted(&self, event_name: EventName, event_type: EventHookTypes) -> bool {
        if let Some(set) = MUTES.try_read_for(ALLOWED_CONTENTION) {
            for x in set.iter() {
                if *x.key() == event_name && (*x.value()).contains(event_type) {
                    return true;
                }
            }
            return false;
        }
        warns!("read contention on muted event names store ({event_name}, {event_type})");
        false
    }

    fn unmute(&self, event_name: EventName) {
        if let Some(ref mut set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            set.remove(&event_name);
        } else {
            warns!("write contention on muted event names store (unmute: {event_name})");
        }
    }

    fn unmute_specific(&self, event_name: EventName, event_type: EventHookTypes) {
        if let Some(ref mut set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            match set.entry(event_name) {
                dashmap::Entry::Occupied(mut x) => {
                    let empty = {
                        let v = x.get_mut();
                        v.set(event_type, false);
                        v.is_empty()
                    };
                    if empty {
                        x.remove_entry();
                    }
                }
                dashmap::Entry::Vacant(_) => {}
            };
        } else {
            warns!(
                "write contention on muted event names store (unmute: {event_name}, {event_type})"
            );
        }
    }
}
