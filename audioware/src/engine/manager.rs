use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use super::id::HandleId;
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
};

use crate::error::InternalError;

pub struct Manager;

static STATICS: OnceLock<Mutex<HashMap<HandleId, StaticSoundHandle>>> = OnceLock::new();
static STREAMS: OnceLock<Mutex<HashMap<HandleId, StreamingSoundHandle<FromFileError>>>> =
    OnceLock::new();

fn audio_manager() -> &'static Mutex<AudioManager<DefaultBackend>> {
    static INSTANCE: OnceLock<Mutex<AudioManager<DefaultBackend>>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let manager =
            AudioManager::new(AudioManagerSettings::default()).expect("instantiate audio manager");
        Mutex::new(manager)
    })
}

impl Manager {
    pub fn try_lock<'a>() -> Result<MutexGuard<'a, AudioManager>, InternalError> {
        audio_manager()
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "audio manager",
            })
    }
}
