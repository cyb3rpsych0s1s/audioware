use std::{sync::LazyLock, time::Duration};

use dashmap::DashSet;
use parking_lot::RwLock;
use red4ext_rs::{ScriptClass, class_kind::Native, types::IScriptable};

use crate::{EventActionType, EventName, utils::warns};

const ALLOWED_CONTENTION: Duration = Duration::from_millis(20);

type Mutes = LazyLock<RwLock<DashSet<(EventName, Option<EventActionType>)>>>;

static MUTES: Mutes = LazyLock::new(|| RwLock::new(DashSet::with_capacity(1024)));

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
    fn is_muted(&self, event_name: EventName) -> bool;
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
}

impl Mute for Replacements {
    fn mute(&self, event_name: EventName) {
        if let Some(set) = MUTES.try_write_for(ALLOWED_CONTENTION) {
            set.insert((event_name, None));
        } else {
            warns!("write contention on muted event names store");
        }
    }

    fn is_muted(&self, event_name: EventName) -> bool {
        if let Some(set) = MUTES.try_read_for(ALLOWED_CONTENTION) {
            for mute in set.iter() {
                if mute.0 == event_name {
                    return true;
                }
            }
            return false;
        }
        warns!("read contention on muted event names store");
        false
    }
}
