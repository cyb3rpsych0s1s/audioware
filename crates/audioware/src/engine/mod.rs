use std::{
    fmt::Debug,
    num::NonZero,
    ops::{Div, Not},
    time::Duration,
};

use audioware_bank::{
    BankData, BankSettings, Banks, Id, Initialization, InitializationOutcome, SceneId, TryGet,
    error::registry::ErrorDisplay,
};
use audioware_core::{Amplitude, SceneDialogSettings, SpatialTrackSettings, With};
use audioware_manifest::{Locale, ScnDialogLineType, Source, ValidateFor};
use either::Either;
use eq::{EqPass, Preset};
use kira::{
    AudioManager, AudioManagerSettings, Decibels, Easing, StartTime, Tween,
    backend::Backend,
    sound::{FromFileError, static_sound::StaticSoundData, streaming::StreamingSoundData},
    track::TrackHandle,
};
use modulators::{Modulators, Parameter};
use red4ext_rs::types::{CName, Cruid, EntityId, GameInstance, Opt};
pub use scene::{AffectedByTimeDilation, DilationUpdate, Scene};
use state::{SpokenLocale, ToGender};
use tracks::Tracks;
use tweens::{DEFAULT, IMMEDIATELY, LAST_BREATH};

use crate::{
    AsAudioSystem, AsGameInstance, AsGameObjectExt, GameObject,
    engine::{
        tracks::TrackEntryOptions,
        traits::{Handle, stop::StopBy, store::Store},
    },
    error::{EngineError, Error},
    propagate_subtitles,
    utils::{fails, lifecycle, success, warns},
};

pub use scene::ToDistances;

pub mod eq;
pub mod queue;
pub mod state;
pub mod traits;

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
    pub last_volume: Option<Decibels>,
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
            last_volume: None,
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
        self.scene = Some(Scene::try_new(&mut self.manager)?);
        Ok(())
    }

    pub fn stop_scene_emitters_and_actors(&mut self) {
        if let Some(mut scene) = self.scene.take() {
            scene.stop_emitters_and_actors(IMMEDIATELY);
        }
    }

    pub fn play_over_the_phone(
        &mut self,
        event_name: CName,
        emitter_name: CName,
        gender: audioware_manifest::PlayerGender,
    ) {
        let spoken = SpokenLocale::get();
        match self.banks.ids.try_get(&event_name, &spoken, Some(&gender)) {
            Ok(key) => {
                let data = self.banks.data(key);
                let destination = &mut self.tracks.holocall;
                let dilatable = true;
                let duration: f32;
                match data {
                    Either::Left(data) => {
                        duration = data.duration().as_secs_f32();
                        if let Ok(handle) = destination.play(data) {
                            self.tracks.handles.statics.store(Handle::new(
                                event_name,
                                handle,
                                TrackEntryOptions {
                                    entity_id: None,
                                    emitter_name: Some(emitter_name),
                                    affected_by_time_dilation: dilatable,
                                },
                            ));
                        }
                    }
                    Either::Right(data) => {
                        duration = data.duration().as_secs_f32();
                        if let Ok(handle) = destination.play(data) {
                            self.tracks.handles.streams.store(Handle::new(
                                event_name,
                                handle,
                                TrackEntryOptions {
                                    entity_id: None,
                                    emitter_name: Some(emitter_name),
                                    affected_by_time_dilation: dilatable,
                                },
                            ));
                        }
                    }
                };
                if !emitter_name.as_str().is_empty() && emitter_name.as_str() != "None" {
                    propagate_subtitles(
                        event_name,
                        EntityId::default(),
                        emitter_name,
                        ScnDialogLineType::Holocall,
                        duration,
                    );
                } else if *key.source() == Source::Voices {
                    warns!(
                        "cannot propagate subtitles for voice, emitterName must be defined: {event_name}"
                    );
                }
            }
            Err(e) => {
                warns!("cannot play over the phone: {e}");
            }
        }
    }

    pub fn play_scene_dialog(
        &mut self,
        string_id: Cruid,
        entity_id: EntityId,
        is_player: bool,
        is_holocall: bool,
        is_rewind: bool,
        seek_time: f32,
    ) {
        if !string_id.is_defined() {
            warns!(
                "cannot play sound with undefined RUID: {}",
                string_id.error_display()
            );
            return;
        }
        if !entity_id.is_defined() {
            warns!("cannot play sound on undefined entity: {}", entity_id);
            return;
        }
        let Some(ref mut scene) = self.scene else {
            warns!("scene is not available, and therefore dialogs aren't either");
            return;
        };
        let gender = entity_id.to_gender();
        let spoken = SpokenLocale::get();
        let Ok(key) = self
            .banks
            .scene_ids
            .try_get(&string_id, &spoken, gender.as_ref())
        else {
            warns!("couldn't find RUID in bank: {}", i64::from(string_id));
            return;
        };
        let key = key.clone();
        let scene_settings = SceneDialogSettings {
            is_rewind,
            seek_time,
        };
        if is_player {
            let data = self.banks.data(&key);
            let destination: &mut TrackHandle = &mut self.tracks.v.vocal;
            match data.with(scene_settings) {
                Either::Left(data) => {
                    if let Ok(handle) = destination.play(data) {
                        scene.actors.v.store(Handle::new(string_id, handle, ()));
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = destination.play(data) {
                        scene.actors.v.store(Handle::new(string_id, handle, ()));
                    }
                }
            }
        } else if is_holocall {
            let data = self.banks.data(&key);
            let destination: &mut TrackHandle = &mut self.tracks.holocall;
            match data.with(scene_settings) {
                Either::Left(data) => {
                    if let Ok(handle) = destination.play(data) {
                        scene
                            .actors
                            .holocall
                            .store(Handle::new(string_id, handle, ()));
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = destination.play(data) {
                        scene
                            .actors
                            .holocall
                            .store(Handle::new(string_id, handle, ()));
                    }
                }
            }
        } else {
            self.play_on_actor(string_id, entity_id, &key, scene_settings);
        }
        // red engine handles subtitles automatically
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
        T: AffectedByTimeDilation
            + ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>>,
    {
        let spoken = SpokenLocale::get();
        let gender = entity_id.as_ref().and_then(ToGender::to_gender);
        match self
            .banks
            .ids
            .try_get(&event_name, &spoken, gender.as_ref())
        {
            Ok(key) => {
                let data = self.banks.data(key);
                if let Some(Err(e)) = ext.as_ref().map(|x| x.validate_for(&data)) {
                    warns!("invalid setting(s) for audio: {e:#?}");
                    return;
                }
                let duration: f32;
                let dilatable = ext
                    .as_ref()
                    .map(AffectedByTimeDilation::affected_by_time_dilation)
                    .unwrap_or(true);
                let is_v = self
                    .scene
                    .as_ref()
                    .is_some_and(|x| Some(x.listener_id()) == entity_id);
                let destination: &mut TrackHandle = if is_v {
                    if key.is_vocal() {
                        &mut self.tracks.v.vocal
                    } else {
                        &mut self.tracks.v.emissive
                    }
                } else {
                    key.to_output_destination(&mut self.tracks)
                };
                match data {
                    Either::Left(data) => {
                        duration = data.duration().as_secs_f32();
                        if let Ok(handle) = destination.play(data.with(ext)) {
                            self.tracks.handles.statics.store(Handle::new(
                                event_name,
                                handle,
                                TrackEntryOptions {
                                    entity_id,
                                    emitter_name,
                                    affected_by_time_dilation: dilatable,
                                },
                            ));
                        }
                    }
                    Either::Right(data) => {
                        duration = data.duration().as_secs_f32();
                        if let Ok(handle) = destination.play(data.with(ext)) {
                            self.tracks.handles.streams.store(Handle::new(
                                event_name,
                                handle,
                                TrackEntryOptions {
                                    entity_id,
                                    emitter_name,
                                    affected_by_time_dilation: dilatable,
                                },
                            ));
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
                } else if *key.source() == Source::Voices {
                    warns!(
                        "cannot propagate subtitles for voice, both entityID and emitterName must be defined: {event_name}"
                    );
                }
            }
            Err(e) => {
                warns!("cannot play sound: {e}");
            }
        }
    }

    pub fn play_on_emitter<T>(
        &mut self,
        sound_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        ext: Option<T>,
    ) where
        StaticSoundData: With<Option<T>>,
        StreamingSoundData<FromFileError>: With<Option<T>>,
        T: AffectedByTimeDilation
            + ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>>,
    {
        if !entity_id.is_defined() {
            warns!("cannot play sound on undefined entity: {sound_name}");
            return;
        }
        let gender = entity_id.to_gender();
        let spoken = SpokenLocale::get();
        if let Some(ref mut scene) = self.scene {
            match self
                .banks
                .ids
                .try_get(&sound_name, &spoken, gender.as_ref())
            {
                Ok(key) => {
                    match scene.emitters.play_on_emitter(
                        key,
                        &self.banks,
                        sound_name,
                        entity_id,
                        tag_name,
                        ext,
                    ) {
                        Err(e) => {
                            warns!("cannot play sound on emitter: {e}");
                        }
                        Ok((duration, emitter_name)) => {
                            let emitter_name = match emitter_name {
                                Some(emitter_name) => Some(emitter_name.as_str().to_string()),
                                None => {
                                    let go = GameInstance::find_entity_by_id(
                                        GameInstance::new(),
                                        entity_id,
                                    )
                                    .cast::<GameObject>();

                                    go.and_then(|x| {
                                        x.is_null().not().then(|| x.resolve_display_name())
                                    })
                                }
                            };
                            if let Some(emitter_name) = emitter_name {
                                propagate_subtitles(
                                    sound_name,
                                    entity_id,
                                    CName::new(emitter_name.as_str()),
                                    ScnDialogLineType::default(),
                                    duration,
                                );
                            } else if *key.source() == Source::Voices {
                                warns!(
                                    "cannot propagate subtitles for voice, couldn't resolve emitter name: {sound_name} [{entity_id}]"
                                );
                            }
                        }
                    };
                }
                Err(e) => {
                    warns!("cannot play sound: {e}");
                }
            }
        }
    }

    pub fn play_on_actor(
        &mut self,
        sound_name: Cruid,
        entity_id: EntityId,
        key: &SceneId,
        scene_settings: SceneDialogSettings,
    ) {
        if let Some(ref mut scene) = self.scene {
            if !scene.exists_actor(&entity_id)
                && let Err(e) = scene.add_actor(&mut self.manager, entity_id, &self.tracks.ambience)
            {
                warns!("could not add actor {entity_id}: {e}");
                return;
            }
            let data = self.banks.data(key);
            let ext = self.banks.settings(key);
            if let Some(Err(e)) = ext.as_ref().map(|x| x.validate_for(&data)) {
                warns!("invalid setting(s) for actor audio: {e:#?}");
                return;
            }
            let mut slot = scene
                .actors
                .emitters
                .get_mut(&entity_id)
                .expect("actor should automatically have been added if missing");
            match data.with(scene_settings) {
                Either::Left(data) => {
                    if let Ok(handle) = slot.value_mut().track_mut().play(data) {
                        slot.handles
                            .statics
                            .store(Handle::new(sound_name, handle, ()));
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = slot.value_mut().track_mut().play(data) {
                        slot.handles
                            .streams
                            .store(Handle::new(sound_name, handle, ()));
                    }
                }
            }
        }
    }

    pub fn stop_on_actors(&mut self, sound_name: Cruid, fade_out: f32) {
        if !sound_name.is_defined() {
            warns!(
                "cannot stop sound with undefined RUID: {}",
                sound_name.error_display()
            );
            return;
        }
        if let Some(ref mut scene) = self.scene {
            let tween = Tween {
                start_time: StartTime::Immediate,
                duration: Duration::from_secs_f32(fade_out),
                easing: Easing::Linear,
            };
            scene.actors.v.stop_by(&sound_name, tween);
            scene.actors.holocall.stop_by(&sound_name, tween);
            scene
                .actors
                .emitters
                .iter_mut()
                .for_each(|mut x| x.value_mut().stop_by(&sound_name, tween));
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
        T: AffectedByTimeDilation
            + ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>>,
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
        tag_name: CName,
        tween: Option<Tween>,
    ) {
        if let Some(x) = self.scene.as_mut() {
            x.stop_on_emitter(event_name, entity_id, tag_name, tween.unwrap_or_default());
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

    pub fn is_registered_emitter(entity_id: EntityId, tag_name: Option<CName>) -> bool {
        Scene::is_registered_emitter(entity_id, tag_name)
    }

    pub fn emitters_count() -> i32 {
        Scene::emitters_count()
    }

    pub fn register_emitter(
        &mut self,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Option<CName>,
        emitter_settings: Option<&(SpatialTrackSettings, NonZero<u64>)>,
    ) -> bool {
        match self.scene {
            Some(ref mut scene) => scene
                .add_emitter(
                    &mut self.manager,
                    entity_id,
                    tag_name,
                    emitter_name,
                    emitter_settings,
                    &self.tracks.ambience,
                )
                .inspect_err(|e| warns!("failed to register emitter: {e}"))
                .is_ok(),
            None => {
                lifecycle!("scene is not initialized");
                false
            }
        }
    }

    pub fn unregister_emitter(&mut self, entity_id: EntityId, tag_name: CName) -> bool {
        match self.scene {
            Some(ref mut scene) => scene.unregister_emitter(&entity_id, &tag_name),
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

    pub fn any_actor(&self) -> bool {
        match self.scene {
            Some(ref scene) => scene.any_actor(),
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

    pub fn mute(&mut self, value: bool) {
        if value {
            self.manager
                .main_track()
                .set_volume(Decibels::SILENCE, DEFAULT);
        } else if let Some(volume) = self.last_volume {
            self.manager.main_track().set_volume(volume, DEFAULT);
        }
    }

    pub fn set_volume(&mut self, setting: CName, value: Amplitude) {
        lifecycle!("about to change {value} for {setting}");
        // kira uses amplitude for volume, default to 1.
        // while default volume expressed as game setting is 100.
        let v = value.div(100.).clamp(0., 1.) as f64;
        let d = crate::engine::modulators::VOLUME_MAPPING.map(v);
        match setting.as_str() {
            "MasterVolume" => {
                self.manager.main_track().set_volume(d, DEFAULT);
                self.last_volume = Some(d);
            }
            "SfxVolume" => self.tracks.sfx.set_volume(d, DEFAULT),
            "DialogueVolume" => self.tracks.dialogue.set_volume(d, DEFAULT),
            "MusicVolume" => self.tracks.music.set_volume(d, DEFAULT),
            "CarRadioVolume" => self.tracks.car_radio.set_volume(d, DEFAULT),
            "RadioportVolume" => self.tracks.radioport.set_volume(d, DEFAULT),
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

    pub fn exists_for_scene(cruid: &Cruid) -> bool {
        #[cfg(not(feature = "hot-reload"))]
        return BANKS
            .get()
            .map(|x| x.exists_for_scene(cruid))
            .unwrap_or(false);
        #[cfg(feature = "hot-reload")]
        BANKS
            .try_read()
            .and_then(|x| x.as_ref().map(|x| x.exists_for_scene(cruid)))
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
        if let Some(banks) = Self::banks().as_ref()
            && cfg!(not(test))
        {
            return banks.languages().iter().cloned().map(CName::from).collect();
        }
        vec![]
    }

    #[cfg(not(feature = "hot-reload"))]
    pub fn banks<'a>() -> Option<&'a Banks> {
        BANKS.get()
    }
    #[cfg(feature = "hot-reload")]
    pub fn banks<'a>()
    -> parking_lot::lock_api::RwLockReadGuard<'a, parking_lot::RawRwLock, Option<Banks>> {
        BANKS.read()
    }
}

pub trait ToOutputDestination {
    fn to_output_destination<'b>(&self, tracks: &'b mut Tracks) -> &'b mut TrackHandle;
}

impl ToOutputDestination for Id {
    #[inline(always)]
    fn to_output_destination<'b>(&self, tracks: &'b mut Tracks) -> &'b mut TrackHandle {
        match self {
            Id::OnDemand(_, source) | Id::InMemory(_, source) => match source {
                Source::Sfx | Source::Ono => &mut tracks.sfx,
                Source::Voices => &mut tracks.dialogue,
                Source::Playlist => &mut tracks.radioport,
                Source::Music => &mut tracks.music,
                Source::Jingle => &mut tracks.car_radio,
            },
        }
    }
}

impl ToOutputDestination for SceneId {
    #[inline(always)]
    fn to_output_destination<'b>(&self, tracks: &'b mut Tracks) -> &'b mut TrackHandle {
        &mut tracks.dialogue
    }
}
