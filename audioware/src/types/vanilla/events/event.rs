use std::fmt;

use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable, ISerializable, RedArray, Ref},
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

const PADDING_08: usize = 0x8;
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
        write!(
            f,
            "{}",
            match self {
                Self::Log3 => "Log3",
                Self::Sine => "Sine",
                Self::InversedSCurve => "InversedSCurve",
                Self::Linear => "Linear",
                Self::SCurve => "SCurve",
                Self::Exp1 => "Exp1",
                Self::ReciprocalOfSineCurve => "ReciprocalOfSineCurve",
                Self::Exp3 => "Exp3",
            }
        )
    }
}

const PADDING_64: usize = 0x68 - 0x64;

#[derive(Debug)]
#[repr(C)]
pub struct AudioEvent {
    base: Event,
    pub event_name: CName,            // 40
    pub emitter_name: CName,          // 48
    pub name_data: CName,             // 50
    pub float_data: f32,              // 58
    pub event_type: EventActionType,  // 5C
    pub event_flags: AudioEventFlags, // 60
    unk64: [u8; PADDING_64],          // 64
}

unsafe impl ScriptClass for AudioEvent {
    type Kind = Native;
    const NAME: &'static str = "entAudioEvent";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EventActionType {
    Play = 0,
    PlayAnimation = 1,
    SetParameter = 2,
    StopSound = 3,
    SetSwitch = 4,
    StopTagged = 5,
    PlayExternal = 6,
    Tag = 7,
    Untag = 8,
    SetAppearanceName = 9,
    SetEntityName = 10,
    AddContainerStreamingPrefetch = 11,
    RemoveContainerStreamingPrefetch = 12,
}

unsafe impl NativeRepr for EventActionType {
    const NAME: &'static str = "EventActionType";
}

impl fmt::Display for EventActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Play => "Play",
                Self::PlayAnimation => "PlayAnimation",
                Self::SetParameter => "SetParameter",
                Self::StopSound => "StopSound",
                Self::SetSwitch => "SetSwitch",
                Self::StopTagged => "StopTagged",
                Self::PlayExternal => "PlayExternal",
                Self::Tag => "Tag",
                Self::Untag => "Untag",
                Self::SetAppearanceName => "SetAppearanceName",
                Self::SetEntityName => "SetEntityName",
                Self::AddContainerStreamingPrefetch => "AddContainerStreamingPrefetch",
                Self::RemoveContainerStreamingPrefetch => "RemoveContainerStreamingPrefetch",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AudioEventFlags {
    NoEventFlags = 0,
    SloMoOnly = 1,
    Music = 2,
    Unique = 4,
    Metadata = 8,
}

unsafe impl NativeRepr for AudioEventFlags {
    const NAME: &'static str = "AudioEventFlags";
}

impl fmt::Display for AudioEventFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoEventFlags => "NoEventFlags",
                Self::SloMoOnly => "SloMoOnly",
                Self::Music => "Music",
                Self::Unique => "Unique",
                Self::Metadata => "Metadata",
            }
        )
    }
}

const PADDING_31: usize = 0x38 - 0x31;

#[repr(C)]
pub struct AudioEventArray {
    base: ISerializable,
    pub is_sorted_by_red_hash: bool,
    unk31: [u8; PADDING_31],
    pub events: RedArray<AudioEventMetadataArrayElement>,
    pub switch_group: RedArray<AudioEventMetadataArrayElement>,
    pub switch: RedArray<AudioEventMetadataArrayElement>,
    pub state_group: RedArray<AudioEventMetadataArrayElement>,
    pub state: RedArray<AudioEventMetadataArrayElement>,
    pub game_parameter: RedArray<AudioEventMetadataArrayElement>,
    pub bus: RedArray<AudioEventMetadataArrayElement>,
}

impl fmt::Debug for AudioEventArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioEventArray")
            .field("base", &self.base)
            .field("is_sorted_by_red_hash", &self.is_sorted_by_red_hash)
            .field("unk31", &self.unk31)
            .finish_non_exhaustive()
    }
}

unsafe impl ScriptClass for AudioEventArray {
    type Kind = Native;
    const NAME: &'static str = "audioAudioEventArray";
}

const PADDING_41: usize = 0x48 - 0x41;

#[derive(Debug)]
#[repr(C)]
pub struct AudioEventMetadata {
    base: ISerializable,
    pub wwise_id: u32,
    pub max_attenuation: f32,
    pub min_duration: f32,
    pub max_duration: f32,
    pub is_looping: bool,
    unk41: [u8; PADDING_41],
    pub stop_action_events: RedArray<CName>,
    pub tags: RedArray<CName>,
}

unsafe impl ScriptClass for AudioEventMetadata {
    type Kind = Native;
    const NAME: &'static str = "audioAudioEventMetadata";
}

const PADDING_41_BIS: usize = 0x44 - 0x41;
const PADDING_4C_BIS: usize = 0x50 - 0x4C;

#[derive(Debug)]
#[repr(C)]
pub struct AudioEventMetadataArrayElement {
    base: ISerializable,
    pub red_id: CName,
    pub wwise_id: u32,
    pub max_attenuation: f32,
    pub is_looping: bool,
    unk41: [u8; PADDING_41_BIS],
    pub min_duration: f32,
    pub max_duration: f32,
    unk4c: [u8; PADDING_4C_BIS],
    pub stop_action_events: RedArray<CName>,
    pub tags: RedArray<CName>,
}

unsafe impl NativeRepr for AudioEventMetadataArrayElement {
    const NAME: &'static str = "audioAudioEventMetadataArrayElement";
}
