use core::fmt;

use red4ext_rs::{
    class_kind::Native,
    types::{CName, Cruid, IScriptable, Ref},
    NativeRepr, ScriptClass,
};

use crate::WorldTransform;

use super::Event;

const PADDING_44: usize = 0x48 - 0x44;

#[derive(Debug)]
#[repr(C)]
pub struct VehicleAudioEvent {
    base: Event,
    pub action: AudioEventAction, // 40
    unk44: [u8; PADDING_44],      // 44
}

unsafe impl ScriptClass for VehicleAudioEvent {
    type Kind = Native;
    const NAME: &'static str = "vehicleAudioEvent";
}

#[allow(clippy::enum_variant_names, reason = "see RED4ext.SDK")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AudioEventAction {
    OnPlayerDriving = 0,
    OnPlayerPassenger = 1,
    OnPlayerEnterCombat = 2,
    OnPlayerExitCombat = 3,
    OnPlayerExitVehicle = 4,
    OnPlayerVehicleSummoned = 5,
}

unsafe impl NativeRepr for AudioEventAction {
    const NAME: &'static str = "AudioEventAction";
}

impl fmt::Display for AudioEventAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OnPlayerDriving => "OnPlayerDriving",
                Self::OnPlayerPassenger => "OnPlayerPassenger",
                Self::OnPlayerEnterCombat => "OnPlayerEnterCombat",
                Self::OnPlayerExitCombat => "OnPlayerExitCombat",
                Self::OnPlayerExitVehicle => "OnPlayerExitVehicle",
                Self::OnPlayerVehicleSummoned => "OnPlayerVehicleSummoned",
            }
        )
    }
}
