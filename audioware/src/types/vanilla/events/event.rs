use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable},
    ScriptClass,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Event(IScriptable);

unsafe impl ScriptClass for Event {
    type Kind = Native;
    const NAME: &'static str = "redEvent";
}

impl AsRef<IScriptable> for Event {
    fn as_ref(&self) -> &IScriptable {
        &self.0
    }
}

const PADDING_5D: usize = 0x60 - 0x5D;

#[derive(Debug)]
#[repr(C)]
pub struct PlaySound {
    base: Event,
    pub sound_name: CName,   // 40
    pub emitter_name: CName, // 48
    pub audio_tag: CName,    // 50
    pub seek_time: f32,      // 58
    pub play_unique: bool,   // 5C
    unk5d: [u8; PADDING_5D], // 5D
}

unsafe impl ScriptClass for PlaySound {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsPlaySound";
}

impl AsRef<Event> for PlaySound {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for PlaySound {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct StopSound {
    base: Event,
    pub sound_name: CName, // 40
}

unsafe impl ScriptClass for StopSound {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsStopSound";
}

impl AsRef<Event> for StopSound {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for StopSound {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SoundSwitch {
    base: Event,
    pub switch_name: CName,  // 40
    pub switch_value: CName, // 48
}

unsafe impl ScriptClass for SoundSwitch {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsSoundSwitch";
}

impl AsRef<Event> for SoundSwitch {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for SoundSwitch {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct StopTaggedSounds {
    base: Event,
    pub audio_tag: CName, // 40
}

unsafe impl ScriptClass for StopTaggedSounds {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsStopTaggedSounds";
}

impl AsRef<Event> for StopTaggedSounds {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for StopTaggedSounds {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_4C: usize = 0x50 - 0x4C;

#[derive(Debug)]
#[repr(C)]
pub struct SoundParameter {
    base: Event,
    pub parameter_name: CName, // 40
    pub parameter_value: f32,  // 48
    unk4c: [u8; PADDING_4C],   // 4C
}

unsafe impl ScriptClass for SoundParameter {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsSoundParameter";
}

impl AsRef<Event> for SoundParameter {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for SoundParameter {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MusicEvent {
    base: Event,
    event_name: CName, // 40
}

unsafe impl ScriptClass for MusicEvent {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsMusicEvent";
}

impl AsRef<Event> for MusicEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for MusicEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
