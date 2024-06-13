use super::super::address::ON_AUDIOSYSTEM_STATE;
use audioware_macros::NativeFunc;

fn audioware_exists(params: &(String, String)) -> bool {
    audioware_core::dbg(format!(
        "AudioSystem::State({:?}, {:?})",
        params.0, params.1
    ));

    false
}

fn noop(_: (String, String)) {}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_STATE,
    inputs = "(String, String)",
    allow = "audioware_exists",
    detour = "noop"
)]
pub struct HookAudioSystemState;
