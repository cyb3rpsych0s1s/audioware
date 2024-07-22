use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use super::id::HandleId;
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
    tween::Tween,
};
use once_cell::sync::Lazy;

use crate::error::{Error, InternalError};

pub struct Manager;

pub struct StaticStorage;
pub struct StreamStorage;

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

impl Manager {
    pub fn try_lock<'a>() -> Result<MutexGuard<'a, AudioManager>, InternalError> {
        audio_manager()
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "audio manager",
            })
    }
    pub fn stop_all() -> Result<(), Error> {
        for (_, v) in StaticStorage::try_lock()?.iter_mut() {
            v.stop(Tween::default());
        }
        for (_, v) in StreamStorage::try_lock()?.iter_mut() {
            v.stop(Tween::default());
        }
        Ok(())
    }
}

impl StaticStorage {
    pub fn try_lock<'a>(
    ) -> Result<MutexGuard<'a, HashMap<HandleId, StaticSoundHandle>>, InternalError> {
        STATICS.try_lock().map_err(|_| InternalError::Contention {
            origin: "static sound handles",
        })
    }
}

impl StreamStorage {
    pub fn try_lock<'a>(
    ) -> Result<MutexGuard<'a, HashMap<HandleId, StreamingSoundHandle<FromFileError>>>, InternalError>
    {
        STREAMS.try_lock().map_err(|_| InternalError::Contention {
            origin: "static sound handles",
        })
    }
}
