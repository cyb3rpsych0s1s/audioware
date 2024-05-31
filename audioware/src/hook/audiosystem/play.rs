use audioware_macros::NativeFunc;
use red4ext_rs::types::{CName, EntityId};

use super::super::address::ON_AUDIOSYSTEM_PLAY;
use super::audioware_exists;
use crate::{engine::Engine, hook::Maybe};

pub fn audioware_play(params: (CName, EntityId, CName)) {
    #[rustfmt::skip] #[cfg(debug_assertions)] red4ext_rs::info!("hooked AudioSystem::Play");
    let (sound_name, entity_id, emitter_name) = params;
    if let Err(ref e) = Engine::play(&sound_name, entity_id.maybe(), emitter_name.maybe()) {
        match e {
            crate::engine::error::Error::BankRegistry { source } => match source {
                crate::bank::error::registry::Error::MissingLocale { .. }
                | crate::bank::error::registry::Error::RequireGender { .. } => {
                    red4ext_rs::warn!("{e}")
                }
                crate::bank::error::registry::Error::NotFound { .. } => {
                    red4ext_rs::error!("{e}")
                }
            },
            e => red4ext_rs::error!("{e}"),
        }
    }
}

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_PLAY,
    inputs = "(CName, EntityId, CName)",
    allow = "audioware_exists",
    detour = "audioware_play"
)]
pub struct HookAudioSystemPlay;
