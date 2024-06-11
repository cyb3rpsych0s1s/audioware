use crate::{engine::effect::MODULATOR_NAME, natives::update_modulator};

use super::super::address::ON_AUDIOSYSTEM_GLOBAL_PARAMETER;
use audioware_macros::NativeFunc;
use red4ext_rs::types::CName;

fn audioware_exists((parameter_name, parameter_value): &(CName, f32)) -> bool {
    // SAFETY: logging to CET tends to crash the game
    crate::utils::dbg(format!(
        "AudioSystem::GlobalParameter({}, {})",
        parameter_name, parameter_value
    ));

    parameter_name == &CName::new(MODULATOR_NAME)
}

fn audioware_global_parameter((parameter_name, parameter_value): (CName, f32)) {
    if parameter_name == CName::new(MODULATOR_NAME) {
        update_modulator(parameter_value);
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
