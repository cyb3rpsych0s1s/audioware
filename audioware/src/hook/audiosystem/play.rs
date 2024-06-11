use audioware_macros::NativeFunc;
use red4ext_rs::types::{CName, EntityId};

use super::super::address::ON_AUDIOSYSTEM_PLAY;
use super::audioware_exists;
use crate::{engine::Engine, hook::Maybe, safe_call};

pub fn audioware_play((sound_name, entity_id, emitter_name): (CName, EntityId, CName)) {
    crate::utils::dbg(format!(
        "AudioSystem::Play({}, {:?}, {})",
        sound_name, entity_id, emitter_name
    ));
    safe_call!(Engine::play(
        &sound_name,
        entity_id.maybe(),
        emitter_name.maybe()
    ));
}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_PLAY,
    inputs = "(CName, EntityId, CName)",
    allow = "audioware_exists",
    detour = "audioware_play"
)]
pub struct HookAudioSystemPlay;
