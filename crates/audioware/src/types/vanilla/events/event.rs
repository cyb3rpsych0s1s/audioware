use std::{fmt, ops::Deref};

use bitflags::bitflags;
use red4ext_rs::{
    NativeRepr, ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, RedArray},
};

use crate::Vector3;

#[derive(Debug, Clone)]
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

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct AudParam {
    pub name: CName, // 0
    pub value: f32,  // 08
}

unsafe impl NativeRepr for AudParam {
    const NAME: &'static str = "Audioware.AudParam";
}

impl fmt::Display for AudParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct AudSwitch {
    pub name: CName,  // 0
    pub value: CName, // 08
}

unsafe impl NativeRepr for AudSwitch {
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ESoundCurveType {
    Log3 = 0,
    Sine = 1,
    InversedSCurve = 3,
    #[default]
    Linear = 4,
    SCurve = 5,
    Exp1 = 6,
    ReciprocalOfSineCurve = 7,
    Exp3 = 8,
}

unsafe impl NativeRepr for ESoundCurveType {
    const NAME: &'static str = "audioESoundCurveType";
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

impl std::fmt::Display for AudioEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "event_name {}, name_data {}, emitter_name {}, event_type {}",
            self.event_name.as_str(),
            self.name_data.as_str(),
            self.emitter_name.as_str(),
            self.event_type,
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum EventActionType {
    #[default]
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

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventActionTypes: u32 {
        const NONE = 0;
        const PLAY = 1 << 1;
        const PLAY_ANIMATION = 1 << 2;
        const SET_PARAMETER = 1 << 3;
        const STOP_SOUND = 1 << 4;
        const SET_SWITCH = 1 << 5;
        const STOP_TAGGED = 1 << 6;
        const PLAY_EXTERNAL = 1 << 7;
        const TAG = 1 << 8;
        const UNTAG = 1 << 9;
        const SET_APPEARANCE_NAME = 1 << 10;
        const SET_ENTITY_NAME = 1 << 11;
        const ADD_CONTAINER_STREAMING_PREFETCH = 1 << 12;
        const REMOVE_CONTAINER_STREAMING_PREFETCH = 1 << 13;
    }
}

impl From<EventActionType> for EventActionTypes {
    fn from(value: EventActionType) -> Self {
        Self::from_bits_truncate(value as u32)
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AudioEventFlags: u32 {
        const NO_EVENT_FLAGS = 0;
        const SLO_MO_ONLY = 1 << 0;
        const MUSIC = 1 << 1;
        const UNIQUE = 1 << 2;
        const METADATA = 1 << 3;
    }
}

unsafe impl NativeRepr for AudioEventFlags {
    const NAME: &'static str = "audioAudioEventFlags";
}

impl fmt::Display for AudioEventFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "none")
        } else {
            let mut out = Vec::with_capacity(4);
            if self.contains(Self::SLO_MO_ONLY) {
                out.push("SloMoOnly");
            }
            if self.contains(Self::MUSIC) {
                out.push("Music");
            }
            if self.contains(Self::UNIQUE) {
                out.push("Unique");
            }
            if self.contains(Self::METADATA) {
                out.push("Metadata");
            }
            if out.is_empty() {
                return write!(f, "none");
            }
            write!(f, "({})", out.join("|"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct AudioEventId(u32);

impl std::fmt::Display for AudioEventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "aeid:{}", self.0)
    }
}

impl AudioEventId {
    pub fn invalid() -> Self {
        Self(0)
    }
}

const INVALID_WWISE_ID: u32 = 2166136261;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct WwiseId(u32);

impl WwiseId {
    pub fn is_null(&self) -> bool {
        self.0 == INVALID_WWISE_ID
    }
    pub fn to_i64(&self) -> i64 {
        self.0 as i64
    }
}

impl Default for WwiseId {
    fn default() -> Self {
        Self(INVALID_WWISE_ID)
    }
}

impl std::fmt::Display for WwiseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wwise:{}", self.0)
    }
}

unsafe impl NativeRepr for WwiseId {
    const NAME: &'static str = u32::NAME;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ApplyEmitterStrategy {
    emitter_name: CName,
    position_name: CName,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union AudioStrategyUnion {
    by_emitter_and_position: ApplyEmitterStrategy,
    by_emitter_id: u32,
    by_position: Vector3,
    by_tags: [CName; 2],
    by_event_id: AudioEventId,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union AudioParamUnion {
    pub name: CName,
    pub float: f32,
}

#[derive(Clone)]
#[repr(C)]
pub struct AudioInternalEvent {
    strategy: AudioStrategyUnion,          // 0
    name: CName,                           // 0x10
    param: AudioParamUnion,                // 0x18
    external_source_path: u64,             // 0x20
    id: AudioEventId,                      // 0x28
    pub action: EventActionType,           // 0x2C
    pub flags: AudioEventFlags,            // 0x30
    strategy_type: EventApplyStrategyType, // 0x34
}

impl AudioInternalEvent {
    pub fn event_name(&self) -> CName {
        self.name
    }
    pub fn emitter_name(&self) -> Option<CName> {
        if self.strategy_type == EventApplyStrategyType::ApplyEmitter
            || self.strategy_type == EventApplyStrategyType::ApplyPositionName
        {
            return Some(unsafe { self.strategy.by_emitter_and_position.emitter_name });
        }
        None
    }
    pub fn event_flags(&self) -> AudioEventFlags {
        self.flags
    }
    pub fn event_type(&self) -> EventActionType {
        self.action
    }
    pub fn name_data(&self) -> Option<CName> {
        if self.action == EventActionType::SetSwitch {
            return Some(unsafe { self.param.name });
        }
        None
    }
    pub fn float_data(&self) -> Option<f32> {
        match self.action {
            EventActionType::PlayExternal if self.external_source_path != 0 => {
                Some(unsafe { self.param.float })
            }
            EventActionType::SetParameter | EventActionType::Play | EventActionType::StopSound => {
                Some(unsafe { self.param.float })
            }
            _ => None,
        }
    }
}

impl std::fmt::Display for AudioInternalEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("name: {}, action: {}", self.name, self.action);
        out.push_str(&format!(", flags: {}", self.flags));
        match self.action {
            EventActionType::SetSwitch => {
                out.push_str(&format!(", param.name: {}", unsafe { self.param.name }));
            }
            EventActionType::PlayExternal => {
                out.push_str(&format!(
                    ", param.external_source_path: {}",
                    self.external_source_path
                ));
                out.push_str(&format!(", param.float: {}", unsafe { self.param.float }));
            }
            _ => {
                out.push_str(&format!(", param.float: {}", unsafe { self.param.float }));
            }
        };
        if self.strategy_type == EventApplyStrategyType::ApplyEmitter
            || self.strategy_type == EventApplyStrategyType::ApplyPositionName
        {
            out.push_str(&format!(", extra.emitter_name: {}", unsafe {
                self.strategy.by_emitter_and_position.emitter_name
            }));
        }
        if self.strategy_type == EventApplyStrategyType::ApplyPositionName {
            out.push_str(&format!(", extra.position_name: {}", unsafe {
                self.strategy.by_emitter_and_position.position_name
            }));
        }
        if self.strategy_type == EventApplyStrategyType::ApplyEmitterWithId {
            out.push_str(&format!(
                ", extra.emitter_id: {}",
                unsafe { self.strategy.by_emitter_id } // EntityId::from(unsafe { self.strategy.by_emitter_id } as u64) ?
            ));
        }
        if self.strategy_type == EventApplyStrategyType::ApplyPosition {
            out.push_str(&format!(", extra.position: {}", unsafe {
                self.strategy.by_position
            }));
        }
        if self.strategy_type == EventApplyStrategyType::ApplyEventId
            && unsafe { self.strategy.by_event_id.0 } != 0
        {
            out.push_str(&format!(", extra.event_id: {}", unsafe {
                self.strategy.by_event_id
            }));
        }
        if self.strategy_type == EventApplyStrategyType::ApplyTagged
            && unsafe { self.strategy.by_tags[0] } != CName::new("None")
        {
            if unsafe { self.strategy.by_tags[1] } != CName::new("None") {
                out.push_str(&format!(
                    ", extra.tags: [{}, {}]",
                    unsafe { self.strategy.by_tags[0] },
                    unsafe { self.strategy.by_tags[1] },
                ));
            } else {
                out.push_str(&format!(", extra.tags: [{}]", unsafe {
                    self.strategy.by_tags[0]
                },));
            }
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum EventApplyStrategyType {
    ApplyEntity = 0,
    ApplyEmitter = 1,
    ApplyEmitterWithId = 2,
    ApplyPosition = 3,
    ApplyEventId = 4,
    ApplyTagged = 5,
    ApplyPositionName = 6,
}

impl std::fmt::Display for EventApplyStrategyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ApplyEntity => "ApplyEntity",
                Self::ApplyEmitter => "ApplyEmitter",
                Self::ApplyEmitterWithId => "ApplyEmitterWithId",
                Self::ApplyPosition => "ApplyPosition",
                Self::ApplyEventId => "ApplyEventId",
                Self::ApplyTagged => "ApplyTagged",
                Self::ApplyPositionName => "ApplyPositionName",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundObjectId(i64);

impl std::fmt::Display for SoundObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "soid:{}", self.0)
    }
}
