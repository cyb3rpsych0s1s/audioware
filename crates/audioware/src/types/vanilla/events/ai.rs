use core::fmt;

use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable, RedArray, WeakRef},
};

use super::Event;

const PADDING_4C: usize = 0x50 - 0x4C;

#[derive(Debug)]
#[repr(C)]
pub struct AIEvent {
    base: Event,
    pub name: CName,         // 40
    pub time_to_live: f32,   // 48
    unk4c: [u8; PADDING_4C], // 4C
}

unsafe impl ScriptClass for AIEvent {
    const NAME: &'static str = "AIAIEvent";
    type Kind = Native;
}

impl AsRef<Event> for AIEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for AIEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct ActionEvent {
    base: AIEvent,
    pub event_action: CName,                  // 50
    pub internal_event: WeakRef<IScriptable>, // 58 (unknown in RED4ext.SDK)
}

impl fmt::Debug for ActionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionEvent")
            .field("base", &self.base)
            .field("event_action", &self.event_action)
            .finish_non_exhaustive()
    }
}

unsafe impl ScriptClass for ActionEvent {
    type Kind = Native;
    const NAME: &'static str = "gameActionEvent";
}

impl AsRef<AIEvent> for ActionEvent {
    fn as_ref(&self) -> &AIEvent {
        &self.base
    }
}

impl AsRef<Event> for ActionEvent {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for ActionEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_50: usize = 0x80 - 0x50;

#[derive(Debug)]
#[repr(C)]
pub struct SetupControlledByStoryEvent {
    base: AIEvent,
    unk50: [u8; PADDING_50], // 50
}

unsafe impl ScriptClass for SetupControlledByStoryEvent {
    type Kind = Native;
    const NAME: &'static str = "gameSetupControlledByStoryEvent";
}

impl AsRef<AIEvent> for SetupControlledByStoryEvent {
    fn as_ref(&self) -> &AIEvent {
        &self.base
    }
}

impl AsRef<Event> for SetupControlledByStoryEvent {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for SetupControlledByStoryEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SignalEvent {
    base: TaggedAIEvent,
}

unsafe impl ScriptClass for SignalEvent {
    type Kind = Native;
    const NAME: &'static str = "AISignalEvent";
}

impl AsRef<TaggedAIEvent> for SignalEvent {
    fn as_ref(&self) -> &TaggedAIEvent {
        &self.base
    }
}

impl AsRef<AIEvent> for SignalEvent {
    fn as_ref(&self) -> &AIEvent {
        self.base.as_ref()
    }
}

impl AsRef<Event> for SignalEvent {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for SignalEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TaggedAIEvent {
    base: AIEvent,
    pub tags: RedArray<CName>, // 50
}

unsafe impl ScriptClass for TaggedAIEvent {
    type Kind = Native;
    const NAME: &'static str = "AITaggedAIEvent";
}

impl AsRef<AIEvent> for TaggedAIEvent {
    fn as_ref(&self) -> &AIEvent {
        &self.base
    }
}

impl AsRef<Event> for TaggedAIEvent {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for TaggedAIEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
