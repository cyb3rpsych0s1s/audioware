use audioware_macros::NativeFunc;
use red4ext_rs::types::{CName, EntityId};

use crate::hook::address::ON_AUDIOSYSTEM_PLAY_ON_EMITTER;

fn audioware_exists(params: &(CName, EntityId, CName)) -> bool {
    audioware_core::utils::dbg(format!(
        "AudioSystem::PlayOnEmitter({}, {:?}, {})",
        params.0, params.1, params.2
    ));

    false
}

pub fn noop(_: (CName, EntityId, CName)) {}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_PLAY_ON_EMITTER,
    inputs = "(CName, EntityId, CName)",
    allow = "audioware_exists",
    detour = "noop"
)]
pub struct HookAudioSystemPlayOnEmitter;
