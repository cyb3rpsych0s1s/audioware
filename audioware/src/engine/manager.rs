use std::sync::{Mutex, MutexGuard, OnceLock};

use kira::manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings};
use lazy_static::lazy_static;

use crate::engine;

lazy_static! {
    static ref AUDIO_MANAGER: OnceLock<Mutex<AudioManager<DefaultBackend>>> = OnceLock::default();
}

pub(crate) fn setup() {
    let mut manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
    if engine::tracks::setup(&mut manager).is_err() {
        red4ext_rs::error!("error initializing tracks on Audio Manager");
    }
    if AUDIO_MANAGER.set(Mutex::new(manager)).is_err() {
        red4ext_rs::error!("error initializing Audio Manager");
    }
}

pub(crate) fn try_get_mut<'a>() -> Option<MutexGuard<'a, AudioManager>> {
    AUDIO_MANAGER.get().and_then(|x| x.try_lock().ok())
}
