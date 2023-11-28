use red4ext_rs::types::CName;

use crate::FromMemory;

// see [PlaySound](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/game/audio/events/PlaySound.hpp)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct PlaySound {
    pub(crate) sound_name: CName,
    pub(crate) emitter_name: CName,
    pub(crate) audio_tag: CName,
    pub(crate) seek_time: f32,
    pub(crate) play_unique: bool,
}

/// see [SoundEvent](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/ent/SoundEvent.hpp).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SoundEvent {
    pub(crate) event_name: CName,
    // TODO ...
}

unsafe impl FromMemory for PlaySound {
    fn from_memory(address: usize) -> Self {
        let sound_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let emitter_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x48) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let audio_tag: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x50) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let seek_time: f32 = unsafe {
            *core::slice::from_raw_parts::<f32>((address + 0x58) as *const f32, 1).get_unchecked(0)
        };
        let play_unique: bool = unsafe {
            *core::slice::from_raw_parts::<bool>((address + 0x5C) as *const bool, 1)
                .get_unchecked(0)
        };
        Self {
            sound_name,
            emitter_name,
            audio_tag,
            seek_time,
            play_unique,
        }
    }
}

unsafe impl FromMemory for SoundEvent {
    fn from_memory(address: usize) -> Self {
        let event_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        Self { event_name }
    }
}
