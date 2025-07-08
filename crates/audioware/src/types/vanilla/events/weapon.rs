use std::fmt;

use red4ext_rs::{NativeRepr, ScriptClass, class_kind::Native, types::IScriptable};

use super::Event;

#[repr(C)]
#[derive(Debug)]
pub struct PreFireEvent {
    base: Event,
}

unsafe impl ScriptClass for PreFireEvent {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsPreFireEvent";
}

impl AsRef<Event> for PreFireEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for PreFireEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct StopWeaponFire {
    base: Event,
}

unsafe impl ScriptClass for StopWeaponFire {
    type Kind = Native;
    const NAME: &'static str = "gameaudioeventsStopWeaponFire";
}

impl AsRef<Event> for StopWeaponFire {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for StopWeaponFire {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct StopFiringEvent {
    base: Event,
}

unsafe impl ScriptClass for StopFiringEvent {
    type Kind = Native;
    const NAME: &'static str = "gameweaponeventsStopFiringEvent";
}

impl AsRef<Event> for StopFiringEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for StopFiringEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SetActiveWeaponEvent {
    base: Event,
}

unsafe impl ScriptClass for SetActiveWeaponEvent {
    type Kind = Native;
    const NAME: &'static str = "gameweaponeventsSetActiveWeaponEvent";
}

impl AsRef<Event> for SetActiveWeaponEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for SetActiveWeaponEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_44: usize = 0x48 - 0x44;

#[repr(C)]
#[derive(Debug)]
pub struct ChangeTriggerModeEvent {
    base: Event,
    pub trigger_mode: TriggerMode, // 40
    unk44: [u8; PADDING_44],       // 44
}

unsafe impl ScriptClass for ChangeTriggerModeEvent {
    type Kind = Native;
    const NAME: &'static str = "gameweaponeventsChangeTriggerModeEvent";
}

impl AsRef<Event> for ChangeTriggerModeEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for ChangeTriggerModeEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ShootEvent {
    base: Event,
}

unsafe impl ScriptClass for ShootEvent {
    type Kind = Native;
    const NAME: &'static str = "gameweaponeventsShootEvent";
}

impl AsRef<Event> for ShootEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for ShootEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TriggerMode {
    Burst = 0,
    Charge = 1,
    FullAuto = 2,
    Lock = 3,
    SemiAuto = 4,
    Windup = 5,
    Count = 6,
    Invalid = 7,
}

unsafe impl NativeRepr for TriggerMode {
    const NAME: &'static str = "TriggerMode";
}

impl fmt::Display for TriggerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Burst => write!(f, "Burst"),
            Self::Charge => write!(f, "Charge"),
            Self::FullAuto => write!(f, "FullAuto"),
            Self::Lock => write!(f, "Lock"),
            Self::SemiAuto => write!(f, "SemiAuto"),
            Self::Windup => write!(f, "Windup"),
            Self::Count => write!(f, "Count"),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
}
