use red4ext_rs::types::CName;

use audioware_types::FromMemory;

/// see [RED4ext::game::audio::events::VoiceEvent](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/game/audio/events/VoiceEvent.hpp).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VoiceEvent {
    pub(crate) event_name: CName,
    pub(crate) grunt_type: VoGruntType,
    pub(crate) grunt_interrupt_mode: VoGruntInterruptMode,
    pub(crate) is_v: bool,
    // pub(crate) unk51: [u8;0x58 - 0x51],
}

/// see [RED4ext::audio::VoGruntType](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/audio/VoGruntType.hpp).
#[derive(Debug, Clone, Copy, strum_macros::Display, strum_macros::FromRepr)]
#[repr(u32)]
#[allow(dead_code)]
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
    None = 4294967295,
}

/// see [RED4ext::audio::VoGruntInterruptMode](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/audio/VoGruntInterruptMode.hpp).
#[derive(Debug, Clone, Copy, strum_macros::Display, strum_macros::FromRepr)]
#[repr(u32)]
#[allow(dead_code)]
pub enum VoGruntInterruptMode {
    DontInterrupt = 0,
    PlayOnlyOnInterrupt = 1,
    CanInterrupt = 2,
}

unsafe impl FromMemory for VoiceEvent {
    fn from_memory(address: usize) -> Self {
        let event_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let grunt_type: VoGruntType = unsafe {
            *core::slice::from_raw_parts::<VoGruntType>((address + 0x48) as *const VoGruntType, 1)
                .get_unchecked(0)
        };
        let grunt_interrupt_mode: VoGruntInterruptMode = unsafe {
            *core::slice::from_raw_parts::<VoGruntInterruptMode>(
                (address + 0x4C) as *const VoGruntInterruptMode,
                1,
            )
            .get_unchecked(0)
        };
        let is_v: bool = unsafe {
            *core::slice::from_raw_parts::<bool>((address + 0x50) as *const bool, 1)
                .get_unchecked(0)
        };
        Self {
            event_name,
            grunt_type,
            grunt_interrupt_mode,
            is_v,
        }
    }
}
