use audioware_engine::Engine;
use audioware_macros::NativeFunc;
use red4ext_rs::types::{CName, EntityId};

use crate::hook::Maybe;

use super::super::address::ON_AUDIOSYSTEM_STOP;
use super::audioware_exists;

pub fn audioware_stop(params: (CName, EntityId, CName)) {
    audioware_core::dbg(format!(
        "AudioSystem::Stop({}, {:?}, {})",
        params.0, params.1, params.2
    ));
    let (sound_name, entity_id, _) = params;
    if let Some(entity_id) = entity_id.maybe() {
        Engine::stop_by_cname_for_entity(&sound_name, entity_id, None)
    } else {
        Engine::stop_by_cname(&sound_name, None)
    }
}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_STOP,
    inputs = "(CName, EntityId, CName)",
    allow = "audioware_exists",
    detour = "audioware_stop"
)]
pub struct HookAudioSystemStop;
