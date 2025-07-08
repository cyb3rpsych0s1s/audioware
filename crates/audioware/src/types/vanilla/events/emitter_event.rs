use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable},
};

use super::Event;

const PADDING_48: usize = 0x58 - 0x48;

#[repr(C)]
pub struct EmitterEvent {
    base: Event,
    pub emitter_name: CName, // 40
    unk48: [u8; PADDING_48], // 48
}

unsafe impl ScriptClass for EmitterEvent {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsEmitterEvent";
}

impl AsRef<Event> for EmitterEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for EmitterEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct PlaySoundOnEmitter {
    base: EmitterEvent,
    pub event_name: CName,
}

unsafe impl ScriptClass for PlaySoundOnEmitter {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsPlaySoundOnEmitter";
}

impl AsRef<EmitterEvent> for PlaySoundOnEmitter {
    fn as_ref(&self) -> &EmitterEvent {
        &self.base
    }
}

impl AsRef<Event> for PlaySoundOnEmitter {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for PlaySoundOnEmitter {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct StopSoundOnEmitter {
    base: EmitterEvent,
    pub sound_name: CName,
}

unsafe impl ScriptClass for StopSoundOnEmitter {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsStopSoundOnEmitter";
}

impl AsRef<EmitterEvent> for StopSoundOnEmitter {
    fn as_ref(&self) -> &EmitterEvent {
        &self.base
    }
}

impl AsRef<Event> for StopSoundOnEmitter {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for StopSoundOnEmitter {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_64: usize = 0x68 - 0x64;

#[repr(C)]
pub struct SetParameterOnEmitter {
    base: EmitterEvent,
    pub param_name: CName,   // 58
    pub param_value: f32,    // 60
    unk64: [u8; PADDING_64], // 64
}

unsafe impl ScriptClass for SetParameterOnEmitter {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsSetParameterOnEmitter";
}

impl AsRef<EmitterEvent> for SetParameterOnEmitter {
    fn as_ref(&self) -> &EmitterEvent {
        &self.base
    }
}

impl AsRef<Event> for SetParameterOnEmitter {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for SetParameterOnEmitter {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_41: usize = 0x48 - 0x41;

#[repr(C)]
#[derive(Debug)]
pub struct SetListenerOverride {
    base: Event,
    pub enable: bool,        // 40
    unk41: [u8; PADDING_41], // 41
}

unsafe impl ScriptClass for SetListenerOverride {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsSetListenerOverride";
}

impl AsRef<Event> for SetListenerOverride {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for SetListenerOverride {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
