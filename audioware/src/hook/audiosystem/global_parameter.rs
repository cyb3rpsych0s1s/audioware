use super::super::address::ON_AUDIOSYSTEM_GLOBAL_PARAMETER;
use audioware_macros::NativeFunc;
use red4ext_rs::types::CName;

fn audioware_exists(params: &(CName, f32)) -> bool {
    #[cfg(debug_assertions)]
    crate::utils::info(format!(
        "AudioSystem::GlobalParameter({}, {})",
        params.0, params.1
    ));

    false
}

fn noop(_: (CName, f32)) {}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_GLOBAL_PARAMETER,
    inputs = "(CName, f32)",
    allow = "audioware_exists",
    detour = "noop"
)]
pub struct HookAudioSystemGlobalParameter;
