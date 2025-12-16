use std::{sync::LazyLock, time::Duration};

use dashmap::DashMap;
use parking_lot::RwLock;
use red4ext_rs::{ScriptClass, class_kind::Native, types::IScriptable};

use crate::{EventActionType, EventActionTypes, EventName, utils::warns};

const ALLOWED_CONTENTION: Duration = Duration::from_millis(20);

type Mutes = LazyLock<RwLock<DashMap<EventName, EventActionTypes>>>;

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
    fn mute(&self, event_name: EventName);
    fn mute_specific(&self, event_name: EventName, event_type: EventActionType);
    fn is_muted(&self, event_name: EventName) -> bool;
    fn is_specific_muted(&self, event_name: EventName, event_type: EventActionType) -> bool;
}

impl Mute for AudioEventManager {
    #[inline]
    fn mute(&self, event_name: EventName) {
        Replacements.mute(event_name);
    }

    #[inline]
    fn is_muted(&self, event_name: EventName) -> bool {
        Replacements.is_muted(event_name)
    }

    #[inline]
    fn mute_specific(&self, event_name: EventName, event_type: EventActionType) {
        Replacements.mute_specific(event_name, event_type);
    }

    #[inline]
    fn is_specific_muted(&self, event_name: EventName, event_type: EventActionType) -> bool {
        Replacements.is_specific_muted(event_name, event_type)
    }
}

impl Mute for Replacements {
    fn mute(&self, event_name: EventName) {
        if let Some(set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            set.entry(event_name)
                .and_modify(|x| *x = EventActionTypes::all())
                .or_insert(EventActionTypes::all());
        } else {
            warns!("write contention on muted event names store ({event_name})");
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

    fn mute_specific(&self, event_name: EventName, event_type: EventActionType) {
        if let Some(ref mut set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            set.entry(event_name)
                .and_modify(|x| x.set(event_type.into(), true))
                .or_insert(event_type.into());
        } else {
            warns!("write contention on muted event names store ({event_name}, {event_type})");
        }
    }

    fn is_specific_muted(&self, event_name: EventName, event_type: EventActionType) -> bool {
        if let Some(set) = MUTES.try_read_for(ALLOWED_CONTENTION) {
            for x in set.iter() {
                if *x.key() == event_name && (*x.value()).contains(event_type.into()) {
                    return true;
                }
            }
            return false;
        }
        warns!("read contention on muted event names store ({event_name})");
        false
    }
}
