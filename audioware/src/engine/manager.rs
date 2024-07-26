use kira::sound::PlaybackState;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::ops::DerefMut;
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
use red4ext_rs::types::{CName, EntityId};

use crate::error::InternalError;

pub struct Manager;

pub trait Manage {
    fn stop(&mut self, tween: Option<Tween>);
    fn stop_by(
        &mut self,
        event_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    );
    fn stop_for(&mut self, entity_id: &EntityId, tween: Option<Tween>);
    fn pause(&mut self, tween: Option<Tween>);
    fn resume(&mut self, tween: Option<Tween>);
}

pub struct StaticStorage;
pub struct StreamStorage;

static STATICS: Lazy<Mutex<HashMap<HandleId, StaticSoundHandle>>> = Lazy::new(Default::default);
static STREAMS: Lazy<Mutex<HashMap<HandleId, StreamingSoundHandle<FromFileError>>>> =
    Lazy::new(Default::default);

impl Manager {
    pub fn try_lock<'a>() -> Result<MutexGuard<'a, AudioManager>, InternalError> {
        static INSTANCE: OnceLock<Mutex<AudioManager<DefaultBackend>>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                let manager = AudioManager::new(AudioManagerSettings::default())
                    .expect("instantiate audio manager");
                Mutex::new(manager)
            })
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "audio manager",
            })
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

impl Manager {
    pub fn clear_tracks(tween: Option<Tween>) -> Result<(), InternalError> {
        Self::stop(tween)?;
        StaticStorage::try_lock()?.deref_mut().clear();
        StreamStorage::try_lock()?.deref_mut().clear();
        Ok(())
    }
    pub fn stop(tween: Option<Tween>) -> Result<(), InternalError> {
        StaticStorage::try_lock()?.deref_mut().stop(tween);
        StreamStorage::try_lock()?.deref_mut().stop(tween);
        Ok(())
    }
    pub fn stop_by(
        event_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    ) -> Result<(), InternalError> {
        StaticStorage::try_lock()?
            .deref_mut()
            .stop_by(event_name, entity_id, emitter_name, tween);
        StreamStorage::try_lock()?
            .deref_mut()
            .stop_by(event_name, entity_id, emitter_name, tween);
        Ok(())
    }
    pub fn stop_for(
        &mut self,
        entity_id: &EntityId,
        tween: Option<Tween>,
    ) -> Result<(), InternalError> {
        StaticStorage::try_lock()?
            .deref_mut()
            .stop_for(entity_id, tween);
        StreamStorage::try_lock()?
            .deref_mut()
            .stop_for(entity_id, tween);
        Ok(())
    }
    pub fn pause(tween: Option<Tween>) -> Result<(), InternalError> {
        StaticStorage::try_lock()?.deref_mut().pause(tween);
        StreamStorage::try_lock()?.deref_mut().pause(tween);
        Ok(())
    }
    pub fn resume(tween: Option<Tween>) -> Result<(), InternalError> {
        StaticStorage::try_lock()?.deref_mut().resume(tween);
        StreamStorage::try_lock()?.deref_mut().resume(tween);
        Ok(())
    }
}

macro_rules! impl_manage {
    ($value_ty:ty) => {
        impl Manage for $value_ty {
            fn stop(&mut self, tween: Option<kira::tween::Tween>) {
                self.values_mut()
                    .par_bridge()
                    .filter(|v| {
                        v.state() != PlaybackState::Stopped && v.state() != PlaybackState::Stopping
                    })
                    .for_each(|v| v.stop(tween.unwrap_or_default()));
            }

            fn stop_by(
                &mut self,
                event_name: &CName,
                entity_id: Option<&EntityId>,
                emitter_name: Option<&CName>,
                tween: Option<Tween>,
            ) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.event_name() == event_name
                            && k.entity_id() == entity_id
                            && k.emitter_name() == emitter_name
                            && v.state() != PlaybackState::Stopped
                            && v.state() != PlaybackState::Stopping
                    })
                    .for_each(|(_, v)| v.stop(tween.unwrap_or_default()));
            }

            fn stop_for(&mut self, entity_id: &EntityId, tween: Option<Tween>) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.entity_id() == Some(entity_id)
                            && v.state() != PlaybackState::Stopped
                            && v.state() != PlaybackState::Stopping
                    })
                    .for_each(|(_, v)| v.stop(tween.unwrap_or_default()));
            }

            fn pause(&mut self, tween: Option<kira::tween::Tween>) {
                self.values_mut()
                    .par_bridge()
                    .filter(|v| v.state() == PlaybackState::Playing)
                    .for_each(|v| v.pause(tween.unwrap_or_default()));
            }

            fn resume(&mut self, tween: Option<kira::tween::Tween>) {
                self.values_mut()
                    .par_bridge()
                    .filter(|v| {
                        v.state() == PlaybackState::Paused || v.state() == PlaybackState::Pausing
                    })
                    .for_each(|v| v.resume(tween.unwrap_or_default()));
            }
        }
    };
}

impl_manage!(HashMap<HandleId,StaticSoundHandle>);
impl_manage!(HashMap<HandleId,StreamingSoundHandle<FromFileError>>);
