use std::fmt;

use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable, RedArray},
    NativeRepr, ScriptClass,
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
    pub event_name: CName, // 40
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

#[derive(Debug)]
#[repr(C)]
pub struct SoundEvent {
    base: Event,
    pub event_name: CName,               // 40
    pub switches: RedArray<AudSwitch>,   // 48
    pub params: RedArray<AudParameter>,  // 58
    pub dynamic_params: RedArray<CName>, // 68
}

unsafe impl ScriptClass for SoundEvent {
    type Kind = Native;
    const NAME: &'static str = "entSoundEvent";
}

impl AsRef<Event> for SoundEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for SoundEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AudSwitch {
    pub name: CName,  // 0
    pub value: CName, // 08
}

unsafe impl ScriptClass for AudSwitch {
    type Kind = Native;
    const NAME: &'static str = "audioAudSwitch";
}

impl fmt::Display for AudSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

const PADDING_08: usize = 0x8 - 0x0;
const PADDING_24: usize = 0x28 - 0x24;

#[derive(Debug)]
#[repr(C)]
pub struct AudParameter {
    unk00: [u8; PADDING_08],               // 0
    pub name: CName,                       // 08
    pub value: f32,                        // 10
    pub enter_curve_type: ESoundCurveType, // 14
    pub enter_curve_time: f32,             // 18
    pub exit_curve_type: ESoundCurveType,  // 1C
    pub exit_curve_time: f32,              // 20
    unk24: [u8; PADDING_24],               // 24
}

unsafe impl ScriptClass for AudParameter {
    type Kind = Native;
    const NAME: &'static str = "audioAudParameter";
}

impl fmt::Display for AudParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (enter: {} at {}, exit: {} at {})",
            self.name,
            self.value,
            self.enter_curve_type,
            self.enter_curve_time,
            self.exit_curve_type,
            self.exit_curve_time
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ESoundCurveType {
    Log3 = 0,
    Sine = 1,
    InversedSCurve = 3,
    Linear = 4,
    SCurve = 5,
    Exp1 = 6,
    ReciprocalOfSineCurve = 7,
    Exp3 = 8,
}

unsafe impl NativeRepr for ESoundCurveType {
    const NAME: &'static str = "ESoundCurveType";
}

impl fmt::Display for ESoundCurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
