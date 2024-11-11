use std::fmt::Debug;

use audioware_bank::{BankData, Banks};
use audioware_manifest::{PlayerGender, SpokenLocale};
use either::Either;
use glam::Vec3;
use handles::{Emitter, Handles};
use kira::{
    manager::{backend::Backend, AudioManager, AudioManagerSettings},
    track::TrackBuilder,
};
use red4ext_rs::types::{CName, EntityId, Opt, Ref};
use scene::Scene;
use track::Tracks;

use crate::{
    error::{EngineError, Error},
    EmitterSettings, Tween,
};

pub mod queue;

mod handles;
mod scene;
mod track;

pub struct Engine<B: Backend> {
    pub banks: Banks,
    pub handles: Handles,
    pub tracks: Tracks,
    pub scene: Scene,
    pub manager: AudioManager<B>,
}

impl<B> Engine<B>
where
    B: Backend,
    <B as Backend>::Error: Debug,
{
    pub fn try_new(settings: AudioManagerSettings<B>) -> Result<Engine<B>, Error> {
        let banks = Banks::new();
        let capacity = settings.capacities.sound_capacity as usize;
        let mut manager = AudioManager::new(settings).map_err(|_| Error::Engine {
            source: EngineError::Manager {
                origin: "audio manager",
            },
        })?;
        let ambience = manager.add_sub_track(TrackBuilder::new())?;
        let scene = Scene::try_new(&mut manager, &ambience)?;
        Ok(Engine {
            banks,
            manager,
            handles: Handles::with_capacity(capacity),
            scene,
            tracks: Tracks { ambience },
        })
    }

    pub fn play(
        &mut self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
    ) {
        if let Ok(key) = self.banks.try_get(&event_name, &spoken, gender.as_ref()) {
            let data = self.banks.data(key);
            let emitter = Emitter::new(entity_id, emitter_name);
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(data) {
                        self.handles.store_static(handle, event_name, emitter);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(data) {
                        self.handles.store_stream(handle, event_name, emitter);
                    }
                }
            }
        }
    }

    pub fn stop(
        &mut self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        if self.banks.exists(&event_name) {
            self.handles
                .stop(event_name, Emitter::new(entity_id, emitter_name), tween);
        }
    }

    pub fn pause(&mut self) {
        self.handles.pause();
    }

    pub fn resume(&mut self) {
        self.handles.resume();
    }

    pub fn reclaim(&mut self) {
        self.handles.reclaim();
    }

    pub fn register_emitter(
        &mut self,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
    ) -> bool {
        self.scene
            .add_emitter(Vec3::ZERO, entity_id, emitter_name, emitter_settings)
            .is_ok()
    }

    pub fn unregister_emitter(&mut self, entity_id: EntityId) -> bool {
        self.scene.remove_emitter(entity_id).unwrap_or(false)
    }
}
