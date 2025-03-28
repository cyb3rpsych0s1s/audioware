use red4ext_rs::{
    class_kind::Native,
    types::{CName, EntityId, IScriptable},
    ScriptClass,
};

use crate::{EntityGameInterface, Vector4};

use super::Event;

const PADDING_UNK78: usize = 0x80 - 0x78;

#[derive(Debug)]
#[repr(C)]
pub struct TriggerEvent {
    base: Event,
    pub trigger_id: EntityId,           // 0x40
    pub component_name: CName,          // 0x48
    pub activator: EntityGameInterface, // 0x50
    pub world_position: Vector4,        // 0x60
    pub num_activators_in_area: u32,    // 0x70
    pub activator_id: u32,              // 0x74
    unk78: [u8; PADDING_UNK78],         // 0x78
}

unsafe impl ScriptClass for TriggerEvent {
    const NAME: &'static str = "entTriggerEvent";
    type Kind = Native;
}

impl AsRef<Event> for TriggerEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for TriggerEvent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AreaEnteredEvent {
    base: TriggerEvent,
}

unsafe impl ScriptClass for AreaEnteredEvent {
    const NAME: &'static str = "entAreaEnteredEvent";
    type Kind = Native;
}

impl AsRef<TriggerEvent> for AreaEnteredEvent {
    #[inline]
    fn as_ref(&self) -> &TriggerEvent {
        &self.base
    }
}

impl AsRef<Event> for AreaEnteredEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for AreaEnteredEvent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AreaExitedEvent {
    base: TriggerEvent,
}

unsafe impl ScriptClass for AreaExitedEvent {
    const NAME: &'static str = "entAreaExitedEvent";
    type Kind = Native;
}

impl AsRef<TriggerEvent> for AreaExitedEvent {
    #[inline]
    fn as_ref(&self) -> &TriggerEvent {
        &self.base
    }
}

impl AsRef<Event> for AreaExitedEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for AreaExitedEvent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
