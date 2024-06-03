use audioware_macros::FromMemory;
use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    types::CName,
};
use serde::Deserialize;

use super::{cruid::Cruid, event::Event, iscriptable::ISCRIPTABLE_SIZE};

/// see [RED4ext::audio::EventActionType](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/audio/EventActionType.hpp).
#[derive(Debug, Clone, Copy, strum_macros::Display, strum_macros::FromRepr, PartialEq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum AudioEventActionType {
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

/// see [RED4ext::audio::AudioEventFlags](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/audio/AudioEventFlags.hpp).
#[derive(Debug, Clone, Copy, strum_macros::Display, strum_macros::FromRepr)]
#[repr(u32)]
#[allow(dead_code)]
pub enum AudioAudioEventFlags {
    NoEventFlags = 0,
    SloMoOnly = 1,
    Music = 2,
    Unique = 4,
    Metadata = 8,
}

/// see [RED4ext::ent::AudioEvent](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/ent/AudioEvent.hpp).
///
/// see [AudioEvent on Cyberdoc](https://jac3km4.github.io/cyberdoc/#21100).
#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
pub struct AudioEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub event_name: CName,
    pub emitter_name: CName,
    pub name_data: CName,
    pub float_data: f32,
    pub event_type: AudioEventActionType,
    pub event_flags: AudioAudioEventFlags,
    unk64: i32,
}

impl ClassType for AudioEvent {
    type BaseClass = Event;
    const NAME: &'static str = "AudioEvent";
    const NATIVE_NAME: &'static str = "entAudioEvent";
}

/// see [RED4ext::scn::DialogLineType](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/scn/DialogLineType.hpp).
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(u32)]
#[serde(rename_all = "kebab-case")]
pub enum ScnDialogLineType {
    None = 0,
    #[default]
    Regular = 1,
    Holocall = 2,
    SceneComment = 3,
    OverHead = 4,
    Radio = 5,
    GlobalTV = 6,
    Invisible = 7,
    OverHeadAlwaysVisible = 9,
    OwnerlessRegular = 10,
    AlwaysCinematicNoSpeaker = 11,
    GlobalTVAlwaysVisible = 12,
    Narrator = 13,
}

impl ScnDialogLineType {
    pub fn dialog() -> Self {
        Self::Regular
    }
}

unsafe impl NativeRepr for ScnDialogLineType {
    const NAME: &'static str = "scnDialogLineType";
}

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
pub struct MusicEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub event_name: CName,
}

impl ClassType for MusicEvent {
    type BaseClass = Event;
    const NAME: &'static str = "MusicEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsMusicEvent";
}

const PADDING_VOICE_EVENT: usize = 0x58 - 0x51;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
pub struct VoiceEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub event_name: CName,
    pub grunt_type: VoGruntType,
    pub grunt_interrupt_mode: VoGruntInterruptMode,
    pub is_v: bool,
    unk51: [u8; PADDING_VOICE_EVENT],
}

impl ClassType for VoiceEvent {
    type BaseClass = Event;
    const NAME: &'static str = "VoicePlayEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsVoiceEvent";
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(u32)]
#[serde(rename_all = "kebab-case")]
pub enum VoGruntType {
    PainLong = 0,
    AgroShort = 1,
    AgroLong = 2,
    LongFall = 3,
    Death = 4,
    SilentDeath = 5,
    Grapple = 6,
    GrappleMovement = 7,
    EnvironmentalKnockdown = 8,
    Bump = 9,
    Curious = 10,
    Fear = 11,
    Jump = 12,
    EffortLong = 13,
    DeathShort = 14,
    Greet = 15,
    LaughHard = 16,
    LaughSoft = 17,
    Phone = 18,
    BraindanceExcited = 19,
    BraindanceFearful = 20,
    BraindanceNeutral = 21,
    BraindanceSexual = 22,
    PainShort = 23,
    Effort = 25,
    #[default]
    None = 4294967295,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(u32)]
#[serde(rename_all = "kebab-case")]
pub enum VoGruntInterruptMode {
    #[default]
    DontInterrupt = 0,
    PlayOnlyOnInterrupt = 1,
    CanInterrupt = 2,
}

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
pub struct DialogLineEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub dialog_line: DialogLineEventData,
}

impl ClassType for DialogLineEvent {
    type BaseClass = Event;
    const NAME: &'static str = "DialogLineEvent";
    const NATIVE_NAME: &'static str = "audioDialogLineEventData";
}

const PADDING_DIALOG_LINE_FIRST: usize = 0x10 - 0xA;
const PADDING_DIALOG_LINE_SECOND: usize = 0x18 - 0x13;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct DialogLineEventData {
    pub string_id: Cruid,
    pub context: VoiceOverContext,
    pub expression: VoiceOverExpression,
    unk0A: [u8; PADDING_DIALOG_LINE_FIRST],
    pub is_player: bool,
    pub is_rewind: bool,
    pub is_holocall: bool,
    unk13: [u8; PADDING_DIALOG_LINE_SECOND],
    pub custom_vo_event: CName,
    pub seek_time: f32,
    pub playback_speed_parameter: f32,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum VoiceOverContext {
    VoContextQuest = 0,
    VoContextCommunity = 1,
    VoContextCombat = 2,
    VoContextMinorActivity = 3,
    #[default]
    DefaultVoContext = 5,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum VoiceOverExpression {
    #[default]
    VoExpressionSpoken = 0,
    VoExpressionPhone = 1,
    VoExpressionInnerDialog = 2,
    VoExpressionLoudspeakerRoom = 3,
    VoExpressionLoudspeakerStreet = 4,
    VoExpressionLoudspeakerCity = 5,
    VoExpressionRadio = 6,
    VoExpressionGlobalTV = 7,
    VoExperessionCbRadio = 8,
    VoExpressionCyberspace = 9,
    VoExpressionPossessed = 10,
    VoExpressionHelmet = 11,
}

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
pub struct DialogLineEndEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
}

impl ClassType for DialogLineEndEvent {
    type BaseClass = Event;
    const NAME: &'static str = "DialogLineEnd";
    const NATIVE_NAME: &'static str = "gameaudioeventsDialogLineEnd";
}

const PADDING_STOP_DIALOG: usize = 0x50 - 0x4C;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct StopDialogLine {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub string_id: Cruid,
    pub fade_out: f32,
    unk4C: [u8; PADDING_STOP_DIALOG],
}

impl ClassType for StopDialogLine {
    type BaseClass = Event;
    const NAME: &'static str = "StopDialogLine";
    const NATIVE_NAME: &'static str = "gameaudioeventsStopDialogLine";
}

const PADDING_VOICE_PLAYED: usize = 0x50 - 0x4D;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct VoicePlayedEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub event_name: CName,
    pub grunt_type: VoGruntType,
    pub is_v: bool,
    unk4D: [u8; PADDING_VOICE_PLAYED],
}

impl ClassType for VoicePlayedEvent {
    type BaseClass = Event;
    const NAME: &'static str = "VoicePlayedEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsVoicePlayedEvent";
}

const PADDING_EMITTER: usize = 0x58 - 0x48;

/// looks unused in the binary
#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct EmitterEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub emitter_name: CName,
    unk48: [u8; PADDING_EMITTER],
}

impl ClassType for EmitterEvent {
    type BaseClass = Event;
    const NAME: &'static str = "EmitterEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsEmitterEvent";
}

/// looks unused in the binary
#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct PlaySoundOnEmitter {
    emitter: [u8; std::mem::size_of::<EmitterEvent>()],
    pub event_name: CName,
}

impl ClassType for PlaySoundOnEmitter {
    type BaseClass = EmitterEvent;
    const NAME: &'static str = "PlaySoundOnEmitter";
    const NATIVE_NAME: &'static str = "gameaudioeventsPlaySoundOnEmitter";
}

/// looks unused in the binary
#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct StopSoundOnEmitter {
    emitter: [u8; std::mem::size_of::<EmitterEvent>()],
    pub sound_name: CName,
}

impl ClassType for StopSoundOnEmitter {
    type BaseClass = EmitterEvent;
    const NAME: &'static str = "StopSoundOnEmitter";
    const NATIVE_NAME: &'static str = "gameaudioeventsStopSoundOnEmitter";
}

const PADDING_SET_PARAMETER: usize = 0x68 - 0x64;

/// looks unused in the binary
#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct SetParameterOnEmitter {
    emitter: [u8; std::mem::size_of::<EmitterEvent>()],
    pub param_name: CName,
    pub param_value: f32,
    unk64: [u8; PADDING_SET_PARAMETER],
}

impl ClassType for SetParameterOnEmitter {
    type BaseClass = EmitterEvent;
    const NAME: &'static str = "SetParameterOnEmitter";
    const NATIVE_NAME: &'static str = "gameaudioeventsSetParameterOnEmitter";
}

const PADDING_PLAYSOUND: usize = 0x60 - 0x5D;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct PlaySound {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub sound_name: CName,
    pub emitter_name: CName,
    pub audio_tag: CName,
    pub seek_time: f32,
    pub play_unique: bool,
    unk5D: [u8; PADDING_PLAYSOUND],
}

impl std::fmt::Display for PlaySound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): {}, {}, {}, {}, {}",
            Self::NAME,
            Self::NATIVE_NAME,
            self.sound_name,
            self.emitter_name,
            self.audio_tag,
            self.seek_time,
            self.play_unique
        )
    }
}

impl ClassType for PlaySound {
    type BaseClass = Event;
    const NAME: &'static str = "SoundPlayEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsPlaySound";
}

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct StopSound {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub sound_name: CName,
}

impl ClassType for StopSound {
    type BaseClass = Event;
    const NAME: &'static str = "SoundStopEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsStopSound";
}

impl std::fmt::Display for StopSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): {}",
            Self::NAME,
            Self::NATIVE_NAME,
            self.sound_name
        )
    }
}

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct StopTaggedSounds {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    audio_tag: CName,
}

impl ClassType for StopTaggedSounds {
    type BaseClass = Event;
    const NAME: &'static str = "StopTaggedSounds";
    const NATIVE_NAME: &'static str = "gameaudioeventsStopTaggedSounds";
}

impl std::fmt::Display for StopTaggedSounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): {}",
            Self::NAME,
            Self::NATIVE_NAME,
            self.audio_tag
        )
    }
}

const PADDING_SOUNDPARAMETER: usize = 0x50 - 0x4C;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct SoundParameter {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    parameter_name: CName,
    parameter_value: f32,
    unk4C: [u8; PADDING_SOUNDPARAMETER],
}

impl ClassType for SoundParameter {
    type BaseClass = Event;
    const NAME: &'static str = "SoundParameterEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsSoundParameter";
}

impl std::fmt::Display for SoundParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): {}, {}",
            Self::NAME,
            Self::NATIVE_NAME,
            self.parameter_name,
            self.parameter_value
        )
    }
}

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(std::mem::size_of::<super::AudioEvent>(), 0x68);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::MusicEvent>(), 0x48);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::VoiceEvent>(), 0x58);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::DialogLineEvent>(), 0x68);
        static_assertions::const_assert_eq!(
            std::mem::size_of::<super::DialogLineEventData>(),
            0x28
        );
        static_assertions::const_assert_eq!(std::mem::size_of::<super::StopDialogLine>(), 0x50);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::VoicePlayedEvent>(), 0x50);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::EmitterEvent>(), 0x58);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::PlaySoundOnEmitter>(), 0x60);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::StopSoundOnEmitter>(), 0x60);
        static_assertions::const_assert_eq!(
            std::mem::size_of::<super::SetParameterOnEmitter>(),
            0x68
        );
        static_assertions::const_assert_eq!(std::mem::size_of::<super::PlaySound>(), 0x60);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::StopSound>(), 0x48);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::StopTaggedSounds>(), 0x48);
        static_assertions::const_assert_eq!(std::mem::size_of::<super::SoundParameter>(), 0x50);
    }
}
