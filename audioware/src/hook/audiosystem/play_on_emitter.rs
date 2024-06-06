use audioware_macros::NativeFunc;
use red4ext_rs::types::{CName, EntityId};

use super::audioware_exists;
use crate::hook::address::ON_AUDIOSYSTEM_PLAY_ON_EMITTER;

pub fn audioware_play_on_emitter(params: (CName, EntityId, CName)) {
    #[rustfmt::skip] #[cfg(debug_assertions)] red4ext_rs::info!("hooked AudioSystem::PlayOnEmitter");
    let (sound_name, entity_id, emitter_name) = params;
    todo!()
}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_PLAY_ON_EMITTER,
    inputs = "(CName, EntityId, CName)",
    allow = "audioware_exists",
    detour = "audioware_play_on_emitter"
)]
pub struct HookAudioSystemPlayOnEmitter;
