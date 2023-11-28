use red4ext_rs::types::CName;

use crate::FromMemory;

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
#[derive(Debug, Clone)]
#[repr(C)]
pub struct AudioEvent {
    pub(crate) event_name: CName,
    pub(crate) emitter_name: CName,
    pub(crate) name_data: CName,
    pub(crate) float_data: f32,
    pub(crate) event_type: AudioEventActionType,
    pub(crate) event_flags: AudioAudioEventFlags,
}

unsafe impl FromMemory for AudioEvent {
    fn from_memory(address: usize) -> Self {
        let event_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let emitter_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x48) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let name_data: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x50) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let float_data: f32 = unsafe {
            *core::slice::from_raw_parts::<f32>((address + 0x58) as *const f32, 1).get_unchecked(0)
        };
        let event_type: AudioEventActionType = unsafe {
            *core::slice::from_raw_parts::<AudioEventActionType>(
                (address + 0x5C) as *const AudioEventActionType,
                1,
            )
            .get_unchecked(0)
        };
        let event_flags: AudioAudioEventFlags = unsafe {
            *core::slice::from_raw_parts::<AudioAudioEventFlags>(
                (address + 0x60) as *const AudioAudioEventFlags,
                1,
            )
            .get_unchecked(0)
        };
        Self {
            event_name,
            emitter_name,
            name_data,
            float_data,
            event_type,
            event_flags,
        }
    }
}
