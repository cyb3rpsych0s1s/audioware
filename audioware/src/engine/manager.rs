use std::sync::{Mutex, OnceLock};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::StaticSoundData,
};
use lazy_static::lazy_static;

use crate::engine;

use super::id::SoundId;

lazy_static! {
    static ref AUDIO_MANAGER: OnceLock<Mutex<AudioManager<DefaultBackend>>> = OnceLock::default();
}

pub(crate) fn setup() {
    let mut manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
    if let Err(_) = engine::tracks::setup(&mut manager) {
        red4ext_rs::error!("error initializing tracks on Audio Manager");
    }
    if let Err(_) = AUDIO_MANAGER.set(Mutex::new(manager)) {
        red4ext_rs::error!("error initializing Audio Manager");
    }
}

pub(crate) fn play(sound: SoundId) {
    if let Some(mut manager) = AUDIO_MANAGER.get().and_then(|x| x.try_lock().ok()) {
        if let Ok(data) = engine::banks::data(sound.clone()) {
            if let Err(_) = manager.play(data) {
                red4ext_rs::error!("error playing sound {sound}");
            }
        } else {
            red4ext_rs::warn!("unknown sound ({sound})");
        }
    }
}
