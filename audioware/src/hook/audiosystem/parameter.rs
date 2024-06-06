use super::super::address::ON_AUDIOSYSTEM_PARAMETER;
use audioware_macros::NativeFunc;
use red4ext_rs::types::{CName, EntityId};

fn audioware_exists(params: &(CName, f32, EntityId, CName)) -> bool {
    crate::utils::dbg(format!(
        "AudioSystem::Parameter({}, {}, {:?}, {})",
        params.0, params.1, params.2, params.3
    ));

    false
}

fn noop(_: (CName, f32, EntityId, CName)) {}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_PARAMETER,
    inputs = "(CName, f32, EntityId, CName)",
    allow = "audioware_exists",
    detour = "noop"
)]
pub struct HookAudioSystemParameter;
