use std::fmt::Debug;

use audioware_bank::{BankData, Banks, Id, Initialization, InitializationOutcome};
use audioware_core::With;
use audioware_manifest::{Locale, ScnDialogLineType, Settings, Source};
use either::Either;
use eq::{EqPass, Preset};
use kira::{
    manager::{backend::Backend, AudioManager, AudioManagerSettings},
    sound::{static_sound::StaticSoundData, streaming::StreamingSoundData, FromFileError},
    spatial::emitter::EmitterSettings,
    tween::Tween,
    OutputDestination,
};
use modulators::{Modulators, Parameter};
use red4ext_rs::types::{CName, EntityId, GameInstance, Opt};
use scene::Scene;
use state::{SpokenLocale, ToGender};
use tracks::Tracks;
use tweens::{
    DEFAULT, DILATION_EASE_IN, DILATION_EASE_OUT, DILATION_LINEAR, IMMEDIATELY, LAST_BREATH,
};

use crate::{
    error::{EngineError, Error},
    propagate_subtitles,
    utils::{fails, lifecycle, success, warns},
    AsAudioSystem, AsGameInstance,
};

pub mod eq;
pub mod queue;
pub mod state;

mod modulators;
mod scene;
mod tracks;
mod tweens;

#[cfg(not(feature = "hot-reload"))]
static BANKS: std::sync::OnceLock<Banks> = std::sync::OnceLock::new();
#[cfg(feature = "hot-reload")]
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
        let (banks, report) = Banks::new();
        #[cfg(not(feature = "hot-reload"))]
        let _ = BANKS.set(banks.clone());
        #[cfg(feature = "hot-reload")]
        {
            *BANKS.write() = Some(banks.clone());
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

    #[cfg(feature = "hot-reload")]
    pub fn hot_reload(&mut self) {
        self.clear();
        self.report = self.banks.hot_reload();
        *BANKS.write() = Some(self.banks.clone());
        self.report_initialization(true);
    }

    pub fn report_initialization(&self, hot_reload: bool) {
        let conjugation = if cfg!(feature = "hot-reload") && hot_reload {
            "hot-reloaded"
        } else {
            "initialized"
        };
        let infinitive = if cfg!(feature = "hot-reload") && hot_reload {
            "hot-reload"
        } else {
            "initialize"
        };

        match self.report.outcome() {
            InitializationOutcome::Success => {
                success!(["{}", self.report];["Audioware {} successfully!", conjugation]);
            }
            InitializationOutcome::PartialSuccess => {
                warns!(["{}", self.report];["Audioware partially failed to {infinitive}. See RED4ext log for more details."]);
            }
            InitializationOutcome::CompleteFailure => {
                fails!(["{}", self.report];["Audioware failed to {infinitive}. See RED4ext log for more details."]);
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

    pub fn play_over_the_phone(
        &mut self,
        event_name: CName,
        emitter_name: CName,
        gender: audioware_manifest::PlayerGender,
    ) {
        let spoken = SpokenLocale::get();
        if let Ok(key) = self.banks.try_get(&event_name, &spoken, Some(&gender)) {
            let data = self.banks.data(key);
            let destination = &self.tracks.holocall;
            let dilatable = true;
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(data.output_destination(destination)) {
                        self.tracks.store_static(
                            handle,
                            event_name,
                            None,
                            Some(emitter_name),
                            dilatable,
                        );
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(data.output_destination(destination)) {
                        self.tracks.store_stream(
                            handle,
                            event_name,
                            None,
                            Some(emitter_name),
                            dilatable,
                        );
                    }
                }
            }
        }
    }

    pub fn play<T>(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        ext: Option<T>,
        line_type: Option<ScnDialogLineType>,
    ) where
        StaticSoundData: With<Option<T>>,
        StreamingSoundData<FromFileError>: With<Option<T>>,
        T: AffectedByTimeDilation,
    {
        let spoken = SpokenLocale::get();
        let gender = entity_id.as_ref().and_then(ToGender::to_gender);
        if let Ok(key) = self.banks.try_get(&event_name, &spoken, gender.as_ref()) {
            let data = self.banks.data(key);
            let duration: f32;
            let dilatable = ext
                .as_ref()
                .map(AffectedByTimeDilation::affected_by_time_dilation)
                .unwrap_or(true);
            let is_v = self
                .scene
                .as_ref()
                .is_some_and(|x| Some(x.listener_id()) == entity_id);
            let destination: OutputDestination = if is_v {
                if key.is_vocal() {
                    self.tracks.v.vocal.id().into()
                } else {
                    self.tracks.v.emissive.id().into()
                }
            } else {
                key.to_output_destination(&self.tracks)
            };
            match data {
                Either::Left(data) => {
                    duration = data.duration().as_secs_f32();
                    if let Ok(handle) = self
                        .manager
                        .play(data.output_destination(destination).with(ext))
                    {
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
                    duration = data.duration().as_secs_f32();
                    if let Ok(handle) = self
                        .manager
                        .play(data.output_destination(destination).with(ext))
                    {
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
            if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
                propagate_subtitles(
                    event_name,
                    entity_id,
                    emitter_name,
                    line_type.unwrap_or_default(),
                    duration,
                )
            }
        }
    }

    pub fn play_on_emitter<T>(
        &mut self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        ext: Option<T>,
    ) where
        StaticSoundData: With<Option<T>>,
        StreamingSoundData<FromFileError>: With<Option<T>>,
        T: AffectedByTimeDilation,
    {
        let gender = entity_id.to_gender();
        let spoken = SpokenLocale::get();
        if let Some(ref mut scene) = self.scene {
            if let Ok(key) = self.banks.try_get(&sound_name, &spoken, gender.as_ref()) {
                if let Some(ref mut emitter) = scene
                    .emitters
                    .get_mut_by_name(&entity_id, &Some(emitter_name))
                {
                    let duration: f32;
                    let data = self.banks.data(key);
                    let dilatable = ext
                        .as_ref()
                        .map(AffectedByTimeDilation::affected_by_time_dilation)
                        .unwrap_or(true);
                    match data {
                        Either::Left(data) => {
                            duration = data.duration().as_secs_f32();
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(emitter.as_ref()).with(ext))
                            {
                                lifecycle!(
                                    "playing static sound {} on {:?}",
                                    sound_name.as_str(),
                                    entity_id
                                );
                                emitter.store_static(sound_name, handle, dilatable);
                            }
                        }
                        Either::Right(data) => {
                            duration = data.duration().as_secs_f32();
                            if let Ok(handle) = self
                                .manager
                                .play(data.output_destination(emitter.as_ref()).with(ext))
                            {
                                lifecycle!(
                                    "playing stream sound {} on {:?}",
                                    sound_name.as_str(),
                                    entity_id
                                );
                                emitter.store_stream(sound_name, handle, dilatable);
                            }
                        }
                    }
                    propagate_subtitles(
                        sound_name,
                        entity_id,
                        emitter_name,
                        ScnDialogLineType::default(),
                        duration,
                    );
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

    pub fn switch<T>(
        &mut self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        switch_name_tween: Option<Tween>,
        switch_value_settings: Option<T>,
    ) where
        StaticSoundData: With<Option<T>>,
        StreamingSoundData<FromFileError>: With<Option<T>>,
        T: AffectedByTimeDilation,
    {
        if Self::exists(&switch_name) {
            self.tracks.stop_by(
                switch_name,
                entity_id,
                emitter_name,
                switch_name_tween.unwrap_or(IMMEDIATELY),
            );
        } else {
            GameInstance::get_audio_system().stop(
                switch_name,
                entity_id.map(Opt::from).unwrap_or(Opt::Default),
                emitter_name.map(Opt::from).unwrap_or(Opt::Default),
            );
        }
        if Self::exists(&switch_value) {
            self.play(
                switch_value,
                entity_id,
                emitter_name,
                switch_value_settings,
                None,
            );
        } else {
            GameInstance::get_audio_system().play(
                switch_value,
                entity_id.map(Opt::from).unwrap_or(Opt::Default),
                emitter_name.map(Opt::from).unwrap_or(Opt::Default),
            );
        }
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
        self.set_reverb_mix(0.);
        self.set_preset(eq::Preset::None);
    }

    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        Scene::is_registered_emitter(entity_id)
    }

    pub fn emitters_count() -> i32 {
        Scene::emitters_count()
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
            x.on_emitter_incapacitated(entity_id, LAST_BREATH)
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

    pub fn exists(sound: &CName) -> bool {
        #[cfg(not(feature = "hot-reload"))]
        return BANKS.get().map(|x| x.exists(sound)).unwrap_or(false);
        #[cfg(feature = "hot-reload")]
        BANKS
            .try_read()
            .and_then(|x| x.as_ref().map(|x| x.exists(sound)))
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.tracks.clear();
        if let Some(scene) = self.scene.as_mut() {
            scene.clear();
        }
    }

    pub fn duration(
        event_name: CName,
        locale: Locale,
        gender: audioware_manifest::PlayerGender,
        total: bool,
    ) -> f32 {
        if let Some(banks) = Self::banks().as_ref() {
            return banks.duration(&event_name, locale, gender, total);
        }
        -1.
    }

    pub fn supported_languages() -> Vec<CName> {
        if let Some(banks) = Self::banks().as_ref() {
            if cfg!(not(test)) {
                return banks.languages().iter().cloned().map(CName::from).collect();
            }
        }
        vec![]
    }

    #[cfg(not(feature = "hot-reload"))]
    pub fn banks<'a>() -> Option<&'a Banks> {
        BANKS.get()
    }
    #[cfg(feature = "hot-reload")]
    pub fn banks<'a>(
    ) -> parking_lot::lock_api::RwLockReadGuard<'a, parking_lot::RawRwLock, Option<Banks>> {
        BANKS.read()
    }
}

pub trait ToOutputDestination {
    fn to_output_destination(&self, tracks: &Tracks) -> OutputDestination;
}

impl ToOutputDestination for Id {
    #[inline(always)]
    fn to_output_destination(&self, tracks: &Tracks) -> OutputDestination {
        match self {
            Id::OnDemand(_, source) | Id::InMemory(_, source) => match source {
                Source::Sfx | Source::Ono => tracks.sfx.as_ref(),
                Source::Voices => tracks.dialogue.as_ref(),
                Source::Playlist => tracks.radioport.as_ref(),
                Source::Music => tracks.music.as_ref(),
                Source::Jingle => tracks.car_radio.as_ref(),
            }
            .id()
            .into(),
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

pub trait AffectedByTimeDilation {
    fn affected_by_time_dilation(&self) -> bool;
}

impl AffectedByTimeDilation for Settings {
    #[inline(always)]
    fn affected_by_time_dilation(&self) -> bool {
        self.affected_by_time_dilation.unwrap_or(true)
    }
}

impl AffectedByTimeDilation for Tween {
    #[inline(always)]
    fn affected_by_time_dilation(&self) -> bool {
        true
    }
}
