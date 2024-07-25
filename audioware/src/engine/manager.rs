use kira::sound::PlaybackState;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use super::{effects::IMMEDIATELY, id::HandleId};
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
    tween::Tween,
};
use once_cell::sync::Lazy;
use red4ext_rs::types::{CName, EntityId};

use crate::error::{Error, InternalError};

pub struct Manager;

pub trait Manage {
    fn stop(&mut self, tween: Option<Tween>);
    fn stop_by_cname(&mut self, cname: &CName, tween: Option<Tween>);
    fn stop_by_cname_for_entity(
        &mut self,
        cname: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    );
    fn pause(&mut self, tween: Option<Tween>);
    fn pause_by_cname(&mut self, cname: &CName, tween: Option<Tween>);
    fn pause_by_cname_for_entity(
        &mut self,
        cname: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    );
    fn resume(&mut self, tween: Option<Tween>);
    fn resume_by_cname(&mut self, cname: &CName, tween: Option<Tween>);
    fn resume_by_cname_for_entity(
        &mut self,
        cname: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    );
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
    pub fn stop_all() -> Result<(), Error> {
        for (_, v) in StaticStorage::try_lock()?.iter_mut() {
            v.stop(IMMEDIATELY);
        }
        for (_, v) in StreamStorage::try_lock()?.iter_mut() {
            v.stop(IMMEDIATELY);
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

macro_rules! impl_manage {
    ($value_ty:ty) => {
        impl Manage for HashMap<HandleId, $value_ty> {
            fn stop(&mut self, tween: Option<Tween>) {
                self.values_mut()
                    .par_bridge()
                    .filter(|v| {
                        v.state() != PlaybackState::Stopped && v.state() != PlaybackState::Stopping
                    })
                    .for_each(|v| v.stop(tween.unwrap_or_default()));
            }

            fn stop_by_cname(&mut self, cname: &CName, tween: Option<Tween>) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.event_name() == cname
                            && v.state() != PlaybackState::Stopped
                            && v.state() != PlaybackState::Stopping
                    })
                    .for_each(|(_, v)| v.stop(tween.unwrap_or_default()));
            }

            fn stop_by_cname_for_entity(
                &mut self,
                cname: &CName,
                entity_id: &EntityId,
                tween: Option<Tween>,
            ) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.event_name() == cname
                            && k.entity_id() == Some(entity_id)
                            && v.state() != PlaybackState::Stopped
                            && v.state() != PlaybackState::Stopping
                    })
                    .for_each(|(_, v)| v.stop(tween.unwrap_or_default()));
            }

            fn pause(&mut self, tween: Option<Tween>) {
                self.values_mut()
                    .par_bridge()
                    .filter(|v| v.state() == PlaybackState::Playing)
                    .for_each(|v| v.pause(tween.unwrap_or_default()));
            }

            fn pause_by_cname(&mut self, cname: &CName, tween: Option<Tween>) {
                self.par_iter_mut()
                    .filter(|(k, v)| k.event_name() == cname && v.state() == PlaybackState::Playing)
                    .for_each(|(_, v)| v.pause(tween.unwrap_or_default()));
            }

            fn pause_by_cname_for_entity(
                &mut self,
                cname: &CName,
                entity_id: &EntityId,
                tween: Option<Tween>,
            ) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.event_name() == cname
                            && k.entity_id() == Some(entity_id)
                            && v.state() == PlaybackState::Playing
                    })
                    .for_each(|(_, v)| v.pause(tween.unwrap_or_default()));
            }

            fn resume(&mut self, tween: Option<Tween>) {
                self.values_mut()
                    .par_bridge()
                    .filter(|v| {
                        v.state() == PlaybackState::Paused || v.state() == PlaybackState::Pausing
                    })
                    .for_each(|v| v.resume(tween.unwrap_or_default()));
            }

            fn resume_by_cname(&mut self, cname: &CName, tween: Option<Tween>) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.event_name() == cname
                            && (v.state() == PlaybackState::Paused
                                || v.state() == PlaybackState::Pausing)
                    })
                    .for_each(|(_, v)| v.resume(tween.unwrap_or_default()));
            }

            fn resume_by_cname_for_entity(
                &mut self,
                cname: &CName,
                entity_id: &EntityId,
                tween: Option<Tween>,
            ) {
                self.par_iter_mut()
                    .filter(|(k, v)| {
                        k.event_name() == cname
                            && k.entity_id() == Some(entity_id)
                            && (v.state() == PlaybackState::Paused
                                || v.state() == PlaybackState::Pausing)
                    })
                    .for_each(|(_, v)| v.resume(tween.unwrap_or_default()));
            }
        }
    };
}

impl_manage!(StaticSoundHandle);
impl_manage!(StreamingSoundHandle<FromFileError>);
