use super::super::address::ON_AUDIOSYSTEM_ADD_TRIGGER_EFFECT;
use audioware_macros::NativeFunc;
use red4ext_rs::types::CName;

fn audioware_exists(params: &(CName, CName)) -> bool {
    audioware_core::utils::dbg(format!(
        "AudioSystem::AddTriggerEffect({}, {})",
        params.0, params.1
    ));

    false
}

fn noop(_: (CName, CName)) {}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_ADD_TRIGGER_EFFECT,
    inputs = "(CName, CName)",
    allow = "audioware_exists",
    detour = "noop"
)]
pub struct HookAudioSystemAddTriggerEffect;
