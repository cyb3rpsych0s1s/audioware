use audioware_macros::FromMemory;
use red4ext_rs::types::CName;

use super::iscriptable::ISCRIPTABLE_SIZE;

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
