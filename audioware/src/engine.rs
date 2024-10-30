//! Audio engine.

use std::{
    sync::{Mutex, MutexGuard, OnceLock, RwLock},
    thread::JoinHandle,
};

use audioware_bank::{BankSubtitles, Banks, Id};
use audioware_manifest::{PlayerGender, ScnDialogLineType, Source, SpokenLocale, WrittenLocale};
use commands::{Command, CommandOps, Lifecycle, OuterCommand, OuterCommandOps};
use crossbeam::{
    channel::{Receiver, Sender, TrySendError},
    select,
};
use kira::{manager::AudioManager, OutputDestination, Volume};
use manager::{Pause, PlayAndStore, Resume, StopBy, StopFor};
use modulators::{
    CarRadioVolume, DialogueVolume, MusicVolume, Parameter, RadioportVolume, ReverbMix, SfxVolume,
};
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance, Opt, Ref},
    PluginOps,
};

use crate::{
    error::Error,
    macros::{ok_or_return, some_or_return},
    states::{GameState, State},
    types::{
        propagate_subtitles, AsAudioSystem, AsGameInstance, AsGameObject, EmitterSettings,
        GameObject, LocalizationPackage, Subtitle, ToTween, Tween,
    },
    utils, Audioware,
};

pub mod commands;
mod effects;
mod eq;
mod id;
mod manager;
mod modulators;
mod scene;
mod settings;
mod tracks;

pub use effects::IMMEDIATELY;
pub use eq::EqPass;
pub use eq::Preset;
pub use manager::Manager;
pub use scene::Scene;
pub use settings::*;
pub use tracks::Tracks;

static BACKGROUND: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();

/// Use to enqueue [sound commands][Command].
static COMMANDS: OnceLock<RwLock<Option<Sender<OuterCommand>>>> = OnceLock::new();

/// Use to enqueue [lifecycle updates][Lifecycle] internally.
static UPDATES: OnceLock<RwLock<Option<Sender<Lifecycle>>>> = OnceLock::new();

/// Execute queued [sound commands][Command].
fn handle_receive(rc: Receiver<OuterCommand>, rl: Receiver<Lifecycle>) {
    'game: loop {
        select! {
            recv(rc) -> msg => match msg {
                Ok(command) => command.into_inner().execute(),
                Err(_) => {
                    crate::utils::lifecycle!("sound commands disconnected");
                    break 'game;
                }
            },
            recv(rl) -> msg => match msg {
                Ok(Lifecycle::Terminate) => {
                    Lifecycle::Terminate.execute();
                    break 'game;
                }
                Ok(lifecycle) => lifecycle.execute(),
                Err(_) => {
                    crate::utils::lifecycle!("lifecycle updates disconnected");
                    break 'game;
                }
            },
            default => {},
        }
    }
}

/// Audio engine built on top of [kira].
pub struct Engine;

/// Context in which sound will be played in [Engine],
/// mostly a convenience method for:
/// - [Manager] mutex guard
/// - [spoken locale][SpokenLocale]
/// - [written locale][WrittenLocale]
/// - optionally V's [gender][PlayerGender]
pub type EngineContext<'a> = (
    MutexGuard<'a, AudioManager>,
    SpokenLocale,
    WrittenLocale,
    Option<PlayerGender>,
    &'a Id,
);

impl Engine {
    /// Engine setup for [Manager], [Tracks] and [Scene].
    pub(crate) fn setup() -> Result<(), Error> {
        let mut manager = Manager::try_lock()?;
        Tracks::setup(&mut manager)?;
        Scene::setup(&mut manager, Tracks::get())?;
        Ok(())
    }
    /// Define [LocalizationPackage] subtitles from [Manifest][audioware_manifest::Manifest]s.
    pub fn define_subtitles(package: Ref<LocalizationPackage>) {
        let written = WrittenLocale::get();
        let subtitles = Banks.subtitles(written);
        for (key, (value_f, value_m)) in subtitles.iter() {
            package.subtitle(key.as_str(), value_f.as_str(), value_m.as_str());
        }
    }
    /// Engine supported languages.
    pub fn supported_languages() -> Vec<CName> {
        Banks::languages().into_iter().map(|x| x.into()).collect()
    }
    /// Shutdown engine.
    pub(crate) fn shutdown() {
        if let Err(e) = Manager.clear_tracks(None) {
            log::error!(Audioware::env(), "couldn't clear tracks on manager: {e}");
        }
        if let Err(e) = Scene::clear_emitters() {
            log::error!(Audioware::env(), "couldn't clear emitters in scene: {e}");
        }
    }
    /// Terminate engine.
    pub(crate) fn terminate() {
        GameState::set(GameState::Unload);
        Self::shutdown();
        let _ = COMMANDS
            .get()
            .expect("should have been initialized")
            .try_write()
            .ok()
            .and_then(|mut x| x.take());
        let _ = UPDATES
            .get()
            .expect("should have been initialized")
            .try_write()
            .ok()
            .and_then(|mut x| x.take());
        if let Ok(mut x) = BACKGROUND
            .get()
            .expect("should have been initialized")
            .try_lock()
        {
            if let Some(x) = x.take() {
                if let Err(e) = x.join() {
                    log::error!(Audioware::env(), "unable to join thread {e:?}");
                }
            }
        }
    }
    /// Notify lifecycle updates.
    pub fn notify(update: Lifecycle) {
        if let Ok(Some(x)) = UPDATES
            .get()
            .expect("should have been initialized")
            .try_read()
            .as_deref()
        {
            if let Err(e) = x.try_send(update) {
                match e {
                    TrySendError::Full(lifecycle) => {
                        log::warn!(
                            Audioware::env(),
                            "couldn't send lifecycle update, channel full: {:#?}",
                            lifecycle
                        );
                    }
                    TrySendError::Disconnected(lifecycle) => {
                        log::warn!(
                            Audioware::env(),
                            "error sending lifecycle update, channel disconnected: {:#?}",
                            lifecycle
                        );
                    }
                }
            }
        }
    }
    /// Send sound command (cancelable).
    pub fn send(command: Command) {
        Self::send_as(command, true);
    }
    /// Send non-cancelable sound command.
    pub(super) fn send_non_cancelable(command: Command) {
        Self::send_as(command, false)
    }
    fn send_as(command: Command, cancelable: bool) {
        if let Ok(Some(x)) = COMMANDS
            .get()
            .expect("should have been initialized")
            .try_read()
            .as_deref()
        {
            if let Err(e) = x.try_send(if cancelable {
                command.cancelable()
            } else {
                command.non_cancelable()
            }) {
                match e {
                    TrySendError::Full(command) => {
                        if command.cancelable() {
                            log::warn!(
                                Audioware::env(),
                                "couldn't send command, channel full: {:#?}",
                                command.into_inner()
                            );
                        } else {
                            Self::send_non_cancelable(command.into_inner());
                        }
                    }
                    TrySendError::Disconnected(command) => {
                        log::warn!(
                            Audioware::env(),
                            "error sending command, channel disconnected: {:#?}",
                            command.into_inner()
                        );
                    }
                }
            }
        }
    }
    /// Register an audio emitter to spatial [Scene].
    ///
    /// ⚠️ Returns `true` if already registered.
    pub fn register_emitter(
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
    ) -> bool {
        if let Err(e) = Scene::register_emitter(
            entity_id,
            emitter_name.into_option(),
            emitter_settings.into_option().map(Into::into),
        ) {
            log::error!(Audioware::env(), "couldn't register emitter to scene: {e}");
            return false;
        }
        true
    }
    /// Unregister an audio emitter from spatial [Scene].
    ///
    /// ⚠️ Returns `true` if already unregistered (or never registered).
    pub fn unregister_emitter(entity_id: EntityId) -> bool {
        if let Err(e) = Scene::unregister_emitter(&entity_id) {
            log::error!(
                Audioware::env(),
                "couldn't unregister emitter from scene: {e}"
            );
            return false;
        }
        true
    }
    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        Scene::is_registered_emitter(&entity_id)
    }
    /// Current number of registered [Scene] audio emitters.
    pub fn emitters_count() -> i32 {
        let count = Scene::emitters_count();
        if let Err(e) = count {
            log::error!(Audioware::env(), "couldn't count emitters in scene: {e}");
            return -1;
        }
        count.unwrap() as i32
    }
    /// Whenever [Scene] audio emitter dies in-game.
    pub(crate) fn on_emitter_dies(entity_id: EntityId) {
        if let Err(e) = Scene::on_emitter_dies(entity_id) {
            log::error!(
                Audioware::env(),
                "couldn't remove dying emitter from scene: {e}"
            );
        }
    }
    /// Toggle [Scene] audio emitters synchonization.
    pub(crate) fn toggle_sync_emitters(enable: bool) {
        Scene::toggle_sync_emitters(enable);
    }
    /// Whether [Scene] audio emitters should be synchronized or not.
    pub(crate) fn should_sync_emitters() -> bool {
        Scene::should_sync_emitters()
    }
    /// [Scene] audio emitters synchronization.
    pub(crate) fn sync_emitters() {
        if let Err(e) = Scene::sync_emitters() {
            log::error!(Audioware::env(), "couldn't sync emitters on scene: {e}");
        }
    }
    /// [Scene] audio listener synchronization.
    pub(crate) fn sync_listener() {
        if let Err(e) = Scene::sync_listener() {
            log::error!(Audioware::env(), "couldn't sync listener on scene: {e}");
        }
    }
    /// Free [Manager] storage from stopped sounds.
    pub(crate) fn reclaim() {
        if let Err(e) = Manager::reclaim() {
            log::error!(
                Audioware::env(),
                "couldn't reclaim stopped sound(s) in storage(s): {e}"
            );
        }
    }
    #[doc(hidden)]
    fn play_over_the_phone(event_name: CName, emitter_name: CName, gender: CName) {
        let mut manager = match Manager::try_lock() {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get audio manager: {e}");
                return;
            }
        };
        let spoken = SpokenLocale::get();
        let gender = ok_or_return!(PlayerGender::try_from(gender), "Play over the phone");
        let id = ok_or_return!(
            Banks::try_get(&event_name, &spoken, Some(&gender)),
            "Unable to get sound ID"
        );
        let _duration = ok_or_return!(
            Manager.play_and_store(
                &mut manager,
                id,
                None,
                Some(emitter_name),
                Some(Tracks::holocall_destination()),
                None::<kira::tween::Tween>
            ),
            "Unable to store sound handle"
        );
        // TODO: handle convo?
    }
    /// Play sound with optional [tween][Tween].
    fn play(
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        tween: Ref<Tween>,
    ) {
        let (mut manager, _, _, _, id) =
            ok_or_return!(Self::context(&sound_name), "Unable to get context");
        let entity_id = entity_id.into_option();
        let emitter_name = emitter_name.into_option();

        let tween = tween.into_tween();
        let duration = ok_or_return!(
            Manager.play_and_store(&mut manager, id, entity_id, emitter_name, None, tween),
            "Unable to store sound handle"
        );
        if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
            propagate_subtitles(
                sound_name,
                entity_id,
                emitter_name,
                line_type.unwrap_or_default(),
                duration,
            )
        }
    }
    /// Play sound with [alternate settings][AudioSettingsExt].
    fn play_with(
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    ) {
        let (mut manager, _, _, _, id) =
            ok_or_return!(Self::context(&sound_name), "Unable to get context");
        let entity_id = entity_id.into_option();
        let emitter_name = emitter_name.into_option();

        let duration = ok_or_return!(
            Manager.play_and_store(&mut manager, id, entity_id, emitter_name, None, ext),
            "Unable to store sound handle"
        );
        if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
            propagate_subtitles(
                sound_name,
                entity_id,
                emitter_name,
                line_type.unwrap_or_default(),
                duration,
            )
        }
    }
    /// Stop sound with optional [tween][Tween].
    fn stop(
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        let env = Audioware::env();
        let entity_id = entity_id.into_option();
        let emitter_name = emitter_name.into_option();
        let tween = tween.into_tween();
        utils::silly!("stop called: {entity_id:?} {emitter_name:?} {tween:?}");
        if let Err(e) = Manager.stop_by(
            &event_name,
            entity_id.as_ref(),
            emitter_name.as_ref(),
            tween,
        ) {
            log::error!(env, "{e}");
        }
    }
    /// Pause all sounds with optional [tween][Tween].
    fn pause(tween: Ref<Tween>) {
        if let Err(e) = Manager.pause(tween.into_tween()) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    /// Resume all sounds with optional [tween][Tween],
    /// except those already stopped.
    fn resume(tween: Ref<Tween>) {
        if let Err(e) = Manager.resume(tween.into_tween()) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    /// Switch one sound for another, with optional [tween][Tween] and [alternate settings][AudioSettingsExt].
    fn switch(
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_settings: Ref<AudioSettingsExt>,
    ) {
        let prev = Banks::exists(&switch_name);
        let next = Banks::exists(&switch_value);
        let system = GameInstance::get_audio_system();

        if prev {
            Engine::stop(switch_name, entity_id, emitter_name, switch_name_tween);
        } else {
            system.stop(switch_name, entity_id, emitter_name);
        }

        if next {
            Engine::play_with(
                switch_value,
                entity_id,
                emitter_name,
                Opt::Default,
                switch_value_settings,
            );
        } else {
            system.play(switch_value, entity_id, emitter_name);
        }
    }
    /// Play sound on audio emitter with optional [tween][Tween].
    fn play_on_emitter(
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        let (mut manager, _, _, _, id) =
            ok_or_return!(Self::context(&sound_name), "Unable to get context");
        let destination = some_or_return!(
            Scene::output_destination(&entity_id),
            "Entity is not registered as emitter",
            entity_id
        );
        let duration = ok_or_return!(
            Manager.play_and_store(
                &mut manager,
                id,
                Some(entity_id),
                Some(emitter_name),
                Some(destination),
                tween,
            ),
            "Unable to store sound handle"
        );
        propagate_subtitles(
            sound_name,
            entity_id,
            emitter_name,
            ScnDialogLineType::default(),
            duration,
        );
    }
    /// Stop sound on audio emitter with optional [tween][Tween].
    fn stop_on_emitter(
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        if let Err(e) = Manager.stop_by(
            &event_name,
            Some(&entity_id),
            Some(&emitter_name),
            tween.into_tween(),
        ) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    /// Stop any sound for given [EntityId].
    #[allow(dead_code)]
    fn stop_for(entity_id: EntityId) {
        if let Err(e) = Manager.stop_for(&entity_id, None) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    /// Set [ReverbMix].
    fn set_reverb_mix(value: f32) {
        if !(0. ..=1.).contains(&value) {
            log::error!(
                Audioware::env(),
                "reverb mix must be between 0. and 1. (inclusive)"
            );
            return;
        }
        ok_or_return!(
            ReverbMix::update(value, IMMEDIATELY),
            "Unable to set reverb mix"
        );
    }
    /// Set audio [Preset].
    fn set_preset(value: Preset) {
        let tracks = Tracks::get();
        let mut eq = ok_or_return!(tracks.ambience.try_eq(), "Unable to set EQ preset");
        eq.set_preset(value);
    }
    fn set_volume(setting: CName, value: f64) {
        if !(0.0..=100.0).contains(&value) {
            log::error!(Audioware::env(), "Volume must be between 0. and 100.");
            return;
        }
        let mut manager = ok_or_return!(Manager::try_lock(), "Unable to get audio manager");
        match setting.as_str() {
            "MasterVolume" => manager
                .main_track()
                .set_volume(Volume::Amplitude(value / 100.), IMMEDIATELY),
            "SfxVolume" => {
                ok_or_return!(
                    SfxVolume::update(value, IMMEDIATELY),
                    "Unable to set SfxVolume"
                );
            }
            "DialogueVolume" => {
                ok_or_return!(
                    DialogueVolume::update(value, IMMEDIATELY),
                    "Unable to set DialogueVolume"
                );
            }
            "MusicVolume" => {
                ok_or_return!(
                    MusicVolume::update(value, IMMEDIATELY),
                    "Unable to set MusicVolume"
                );
            }
            "CarRadioVolume" => {
                ok_or_return!(
                    CarRadioVolume::update(value, IMMEDIATELY),
                    "Unable to set CarRadioVolume"
                );
            }
            "RadioportVolume" => {
                ok_or_return!(
                    RadioportVolume::update(value, IMMEDIATELY),
                    "Unable to set RadioportVolume"
                );
            }
            _ => {
                log::error!(Audioware::env(), "Unknown setting: {setting}");
            }
        };
    }
    /// Retrieve current [engine context][EngineContext].
    fn context(sound_name: &CName) -> Result<EngineContext, Error> {
        let manager = Manager::try_lock()?;
        let spoken = SpokenLocale::get();
        let written = WrittenLocale::get();
        let gender = PlayerGender::get();
        let id = Banks::try_get(sound_name, &spoken, gender.as_ref())?;
        Ok((manager, spoken, written, gender, id))
    }
}

pub trait ToOutputDestination {
    fn output_destination(&self) -> OutputDestination;
}

pub enum Context<'a> {
    Unknown { id: &'a Id },
    Entity { id: &'a Id, entity_id: &'a EntityId },
}
impl<'a> Context<'a> {
    pub fn new(id: &'a Id, entity_id: Option<&'a EntityId>) -> Self {
        match entity_id {
            Some(entity_id) => Self::Entity { id, entity_id },
            None => Self::Unknown { id },
        }
    }
}

impl ToOutputDestination for Id {
    fn output_destination(&self) -> OutputDestination {
        match self {
            Id::OnDemand(_, source) | Id::InMemory(_, source) => match source {
                Source::Sfx => (&Tracks::get().sfx).into(),
                Source::Ono => (&Tracks::get().sfx).into(),
                Source::Voices => (&Tracks::get().dialogue).into(),
                Source::Playlist => (&Tracks::get().radioport).into(),
                Source::Music => (&Tracks::get().music).into(),
                Source::Jingle => (&Tracks::get().car_radio).into(),
            },
        }
    }
}

impl<'a> ToOutputDestination for Context<'a> {
    fn output_destination(&self) -> OutputDestination {
        match self {
            Context::Unknown { id } => id.output_destination(),
            Context::Entity { id, entity_id } => {
                let game = GameInstance::new();
                let entity = GameInstance::find_entity_by_id(game, **entity_id);
                if let Some(go) = entity.cast::<GameObject>() {
                    if go.is_player() {
                        match id {
                            Id::OnDemand(_, source) | Id::InMemory(_, source) => {
                                if let Some(destination) =
                                    Tracks::get().v.output_destination(source)
                                {
                                    return destination;
                                }
                            }
                        };
                    }
                }
                id.output_destination()
            }
        }
    }
}
