use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use audioware_core::Error;
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    tween::Tween,
};
use once_cell::sync::OnceCell;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use red4ext_rs::types::{CName, EntityId};

use super::{effect::MODULATOR_NAME, id::HandleId};

static MODULATOR: OnceCell<Mutex<TweenerHandle>> = OnceCell::new();
static STATICS: OnceCell<Mutex<HashMap<HandleId, StaticSoundHandle>>> = OnceCell::new();
static STREAMS: OnceCell<Mutex<HashMap<HandleId, StreamingSoundHandle<FromFileError>>>> =
    OnceCell::new();

pub fn audio_manager() -> &'static Mutex<AudioManager<DefaultBackend>> {
    static INSTANCE: OnceCell<Mutex<AudioManager<DefaultBackend>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut manager =
            AudioManager::new(AudioManagerSettings::default()).expect("instantiate audio manager");
        let tweener = manager
            .add_modulator(TweenerBuilder { initial_value: 0.0 })
            .expect("instantiate tweener builder");
        MODULATOR
            .set(Mutex::new(tweener))
            .expect("store tweener handle");
        Mutex::new(manager)
    })
}

pub fn audio_modulator() -> &'static Mutex<TweenerHandle> {
    let _ = audio_manager();
    let _ = CName::new_pooled(MODULATOR_NAME);
    MODULATOR.get().expect("initialized modulator")
}

pub fn maybe_statics() -> Result<MutexGuard<'static, HashMap<HandleId, StaticSoundHandle>>, Error> {
    STATICS
        .get_or_init(Default::default)
        .try_lock()
        .map_err(Error::from)
}

pub fn maybe_streams(
) -> Result<MutexGuard<'static, HashMap<HandleId, StreamingSoundHandle<FromFileError>>>, Error> {
    STREAMS
        .get_or_init(Default::default)
        .try_lock()
        .map_err(Error::from)
}

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
                        AsRef::<CName>::as_ref(&k.key) == cname
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
                        AsRef::<CName>::as_ref(&k.key) == cname
                            && k.entity_id.as_ref() == Some(entity_id)
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
                    .filter(|(k, v)| {
                        AsRef::<CName>::as_ref(&k.key) == cname
                            && v.state() == PlaybackState::Playing
                    })
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
                        AsRef::<CName>::as_ref(&k.key) == cname
                            && k.entity_id.as_ref() == Some(entity_id)
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
                        AsRef::<CName>::as_ref(&k.key) == cname
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
                        AsRef::<CName>::as_ref(&k.key) == cname
                            && k.entity_id.as_ref() == Some(entity_id)
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
