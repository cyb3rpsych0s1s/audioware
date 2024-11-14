use std::{fmt::Debug, ops::DerefMut};

use audioware_bank::{BankData, Banks, Initialization};
use audioware_core::With;
use audioware_manifest::{PlayerGender, Settings, SpokenLocale};
use either::Either;
use eq::{EqPass, Preset};
use handles::{Emitter, Handles};
use kira::{
    manager::{backend::Backend, AudioManager, AudioManagerSettings},
    spatial::emitter::EmitterSettings,
    tween::Tween,
};
use modulators::{Modulators, Parameter};
use red4ext_rs::types::{CName, EntityId};
use scene::{EmitterId, Scene};
use tracks::Tracks;
use tweens::{DEFAULT, IMMEDIATELY};

use crate::{
    error::{EngineError, Error},
    utils::lifecycle,
};

pub mod queue;

pub mod eq;
mod handles;
mod modulators;
mod scene;
mod tracks;
mod tweens;

#[cfg(not(debug_assertions))]
static BANKS: std::sync::OnceLock<Banks> = std::sync::OnceLock::new();
#[cfg(debug_assertions)]
static BANKS: parking_lot::RwLock<Option<Banks>> = parking_lot::RwLock::new(None);

pub struct Engine<B: Backend> {
    pub handles: Handles,
    pub tracks: Tracks,
    pub scene: Option<Scene>,
    pub modulators: Modulators,
    pub manager: AudioManager<B>,
    pub banks: Banks,
    pub report: Initialization,
}

impl<B> Engine<B>
where
    B: Backend,
    <B as Backend>::Error: Debug,
{
    pub fn try_new(settings: AudioManagerSettings<B>) -> Result<Engine<B>, Error> {
        let (banks, report) = Banks::new();
        #[cfg(not(debug_assertions))]
        let _ = BANKS.set(banks.clone());
        #[cfg(debug_assertions)]
        {
            *BANKS.write().deref_mut() = Some(banks.clone());
        }
        let capacity = settings.capacities.sound_capacity as usize;
        let mut manager = AudioManager::new(settings).map_err(|_| Error::Engine {
            source: EngineError::Manager {
                origin: "audio manager",
            },
        })?;
        let modulators = Modulators::try_new(&mut manager)?;
        let tracks = Tracks::try_new(&mut manager, &modulators)?;
        Ok(Engine {
            banks,
            manager,
            handles: Handles::with_capacity(capacity),
            scene: None,
            modulators,
            tracks,
            report,
        })
    }

    pub fn any_handle(&self) -> bool {
        !self.handles.is_empty()
    }

    pub fn try_new_scene(&mut self) -> Result<(), Error> {
        self.scene = Some(Scene::try_new(&mut self.manager, &self.tracks)?);
        Ok(())
    }

    pub fn clear_scene(&mut self) {
        self.handles.stop_emitters(IMMEDIATELY);
        self.scene = None;
    }

    pub fn play(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
        tween: Option<Tween>,
    ) {
        if let Ok(key) = self.banks.try_get(&event_name, &spoken, gender.as_ref()) {
            let data = self.banks.data(key);
            let emitter = Emitter::new(entity_id, emitter_name);
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(data.with(tween)) {
                        self.handles.store_static(handle, event_name, emitter);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(data.with(tween)) {
                        self.handles.store_stream(handle, event_name, emitter);
                    }
                }
            }
        }
    }

    pub fn play_ext(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
        ext: Option<Settings>,
    ) {
        if let Ok(key) = self.banks.try_get(&event_name, &spoken, gender.as_ref()) {
            let data = self.banks.data(key);
            let emitter = Emitter::new(entity_id, emitter_name);
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(data.with(ext)) {
                        self.handles.store_static(handle, event_name, emitter);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(data.with(ext)) {
                        self.handles.store_stream(handle, event_name, emitter);
                    }
                }
            }
        }
    }

    pub fn play_on_emitter(
        &mut self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Option<Tween>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
    ) {
        if let Some(ref scene) = self.scene {
            if let Ok(key) = self.banks.try_get(&sound_name, &spoken, gender.as_ref()) {
                if let Some(ref pair) = scene
                    .emitters
                    .get(&EmitterId::new(entity_id, Some(emitter_name)))
                {
                    let data = self.banks.data(key);
                    let handle = pair.value().handle();
                    let emitter = Emitter::new(Some(entity_id), Some(emitter_name));
                    match data {
                        Either::Left(data) => {
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(handle).with(tween))
                            {
                                self.handles.store_static(handle, sound_name, emitter);
                            }
                        }
                        Either::Right(data) => {
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(handle).with(tween))
                            {
                                self.handles.store_stream(handle, sound_name, emitter);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn stop(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        tween: Option<Tween>,
    ) {
        if self.banks.exists(&event_name) {
            self.handles.stop_by(
                event_name,
                Emitter::new(entity_id, emitter_name),
                tween.unwrap_or_default(),
            );
        }
    }

    pub fn terminate(&mut self) {
        self.handles.clear();
        let _ = self.scene.take();
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

    pub fn reset(&mut self) {
        self.tracks
            .ambience
            .equalizer()
            .set_preset(eq::Preset::None);
    }

    pub fn register_emitter(
        &mut self,
        entity_id: EntityId,
        emitter_name: Option<CName>,
        emitter_settings: Option<EmitterSettings>,
    ) -> bool {
        match self.scene {
            Some(ref mut scene) => scene
                .add_emitter(entity_id, emitter_name, emitter_settings)
                .is_ok(),
            None => {
                lifecycle!("scene is not initialized");
                false
            }
        }
    }

    pub fn unregister_emitter(&mut self, entity_id: EntityId) -> bool {
        match self.scene {
            Some(ref mut scene) => scene.remove_emitter(entity_id).is_ok(),
            None => {
                lifecycle!("scene is not initialized");
                false
            }
        }
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        if let Some(ref mut scene) = self.scene {
            if let Err(e) = scene.remove_emitter(entity_id) {
                lifecycle!("failed to remove emitter: {e}");
            }
        }
    }

    pub fn any_emitter(&self) -> bool {
        match self.scene {
            Some(ref scene) => scene.any_emitter(),
            None => false,
        }
    }

    pub fn sync_scene(&mut self) {
        match self.scene {
            Some(ref mut scene) => {
                if let Err(e) = scene.sync() {
                    lifecycle!("failed to sync scene: {e}")
                }
            }
            None => lifecycle!("scene is not initialized"),
        }
    }

    pub fn is_registered_emitter(&self, entity_id: EntityId) -> bool {
        match self.scene {
            Some(ref scene) => scene.is_registered_emitter(entity_id),
            None => false,
        }
    }

    pub fn set_volume(&mut self, setting: CName, value: f64) {
        match setting.as_str() {
            "MasterVolume" => self.manager.main_track().set_volume(value, DEFAULT),
            "SfxVolume" => self.modulators.sfx_volume.update(value, DEFAULT),
            "DialogueVolume" => self.modulators.dialogue_volume.update(value, DEFAULT),
            "MusicVolume" => self.modulators.music_volume.update(value, DEFAULT),
            "CarRadioVolume" => self.modulators.car_radio_volume.update(value, DEFAULT),
            "RadioportVolume" => self.modulators.radioport_volume.update(value, DEFAULT),
            _ => lifecycle!("unknown volume setting: {}", setting.as_str()),
        }
    }

    pub fn set_reverb_mix(&mut self, value: f32) {
        self.modulators.reverb_mix.update(value, DEFAULT);
    }

    pub fn set_preset(&mut self, preset: Preset) {
        self.tracks.ambience.equalizer().set_preset(preset);
    }

    pub fn exists(sound: &CName, spoken: &SpokenLocale, gender: Option<&PlayerGender>) -> bool {
        #[cfg(not(debug_assertions))]
        return BANKS
            .get()
            .and_then(|x| x.try_get(sound, spoken, gender).is_ok())
            .unwrap_or(false);
        #[cfg(debug_assertions)]
        BANKS
            .try_read()
            .and_then(|x| x.as_ref().map(|x| x.try_get(sound, spoken, gender).is_ok()))
            .unwrap_or(false)
    }
}
