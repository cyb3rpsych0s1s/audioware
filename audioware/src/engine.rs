mod commands;
mod effects;
mod eq;
mod id;
mod modulators;
mod scene;
mod settings;
mod tracks;

use std::ops::DerefMut;

use audioware_bank::{BankData, Banks};
use audioware_core::AudioDuration;
use audioware_manifest::{PlayerGender, ScnDialogLineType, SpokenLocale};
pub use commands::*;
use dashmap::DashMap;
pub use id::*;
use kira::{
    manager::{
        backend::cpal::{CpalBackend, CpalBackendSettings},
        AudioManager, AudioManagerSettings,
    },
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    tween::Tween,
};
use modulators::Modulators;
use red4ext_rs::{
    log,
    types::{CName, EntityId},
    PluginOps,
};
use scene::Scene;
pub use settings::*;
pub use tracks::*;

use crate::{config::BufferSize, error::Error, Audioware, COMMAND, LIFECYCLE};

pub struct Engine {
    pub banks: Banks,
    pub statics: DashMap<HandleId, StaticSoundHandle>,
    pub streams: DashMap<HandleId, StreamingSoundHandle<FromFileError>>,
    pub scene: Scene,
    pub tracks: Tracks,
    pub manager: AudioManager<CpalBackend>,
}

impl Engine {
    pub(crate) fn try_new(buffer_size: BufferSize, banks: Banks) -> Result<Engine, Error> {
        let mut backend_settings = CpalBackendSettings::default();
        if buffer_size != BufferSize::Auto {
            backend_settings.buffer_size = cpal::BufferSize::Fixed(buffer_size as u32);
            log::info!(
                Audioware::env(),
                "buffer size read from .ini: {}",
                buffer_size as u32
            );
        }
        let manager_settings = AudioManagerSettings::<CpalBackend> {
            backend_settings,
            ..Default::default()
        };
        let statics = DashMap::<HandleId, StaticSoundHandle>::with_capacity(32);
        let streams = DashMap::<HandleId, StreamingSoundHandle<FromFileError>>::with_capacity(32);
        let mut manager =
            AudioManager::<CpalBackend>::new(manager_settings).expect("instantiate audio manager");
        let tracks = Tracks::try_new(&mut manager).expect("tracks");
        let scene = Scene::try_new(&mut manager, &tracks).expect("scene");
        Modulators::setup(&mut manager).expect("modulators");
        Ok(Engine {
            banks,
            statics,
            streams,
            scene,
            tracks,
            manager,
        })
    }
    pub fn play(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
    ) -> Result<(), Error> {
        let id = self.banks.try_get(&event_name, &spoken, gender.as_ref())?;
        match self.banks.data(id) {
            either::Either::Left(data) => {
                let duration = data.slice_duration().as_secs_f32();
                let handle = self.manager.play(data)?;
                self.statics
                    .insert(HandleId::new(id, entity_id, emitter_name), handle);
                if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
                    crate::propagate_subtitles(
                        event_name,
                        entity_id,
                        emitter_name,
                        ScnDialogLineType::default(),
                        duration,
                    );
                }
                Ok(())
            }
            either::Either::Right(data) => {
                let duration = data.slice_duration().as_secs_f32();
                let handle = self.manager.play(data)?;
                self.streams
                    .insert(HandleId::new(id, entity_id, emitter_name), handle);
                if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
                    crate::propagate_subtitles(
                        event_name,
                        entity_id,
                        emitter_name,
                        ScnDialogLineType::default(),
                        duration,
                    );
                }
                Ok(())
            }
        }
    }
    pub fn stop(
        &mut self,
        event_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    ) -> Result<(), Error> {
        use rayon::iter::ParallelIterator;
        self.statics
            .par_iter_mut()
            .filter(|entry| {
                entry.key().event_name() == event_name
                    && entry.key().entity_id() == entity_id
                    && entry.key().emitter_name() == emitter_name
                    && entry.value().state() != PlaybackState::Stopped
            })
            .for_each(|mut entry| {
                entry.value_mut().stop(tween.unwrap_or_default());
            });
        self.streams
            .par_iter_mut()
            .filter(|entry| {
                entry.key().event_name() == event_name
                    && entry.key().entity_id() == entity_id
                    && entry.key().emitter_name() == emitter_name
                    && entry.value().state() != PlaybackState::Stopped
            })
            .for_each(|mut entry| {
                entry.value_mut().stop(tween.unwrap_or_default());
            });
        Ok(())
    }
    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        Scene::is_registered_emitter(&entity_id)
    }
    /// Internally notify about plugin and game lifecycle.
    pub fn notify(lifecycle: Lifecycle) {
        if let Some(x) = LIFECYCLE.get() {
            if let Some(mut x) = x.try_write() {
                if let Some(x) = x.deref_mut() {
                    if let Err(e) = x.try_send(lifecycle) {
                        log::error!(Audioware::env(), "failed to notify lifecycle: {}", e);
                    }
                }
            }
        }
    }
    /// Send sound command.
    pub fn send(command: Command) {
        if let Some(x) = COMMAND.get() {
            if let Some(mut x) = x.try_write() {
                if let Some(x) = x.deref_mut() {
                    if let Err(e) = x.try_send(command) {
                        log::error!(Audioware::env(), "failed to send command: {}", e);
                    }
                }
            }
        }
    }
    /// Current number of registered [Scene] audio emitters.
    pub fn emitters_count() -> i32 {
        Scene::emitters_count()
    }
    /// Whenever [Scene] audio emitter dies in-game.
    pub fn on_emitter_dies(entity_id: EntityId) {
        Self::notify(Lifecycle::UnregisterEmitter { entity_id });
    }
    /// Toggle [Scene] audio emitters synchonization.
    pub(crate) fn toggle_sync_emitters(enable: bool) {
        todo!()
    }
}
