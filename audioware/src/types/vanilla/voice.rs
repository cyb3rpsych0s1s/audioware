use std::fmt;

use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable},
    NativeRepr, ScriptClass,
};

use super::Event;

const PADDING_51: usize = 0x58 - 0x51;

#[derive(Debug)]
#[repr(C)]
pub struct VoiceEvent {
    base: Event,
    pub event_name: CName,                          // 40
    pub grunt_type: VoGruntType,                    // 48
    pub grunt_interrupt_mode: VoGruntInterruptMode, // 4C
    pub is_v: bool,                                 // 50
    unk51: [u8; PADDING_51],                        // 51
}

unsafe impl ScriptClass for VoiceEvent {
    const NAME: &'static str = "gameaudioeventsVoiceEvent";
    type Kind = Native;
}

impl AsRef<Event> for VoiceEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for VoiceEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_4D: usize = 0x50 - 0x4D;

#[derive(Debug)]
#[repr(C)]
pub struct VoicePlayedEvent {
    base: Event,
    pub event_name: CName,       // 40
    pub grunt_type: VoGruntType, // 48
    pub is_v: bool,              // 4C
    unk4d: [u8; PADDING_4D],     // 4D
}

unsafe impl ScriptClass for VoicePlayedEvent {
    const NAME: &'static str = "gameaudioeventsVoicePlayedEvent";
    type Kind = Native;
}

impl AsRef<Event> for VoicePlayedEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for VoicePlayedEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
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

unsafe impl NativeRepr for VoGruntType {
    const NAME: &'static str = "VoGruntType";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VoGruntInterruptMode {
    DontInterrupt = 0,
    PlayOnlyOnInterrupt = 1,
    CanInterrupt = 2,
}

impl fmt::Display for VoGruntInterruptMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::DontInterrupt => "DontInterrupt",
                Self::PlayOnlyOnInterrupt => "PlayOnlyOnInterrupt",
                Self::CanInterrupt => "CanInterrupt",
            }
        )
    }
}

unsafe impl NativeRepr for VoGruntInterruptMode {
    const NAME: &'static str = "VoGruntInterruptMode";
}

impl fmt::Display for VoGruntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::PainLong => "PainLong",
                Self::AgroShort => "AgroShort",
                Self::AgroLong => "AgroLong",
                Self::LongFall => "LongFall",
                Self::Death => "Death",
                Self::SilentDeath => "SilentDeath",
                Self::Grapple => "Grapple",
                Self::GrappleMovement => "GrappleMovement",
                Self::EnvironmentalKnockdown => "EnvironmentalKnockdown",
                Self::Bump => "Bump",
                Self::Curious => "Curious",
                Self::Fear => "Fear",
                Self::Jump => "Jump",
                Self::EffortLong => "EffortLong",
                Self::DeathShort => "DeathShort",
                Self::Greet => "Greet",
                Self::LaughHard => "LaughHard",
                Self::LaughSoft => "LaughSoft",
                Self::Phone => "Phone",
                Self::BraindanceExcited => "BraindanceExcited",
                Self::BraindanceFearful => "BraindanceFearful",
                Self::BraindanceNeutral => "BraindanceNeutral",
                Self::BraindanceSexual => "BraindanceSexual",
                Self::PainShort => "PainShort",
                Self::Effort => "Effort",
                Self::None => "None",
            }
        )
    }
}
