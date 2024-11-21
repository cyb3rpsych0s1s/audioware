use std::{fmt::Debug, ops::DerefMut};

use audioware_bank::{BankData, Banks, Id, Initialization, InitializationOutcome};
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
use tweens::{
    DEFAULT, DILATION_EASE_IN, DILATION_EASE_OUT, DILATION_LINEAR, IMMEDIATELY, LAST_BREATH,
};

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
    }
}

impl<B> Engine<B>
where
    B: Backend,
    <B as Backend>::Error: Debug,
{
    pub fn try_new(settings: AudioManagerSettings<B>) -> Result<Engine<B>, Error> {
        let (banks, report) = Banks::new(
            #[cfg(debug_assertions)]
            false,
        );
        #[cfg(not(debug_assertions))]
        let _ = BANKS.set(banks.clone());
        #[cfg(debug_assertions)]
        {
            *BANKS.write().deref_mut() = Some(banks.clone());
        }
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

    #[cfg(debug_assertions)]
    pub fn hot_reload(&mut self) {
        self.clear();
        let (banks, report) = Banks::new(true);
        *BANKS.write() = Some(banks.clone());
        self.banks = banks;
        self.report = report;
        self.report_initialization(true);
    }

    pub fn report_initialization(&self, #[cfg(debug_assertions)] hot_reload: bool) {
        let conjugation = if cfg!(debug_assertions) && hot_reload {
            "hot-reloaded"
        } else {
            "initialized"
        };
        let infinitive = if cfg!(debug_assertions) && hot_reload {
            "hot-reload"
        } else {
            "initialize"
        };

        match self.report.outcome() {
            InitializationOutcome::Success => {
                crate::utils::lifecycle!("{}", self.report);
                crate::utils::info(format!("Audioware {conjugation} successfully!"))
            }
            InitializationOutcome::PartialSuccess => {
                crate::utils::lifecycle!("{}", self.report);
                crate::utils::warn(format!(
                    "Audioware {conjugation} partially. See RED4ext log for more details."
                ))
            }
            InitializationOutcome::CompleteFailure => {
                crate::utils::fails!("{}", self.report);
                crate::utils::error(format!(
                    "Audioware failed to {infinitive}. See RED4ext log for more details."
                ))
            }
        };
    }

    pub fn any_handle(&self) -> bool {
        self.tracks.any_handle()
    }

    pub fn try_new_scene(&mut self) -> Result<(), Error> {
        self.scene = Some(Scene::try_new(&mut self.manager, &self.tracks)?);
        Ok(())
    }

    pub fn stop_scene_emitters(&mut self) {
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
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(tween),
                    ) {
                        self.tracks
                            .store_static(handle, event_name, entity_id, emitter_name, true);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(tween),
                    ) {
                        self.tracks
                            .store_stream(handle, event_name, entity_id, emitter_name, true);
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
            let dilatable = ext
                .as_ref()
                .and_then(|x| x.affected_by_time_dilation)
                .unwrap_or(true);
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(ext),
                    ) {
                        self.tracks.store_static(
                            handle,
                            event_name,
                            entity_id,
                            emitter_name,
                            dilatable,
                        );
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(
                        data.output_destination(key.to_output_destination(&self.tracks))
                            .with(ext),
                    ) {
                        self.tracks.store_stream(
                            handle,
                            event_name,
                            entity_id,
                            emitter_name,
                            dilatable,
                        );
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
        ext: Option<Settings>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
    ) {
        if let Some(ref mut scene) = self.scene {
            if let Ok(key) = self.banks.try_get(&sound_name, &spoken, gender.as_ref()) {
                if let Some(ref mut emitter) = scene
                    .emitters
                    .get_mut_with_name(&entity_id, &Some(emitter_name))
                {
                    let data = self.banks.data(key);
                    let persistable = emitter.persist_until_sounds_finish;
                    let dilatable = ext
                        .as_ref()
                        .and_then(|x| x.affected_by_time_dilation)
                        .unwrap_or(true);
                    match data {
                        Either::Left(data) => {
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(emitter.as_ref()).with(ext))
                            {
                                lifecycle!(
                                    "playing static sound {} on {:?}",
                                    sound_name.as_str(),
                                    entity_id
                                );
                                emitter.store_static(sound_name, handle, dilatable, persistable);
                            }
                        }
                        Either::Right(data) => {
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(emitter.as_ref()).with(ext))
                            {
                                lifecycle!(
                                    "playing stream sound {} on {:?}",
                                    sound_name.as_str(),
                                    entity_id
                                );
                                emitter.store_stream(sound_name, handle, dilatable, persistable);
                            }
                        }
                    }
                } else {
                    lifecycle!("failed to find emitter {entity_id:?} with name {emitter_name}",);
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
        self.tracks.stop_by(
            event_name,
            entity_id,
            emitter_name,
            tween.unwrap_or_default(),
        );
    }

    pub fn stop_on_emitter(
        &mut self,
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Option<Tween>,
    ) {
        if let Some(x) = self.scene.as_mut() {
            x.stop_on_emitter(
                event_name,
                entity_id,
                Some(emitter_name),
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

    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        Scene::is_registered_emitter(entity_id)
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
                scene.remove_emitter(entity_id)
            }
            None => {
                lifecycle!("scene is not initialized");
                false
            }
        }
    }

    pub fn set_listener_dilation(&mut self, value: DilationUpdate) {
        match self.scene {
            Some(ref mut scene) => {
                if scene.set_listener_dilation(&value) {
                    self.tracks.sync_dilation(scene.listener_id(), value);
                }
            }
            None => lifecycle!("scene is not initialized"),
        }
    }

    pub fn unset_listener_dilation(&mut self, value: DilationUpdate) {
        match self.scene {
            Some(ref mut scene) => {
                if scene.unset_listener_dilation(&value) {
                    self.tracks.sync_dilation(scene.listener_id(), value);
                }
            }
            None => lifecycle!("scene is not initialized"),
        }
    }

    pub fn set_emitter_dilation(&mut self, entity_id: EntityId, value: DilationUpdate) {
        match self.scene {
            Some(ref mut scene) => {
                if scene.set_emitter_dilation(entity_id, &value) {
                    self.tracks.sync_dilation(entity_id, value);
                }
            }
            None => lifecycle!("scene is not initialized"),
        }
    }

    pub fn unset_emitter_dilation(&mut self, entity_id: EntityId, value: DilationUpdate) {
        match self.scene {
            Some(ref mut scene) => {
                if scene.unset_emitter_dilation(entity_id, &value) {
                    self.tracks.sync_dilation(entity_id, value);
                }
            }
            None => lifecycle!("scene is not initialized"),
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

    pub fn clear(&mut self) {
        self.tracks.clear();
        if let Some(scene) = self.scene.as_mut() {
            scene.clear();
        }
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

#[derive(Debug, Clone)]
pub enum DilationUpdate {
    Set {
        value: f32,
        reason: CName,
        ease_in_curve: CName,
    },
    Unset {
        reason: CName,
        ease_out_curve: CName,
    },
}

impl DilationUpdate {
    pub fn dilation(&self) -> f64 {
        match self {
            Self::Set { value, .. } => *value as f64,
            Self::Unset { .. } => 1.,
        }
    }
    pub fn tween_curve(&self) -> Tween {
        if !self.has_curve() {
            DILATION_LINEAR
        } else {
            match self {
                Self::Set { .. } => DILATION_EASE_IN,
                Self::Unset { .. } => DILATION_EASE_OUT,
            }
        }
    }
}

impl DilationUpdate {
    pub fn has_curve(&self) -> bool {
        let curve = match self {
            Self::Set { ease_in_curve, .. } => ease_in_curve,
            Self::Unset { ease_out_curve, .. } => ease_out_curve,
        }
        .as_str();
        curve != "None" && !curve.is_empty()
    }
}

impl PartialEq for DilationUpdate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Set { value, reason, .. },
                Self::Set {
                    value: x,
                    reason: y,
                    ..
                },
            ) => *value == *x && *reason == *y,
            (Self::Unset { reason, .. }, Self::Unset { reason: y, .. }) => *reason == *y,
            _ => false,
        }
    }
}
