use std::{fmt::Debug, ops::DerefMut};

use audioware_bank::{BankData, Banks, Id, Initialization};
use audioware_core::With;
use audioware_manifest::{PlayerGender, Settings, Source, SpokenLocale};
use either::Either;
use eq::{EqPass, Preset};
use kira::{
    manager::{backend::Backend, AudioManager, AudioManagerSettings},
    spatial::emitter::EmitterSettings,
    tween::Tween,
    OutputDestination,
};
use modulators::{Modulators, Parameter};
use red4ext_rs::types::{CName, EntityId};
use scene::Scene;
use tracks::Tracks;
use tweens::{DEFAULT, IMMEDIATELY, LAST_BREATH};

use crate::{
    error::{EngineError, Error},
    utils::lifecycle,
};

pub mod queue;

pub mod eq;
mod modulators;
mod scene;
mod tracks;
mod tweens;

#[cfg(not(debug_assertions))]
static BANKS: std::sync::OnceLock<Banks> = std::sync::OnceLock::new();
#[cfg(debug_assertions)]
static BANKS: parking_lot::RwLock<Option<Banks>> = parking_lot::RwLock::new(None);

pub struct Engine<B: Backend> {
    pub scene: Option<Scene>,
    pub tracks: Tracks,
    pub modulators: Modulators,
    pub manager: AudioManager<B>,
    pub report: Initialization,
    pub banks: Banks,
}

#[cfg(debug_assertions)]
impl<B: Backend> Drop for Engine<B> {
    fn drop(&mut self) {
        lifecycle!("drop engine");
        // bug in kira DecodeScheduler NextStep::Wait
        // self.handles.stop_all(IMMEDIATELY);
        // self.handles.clear();
    }
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
            scene: None,
            modulators,
            tracks,
            report,
        })
    }

    pub fn any_handle(&self) -> bool {
        self.tracks.any_handle()
    }

    pub fn try_new_scene(&mut self) -> Result<(), Error> {
        self.scene = Some(Scene::try_new(&mut self.manager, &self.tracks)?);
        Ok(())
    }

    pub fn clear_scene(&mut self) {
        if let Some(mut scene) = self.scene.take() {
            scene.stop_emitters(IMMEDIATELY);
        }
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
            // let emitter = Emitter::new(entity_id, emitter_name);
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(tween),
                    ) {
                        self.tracks
                            .store_static(handle, event_name, entity_id, emitter_name);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(tween),
                    ) {
                        self.tracks
                            .store_stream(handle, event_name, entity_id, emitter_name);
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
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(ext),
                    ) {
                        self.tracks
                            .store_static(handle, event_name, entity_id, emitter_name);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(ext),
                    ) {
                        self.tracks
                            .store_stream(handle, event_name, entity_id, emitter_name);
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
                if let Some(ref mut emitter) =
                    scene.emitters.get_mut(&(entity_id, Some(emitter_name)))
                {
                    let data = self.banks.data(key);
                    match data {
                        Either::Left(data) => {
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(emitter.as_ref()).with(tween))
                            {
                                lifecycle!(
                                    "playing static sound {} on {:?}",
                                    sound_name.as_str(),
                                    entity_id
                                );
                                emitter.store_static(handle);
                            }
                        }
                        Either::Right(data) => {
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(emitter.as_ref()).with(tween))
                            {
                                lifecycle!(
                                    "playing stream sound {} on {:?}",
                                    sound_name.as_str(),
                                    entity_id
                                );
                                emitter.store_stream(handle);
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
            self.tracks.stop_by(
                event_name,
                entity_id,
                emitter_name,
                tween.unwrap_or_default(),
            );
        }
    }

    pub fn pause(&mut self) {
        self.tracks.pause(Default::default());
        if let Some(x) = self.scene.as_mut() {
            x.pause(Default::default())
        }
    }

    pub fn resume(&mut self) {
        self.tracks.resume(Default::default());
        if let Some(x) = self.scene.as_mut() {
            x.resume(Default::default())
        }
    }

    pub fn reclaim(&mut self) {
        self.tracks.reclaim();
        if let Some(x) = self.scene.as_mut() {
            x.reclaim()
        }
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
            Some(ref mut scene) => {
                scene.on_emitter_dies(entity_id);
                scene.remove_emitter(entity_id).is_ok()
            }
            None => {
                lifecycle!("scene is not initialized");
                false
            }
        }
    }

    pub fn on_emitter_incapacitated(&mut self, entity_id: EntityId) {
        if let Some(x) = self.scene.as_mut() {
            x.stop_for(entity_id, LAST_BREATH)
        }
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        if let Some(x) = self.scene.as_mut() {
            x.on_emitter_dies(entity_id)
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

pub trait ToOutputDestination {
    fn to_output_destination(&self, tracks: &Tracks) -> OutputDestination;
}

impl ToOutputDestination for Id {
    fn to_output_destination(&self, tracks: &Tracks) -> OutputDestination {
        match self {
            Id::OnDemand(_, source) | Id::InMemory(_, source) => match source {
                Source::Sfx | Source::Ono => tracks.sfx.as_ref().into(),
                Source::Voices => tracks.dialogue.as_ref().into(),
                Source::Playlist => tracks.radioport.as_ref().into(),
                Source::Music => tracks.music.as_ref().into(),
                Source::Jingle => tracks.car_radio.as_ref().into(),
            },
        }
    }
}
