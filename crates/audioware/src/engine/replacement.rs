use bitflags::Flags;
use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable},
};

use crate::{
    EventHookTypes, EventName,
    abi::lifecycle::{Lifecycle, ReplacementNotification},
    engine::{
        callbacks::{publish_muted, with_muted},
        queue,
    },
    utils::{fails, warns},
};

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
        let mut len = 0;
        with_muted(|x| {
            len = x.len();
        });
        let mut new: Vec<(EventName, EventHookTypes)> = Vec::with_capacity(len);
        with_muted(|x| {
            new.copy_from_slice(x);
        });
        if let Some(x) = new.iter().position(|x| x.0 == event_name) {
            let x = new.get_mut(x).unwrap();
            x.1.set(EventHookTypes::all(), true);
        } else {
            new.push((event_name, EventHookTypes::all()));
        }
        publish_muted(new);
    }

    fn is_muted(&self, event_name: EventName) -> bool {
        let mut is_muted = false;
        with_muted(|x| {
            is_muted = x.contains(&(event_name, EventHookTypes::all()));
        });
        is_muted
    }

    fn mute_specific(&self, event_name: EventName, event_type: EventHookTypes) {
        let mut len = 0;
        with_muted(|x| {
            len = x.len();
        });
        let mut new: Vec<(EventName, EventHookTypes)> = Vec::with_capacity(len);
        with_muted(|x| {
            new.copy_from_slice(x);
        });
        if let Some(x) = new.iter().position(|x| x.0 == event_name) {
            let x = new.get_mut(x).unwrap();
            x.1.insert(event_type);
        } else {
            new.push((event_name, event_type));
        }
        publish_muted(new);
    }

    fn is_specific_muted(&self, event_name: EventName, event_type: EventHookTypes) -> bool {
        let mut is_muted = false;
        with_muted(|x| {
            is_muted = x.contains(&(event_name, event_type));
        });
        is_muted
    }

    fn unmute(&self, event_name: EventName) {
        let mut new: Vec<(EventName, EventHookTypes)> = vec![];
        with_muted(|x| {
            new.copy_from_slice(x);
        });
        if let Some(x) = new.iter().position(|x| x.0 == event_name) {
            new.remove(x);
            publish_muted(new);
        }
    }

    fn unmute_specific(&self, event_name: EventName, event_type: EventHookTypes) {
        let mut new: Vec<(EventName, EventHookTypes)> = vec![];
        with_muted(|x| {
            new.copy_from_slice(x);
        });
        if let Some(x) = new.iter().position(|x| x.0 == event_name) {
            let x = new.get_mut(x).unwrap();
            x.1.set(event_type, false);
            publish_muted(new);
        }
    }
}
