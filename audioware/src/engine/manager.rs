use audioware_bank::Banks;
use audioware_bank::Id;
use crossbeam::channel::bounded;
use dashmap::DashMap;
use kira::manager::backend::cpal::CpalBackend;
use kira::manager::backend::cpal::CpalBackendSettings;
use kira::manager::error::PlaySoundError;
use kira::sound::static_sound::StaticSoundData;
use kira::sound::streaming::StreamingSoundData;
use kira::sound::PlaybackState;
use kira::sound::SoundData;
use kira::OutputDestination;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use red4ext_rs::log;
use red4ext_rs::types::Ref;
use red4ext_rs::PluginOps;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;

use super::id::HandleId;
use super::AudioSettingsExt;
use super::Context;
use super::ToOutputDestination;
use kira::{
    manager::{AudioManager, AudioManagerSettings},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
    tween::Tween,
};
use once_cell::sync::Lazy;
use red4ext_rs::types::{CName, EntityId};

use crate::config::BufferSize;
use crate::engine::commands::OuterCommand;
use crate::engine::handle_receive;
use crate::engine::modulators::Modulators;
use crate::engine::SENDER;
use crate::error::Error;
use crate::error::InternalError;
use crate::ext::MergeArgs;
use crate::types::ToTween;
use crate::Audioware;

pub struct Manager;

pub trait Stopped {
    fn stopped(&self) -> bool;
}

impl<T: State> Stopped for T {
    #[inline]
    fn stopped(&self) -> bool {
        self.state() == PlaybackState::Stopped
    }
}

pub struct StaticStorage;
pub struct StreamStorage;

static STATICS: Lazy<Mutex<DashMap<HandleId, StaticSoundHandle>>> = Lazy::new(Default::default);
static STREAMS: Lazy<Mutex<DashMap<HandleId, StreamingSoundHandle<FromFileError>>>> =
    Lazy::new(Default::default);

impl Manager {
    pub(super) fn try_lock<'a>() -> Result<MutexGuard<'a, AudioManager>, Error> {
        static INSTANCE: OnceLock<Mutex<AudioManager<CpalBackend>>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                let mut backend_settings = CpalBackendSettings::default();
                let buffer_size = BufferSize::read_ini();
                if buffer_size != BufferSize::Auto {
                    backend_settings.buffer_size = cpal::BufferSize::Fixed(buffer_size as u32);
                    log::info!(
                        Audioware::env(),
                        "buffer size read from .ini: {}",
                        buffer_size as u32
                    );
                }
                let manager_settings = AudioManagerSettings {
                    backend_settings,
                    ..Default::default()
                };
                let commands_capacity = manager_settings.capacities.command_capacity;
                let mut manager =
                    AudioManager::new(manager_settings).expect("instantiate audio manager");
                Modulators::setup(&mut manager).expect("modulators");
                let (s, r) = bounded::<OuterCommand>(commands_capacity);
                let _ = SENDER.set(s);
                thread::spawn(move || handle_receive(r));
                Mutex::new(manager)
            })
            .try_lock()
            .map_err(|_| {
                InternalError::Contention {
                    origin: "audio manager",
                }
                .into()
            })
    }
    /// Retain non-stopped sounds only.
    pub(super) fn reclaim() -> Result<(), Error> {
        let storage = StaticStorage::try_lock()?;
        storage.retain(|_, v| !v.stopped());
        let storage = StreamStorage::try_lock()?;
        storage.retain(|_, v| !v.stopped());
        Ok(())
    }
}

impl StaticStorage {
    pub fn try_lock<'a>(
    ) -> Result<MutexGuard<'a, DashMap<HandleId, StaticSoundHandle>>, InternalError> {
        STATICS.try_lock().map_err(|_| InternalError::Contention {
            origin: "static sound handles",
        })
    }
}

impl StreamStorage {
    pub fn try_lock<'a>(
    ) -> Result<MutexGuard<'a, DashMap<HandleId, StreamingSoundHandle<FromFileError>>>, InternalError>
    {
        STREAMS.try_lock().map_err(|_| InternalError::Contention {
            origin: "static sound handles",
        })
    }
}

impl Manager {
    /// Stop and clear all tracks.
    pub fn clear_tracks(&mut self, tween: Option<Tween>) -> Result<(), InternalError> {
        self.stop(tween)?;
        StaticStorage::try_lock()?.deref_mut().clear();
        StreamStorage::try_lock()?.deref_mut().clear();
        Ok(())
    }
}

pub trait Play<T> {
    type Handle;
    fn play(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        destination: Option<OutputDestination>,
        extra: T,
    ) -> Result<(f32, Self::Handle), Error>;
}

impl<T> Play<Option<Tween>> for T
where
    T: Data + SoundData,
    PlaySoundError<<T as SoundData>::Error>: Into<Error>,
{
    type Handle = <T as SoundData>::Handle;

    fn play(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        destination: Option<OutputDestination>,
        tween: Option<Tween>,
    ) -> Result<(f32, Self::Handle), Error> {
        let duration = self.duration_as_secs();
        let handle = manager
            .play(
                self.with_tween(tween)
                    .with_route(id, entity_id, destination),
            )
            .map_err(Into::into)?;
        Ok((duration, handle))
    }
}

pub trait Data {
    fn duration_as_secs(&self) -> f32;
    fn with_tween(self, tween: Option<Tween>) -> Self;
    fn with_route(
        self,
        id: &Id,
        entity_id: Option<EntityId>,
        destination: Option<OutputDestination>,
    ) -> Self;
}

macro_rules! impl_data {
    ($ty:path) => {
        impl Data for $ty {
            fn with_tween(self, tween: Option<Tween>) -> Self {
                if tween.is_some() {
                    return self.fade_in_tween(tween);
                }
                self
            }

            fn with_route(
                self,
                id: &Id,
                entity_id: Option<EntityId>,
                destination: Option<OutputDestination>,
            ) -> Self {
                if let Some(destination) = destination {
                    self.output_destination(destination)
                } else {
                    self.output_destination(
                        Context::new(id, entity_id.as_ref()).output_destination(),
                    )
                }
            }

            fn duration_as_secs(&self) -> f32 {
                self.duration().as_secs_f32()
            }
        }
    };
}

impl_data!(StaticSoundData);
impl_data!(StreamingSoundData<FromFileError>);

pub trait Store {
    fn store(
        self,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    ) -> Result<(), Error>;
}

impl Store for StaticSoundHandle {
    fn store(
        self,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    ) -> Result<(), Error> {
        let storage = StaticStorage::try_lock()?;
        storage.insert(HandleId::new(id, entity_id, emitter_name), self);
        Ok(())
    }
}

impl Store for StreamingSoundHandle<FromFileError> {
    fn store(
        self,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    ) -> Result<(), Error> {
        let storage = StreamStorage::try_lock()?;
        storage.insert(HandleId::new(id, entity_id, emitter_name), self);
        Ok(())
    }
}

pub trait PlayAndStore<T> {
    fn play_and_store(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        destination: Option<OutputDestination>,
        extra: T,
    ) -> Result<f32, Error>;
}

impl<T, U> PlayAndStore<U> for T
where
    T: Data + SoundData,
    PlaySoundError<<T as SoundData>::Error>: Into<Error>,
    <T as SoundData>::Handle: Store,
    U: ToTween,
{
    fn play_and_store(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        destination: Option<OutputDestination>,
        tween: U,
    ) -> Result<f32, Error> {
        let (duration, handle) =
            self.play(manager, id, entity_id, destination, tween.into_tween())?;
        handle.store(id, entity_id, emitter_name)?;
        Ok(duration)
    }
}

impl<U> PlayAndStore<U> for Manager
where
    U: ToTween,
{
    fn play_and_store(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        destination: Option<OutputDestination>,
        tween: U,
    ) -> Result<f32, Error> {
        match Banks::data(id) {
            either::Either::Left(data) => {
                data.play_and_store(manager, id, entity_id, emitter_name, destination, tween)
            }
            either::Either::Right(data) => {
                data.play_and_store(manager, id, entity_id, emitter_name, destination, tween)
            }
        }
    }
}

impl<T> Play<Ref<AudioSettingsExt>> for T
where
    T: Data + SoundData + MergeArgs,
    PlaySoundError<<T as SoundData>::Error>: Into<Error>,
{
    type Handle = <T as SoundData>::Handle;

    fn play(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        destination: Option<OutputDestination>,
        ext: Ref<AudioSettingsExt>,
    ) -> Result<(f32, Self::Handle), Error> {
        let duration = self.duration_as_secs();
        let handle = manager
            .play(self.merge_args(&ext).with_route(id, entity_id, destination))
            .map_err(Into::into)?;
        Ok((duration, handle))
    }
}

impl<T> PlayAndStore<Ref<AudioSettingsExt>> for T
where
    T: Data + SoundData + MergeArgs,
    PlaySoundError<<T as SoundData>::Error>: Into<Error>,
    <T as SoundData>::Handle: Store,
{
    fn play_and_store(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        destination: Option<OutputDestination>,
        ext: Ref<AudioSettingsExt>,
    ) -> Result<f32, Error> {
        let (duration, handle) = self.play(manager, id, entity_id, destination, ext)?;
        handle.store(id, entity_id, emitter_name)?;
        Ok(duration)
    }
}

impl PlayAndStore<Ref<AudioSettingsExt>> for Manager {
    fn play_and_store(
        self,
        manager: &mut AudioManager,
        id: &Id,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        destination: Option<OutputDestination>,
        ext: Ref<AudioSettingsExt>,
    ) -> Result<f32, Error> {
        match Banks::data(id) {
            either::Either::Left(data) => {
                data.play_and_store(manager, id, entity_id, emitter_name, destination, ext)
            }
            either::Either::Right(data) => {
                data.play_and_store(manager, id, entity_id, emitter_name, destination, ext)
            }
        }
    }
}

pub trait Stop {
    type Output;

    fn stop(&mut self, tween: Option<Tween>) -> Self::Output;
}

impl<T: Send + Sync + State + Stop> Stop for DashMap<HandleId, T> {
    type Output = ();

    fn stop(&mut self, tween: Option<Tween>) -> Self::Output {
        self.par_iter_mut()
            .filter(|v| v.value().state() != PlaybackState::Stopped)
            .for_each(|mut v| {
                v.value_mut().stop(tween);
            });
    }
}

pub trait State {
    fn state(&self) -> PlaybackState;
}

impl State for StaticSoundHandle {
    #[inline]
    fn state(&self) -> PlaybackState {
        Self::state(self)
    }
}

impl State for StreamingSoundHandle<FromFileError> {
    #[inline]
    fn state(&self) -> PlaybackState {
        Self::state(self)
    }
}

impl Stop for Manager {
    type Output = Result<(), InternalError>;

    fn stop(&mut self, tween: Option<Tween>) -> Result<(), InternalError> {
        StaticStorage::try_lock()?.deref_mut().stop(tween);
        StreamStorage::try_lock()?.deref_mut().stop(tween);
        Ok(())
    }
}

impl Stop for StaticSoundHandle {
    type Output = ();

    #[inline]
    fn stop(&mut self, tween: Option<Tween>) -> Self::Output {
        Self::stop(self, tween.unwrap_or_default());
    }
}

impl Stop for StreamingSoundHandle<FromFileError> {
    type Output = ();

    #[inline]
    fn stop(&mut self, tween: Option<Tween>) -> Self::Output {
        Self::stop(self, tween.unwrap_or_default());
    }
}

pub trait StopBy {
    type Output;

    fn stop_by(
        &mut self,
        event_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    ) -> Self::Output;
}

impl StopBy for Manager {
    type Output = Result<(), InternalError>;

    fn stop_by(
        &mut self,
        event_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    ) -> Self::Output {
        StaticStorage::try_lock()?
            .deref_mut()
            .stop_by(event_name, entity_id, emitter_name, tween);
        StreamStorage::try_lock()?
            .deref_mut()
            .stop_by(event_name, entity_id, emitter_name, tween);
        Ok(())
    }
}

impl<T: Send + Sync + State + Stop> StopBy for DashMap<HandleId, T> {
    type Output = ();

    fn stop_by(
        &mut self,
        event_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    ) -> Self::Output {
        self.par_iter_mut()
            .filter(|entry| {
                entry.key().event_name() == event_name
                    && entry.key().entity_id() == entity_id
                    && entry.key().emitter_name() == emitter_name
                    && entry.value().state() != PlaybackState::Stopped
            })
            .for_each(|mut entry| {
                entry.value_mut().stop(tween);
            });
    }
}

pub trait StopFor {
    type Output;
    fn stop_for(&mut self, entity_id: &EntityId, tween: Option<Tween>) -> Self::Output;
}

impl StopFor for Manager {
    type Output = Result<(), InternalError>;

    fn stop_for(&mut self, entity_id: &EntityId, tween: Option<Tween>) -> Self::Output {
        StaticStorage::try_lock()?
            .deref_mut()
            .stop_for(entity_id, tween);
        StreamStorage::try_lock()?
            .deref_mut()
            .stop_for(entity_id, tween);
        Ok(())
    }
}

impl<T: Send + Sync + State + Stop> StopFor for DashMap<HandleId, T> {
    type Output = ();

    fn stop_for(&mut self, entity_id: &EntityId, tween: Option<Tween>) -> Self::Output {
        self.par_iter_mut()
            .filter(|entry| {
                entry.key().entity_id() == Some(entity_id)
                    && entry.value().state() != PlaybackState::Stopped
            })
            .for_each(|mut entry| {
                entry.value_mut().stop(tween);
            });
    }
}

pub trait Pause {
    type Output;
    fn pause(&mut self, tween: Option<Tween>) -> Self::Output;
}

impl Pause for Manager {
    type Output = Result<(), InternalError>;

    fn pause(&mut self, tween: Option<Tween>) -> Self::Output {
        StaticStorage::try_lock()?.deref_mut().pause(tween);
        StreamStorage::try_lock()?.deref_mut().pause(tween);
        Ok(())
    }
}

impl<T: Send + Sync + State + Pause> Pause for DashMap<HandleId, T> {
    type Output = ();

    fn pause(&mut self, tween: Option<Tween>) -> Self::Output {
        self.par_iter_mut()
            .filter(|entry| {
                entry.value().state() != PlaybackState::Paused
                    && entry.value().state() != PlaybackState::Stopped
            })
            .for_each(|mut entry| {
                entry.value_mut().pause(tween);
            });
    }
}

impl Pause for StaticSoundHandle {
    type Output = ();

    #[inline]
    fn pause(&mut self, tween: Option<Tween>) -> Self::Output {
        Self::pause(self, tween.unwrap_or_default());
    }
}

impl Pause for StreamingSoundHandle<FromFileError> {
    type Output = ();

    #[inline]
    fn pause(&mut self, tween: Option<Tween>) -> Self::Output {
        Self::pause(self, tween.unwrap_or_default());
    }
}

pub trait Resume {
    type Output;
    fn resume(&mut self, tween: Option<Tween>) -> Self::Output;
}

impl Resume for Manager {
    type Output = Result<(), InternalError>;

    fn resume(&mut self, tween: Option<Tween>) -> Self::Output {
        StaticStorage::try_lock()?.deref_mut().resume(tween);
        StreamStorage::try_lock()?.deref_mut().resume(tween);
        Ok(())
    }
}

impl<T: Send + Sync + State + Resume> Resume for DashMap<HandleId, T> {
    type Output = ();

    fn resume(&mut self, tween: Option<Tween>) -> Self::Output {
        self.par_iter_mut()
            .filter(|entry| entry.value().state() != PlaybackState::Stopped)
            .for_each(|mut entry| {
                entry.value_mut().resume(tween);
            });
    }
}

impl Resume for StaticSoundHandle {
    type Output = ();

    #[inline]
    fn resume(&mut self, tween: Option<Tween>) -> Self::Output {
        Self::resume(self, tween.unwrap_or_default());
    }
}

impl Resume for StreamingSoundHandle<FromFileError> {
    type Output = ();

    #[inline]
    fn resume(&mut self, tween: Option<Tween>) -> Self::Output {
        Self::resume(self, tween.unwrap_or_default());
    }
}
