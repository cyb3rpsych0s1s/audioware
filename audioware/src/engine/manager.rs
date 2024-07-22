use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use super::id::HandleId;
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
};
use once_cell::sync::Lazy;

use crate::error::InternalError;

pub struct Manager;

static STATICS: Lazy<Mutex<HashMap<HandleId, StaticSoundHandle>>> = Lazy::new(Default::default);
static STREAMS: Lazy<Mutex<HashMap<HandleId, StreamingSoundHandle<FromFileError>>>> =
    Lazy::new(Default::default);

fn audio_manager() -> &'static Mutex<AudioManager<DefaultBackend>> {
    static INSTANCE: OnceLock<Mutex<AudioManager<DefaultBackend>>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let manager =
            AudioManager::new(AudioManagerSettings::default()).expect("instantiate audio manager");
        Mutex::new(manager)
    })
}

pub fn static_handles<'a>(
) -> Result<MutexGuard<'a, HashMap<HandleId, StaticSoundHandle>>, InternalError> {
    STATICS.try_lock().map_err(|_| InternalError::Contention {
        origin: "static sound handles",
    })
}

pub fn streaming_handles<'a>(
) -> Result<MutexGuard<'a, HashMap<HandleId, StreamingSoundHandle<FromFileError>>>, InternalError> {
    STREAMS.try_lock().map_err(|_| InternalError::Contention {
        origin: "static sound handles",
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
