use audioware_macros::FromMemory;
use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    types::CName,
};

use super::{event::Event, iscriptable::ISCRIPTABLE_SIZE};

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

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(std::mem::size_of::<super::AudioEvent>(), 0x68);
    }
}

/// see [RED4ext::scn::DialogLineType](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/scn/DialogLineType.hpp).
#[derive(Debug, Clone, Copy, strum_macros::Display, strum_macros::FromRepr, PartialEq)]
#[repr(u32)]
pub enum ScnDialogLineType {
    None = 0,
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

unsafe impl NativeRepr for ScnDialogLineType {
    const NAME: &'static str = "scnDialogLineType";
}
