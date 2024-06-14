use crate::natives::update_volume;

use super::super::address::ON_AUDIOSYSTEM_GLOBAL_PARAMETER;
use audioware_core::audioware_dbg;
use audioware_engine::{GlobalParameter, GlobalParameters, VolumeModulator};
use audioware_macros::NativeFunc;
use red4ext_rs::types::CName;

fn audioware_exists((parameter_name, parameter_value): &(CName, f32)) -> bool {
    // SAFETY: logging to CET tends to crash the game
    audioware_dbg!(
        "AudioSystem::GlobalParameter({}, {})",
        parameter_name,
        parameter_value
    );

    GlobalParameters::contains(parameter_name).unwrap_or(false)
}

fn audioware_global_parameter((parameter_name, parameter_value): (CName, f32)) {
    if parameter_name == VolumeModulator::name() {
        update_volume(parameter_value);
    }
}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_GLOBAL_PARAMETER,
    inputs = "(CName, f32)",
    allow = "audioware_exists",
    detour = "audioware_global_parameter"
)]
pub struct HookAudioSystemGlobalParameter;
