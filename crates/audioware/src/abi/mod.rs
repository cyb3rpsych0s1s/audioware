use audioware_manifest::{Locale, PlayerGender, ScnDialogLineType, Validate};
use command::Command;
use crossbeam::channel::bounded;
use kira::manager::backend::cpal::CpalBackend;
use lifecycle::{Board, Lifecycle, Session, System};
use red4ext_rs::{
    exports, methods,
    types::{CName, EntityId, IScriptable, Opt, Ref},
    ClassExport, Exportable, GameApp, RttiRegistrator, ScriptClass, SdkEnv, StateListener,
    StateType,
};

use crate::{
    engine::{eq::Preset, state, Engine},
    queue,
    utils::{fails, lifecycle, warns},
    Audioware, EmitterSettings, LocalizationPackage, ToTween, Tween,
};

pub mod command;
pub mod lifecycle;

mod types;
use types::*;

/// Register [plugin][super::Plugin] lifecycle listeners.
pub fn register_listeners(env: &SdkEnv) {
    RttiRegistrator::add(Some(register), Some(post_register));
    env.add_listener(
        StateType::Initialization,
        StateListener::default().with_on_exit(on_exit_initialization),
    );
    env.add_listener(
        StateType::Running,
        StateListener::default().with_on_exit(on_exit_running),
    );
}

macro_rules! g {
    ($reds:literal, $rust:path) => {
        ::red4ext_rs::GlobalExport(::red4ext_rs::global!($reds, $rust))
    };
}

/// Register types in [RTTI][red4ext_rs::RttiSystem].
#[allow(clippy::transmute_ptr_to_ref)] // upstream lint
#[rustfmt::skip]
pub fn exports() -> impl Exportable {
    let exp = exports![
        ClassExport::<AudioSystemExt>::builder()
                .base(IScriptable::NAME)
                .methods(methods![
                    final c"Play" => AudioSystemExt::play,
                    final c"Stop" => AudioSystemExt::stop,
                    final c"Switch" => AudioSystemExt::switch,
                    final c"PlayOverThePhone" => AudioSystemExt::play_over_the_phone,
                    final c"PlayOnEmitter" => AudioSystemExt::play_on_emitter,
                    final c"StopOnEmitter" => AudioSystemExt::stop_on_emitter,
                    final c"RegisterEmitter" => AudioSystemExt::register_emitter,
                    final c"UnregisterEmitter" => AudioSystemExt::unregister_emitter,
                    final c"IsRegisteredEmitter" => AudioSystemExt::is_registered_emitter,
                    final c"OnEmitterDies" => AudioSystemExt::on_emitter_dies,
                    final c"OnEmitterIncapacitated" => AudioSystemExt::on_emitter_incapacitated,
                    final c"OnEmitterDefeated" => AudioSystemExt::on_emitter_defeated,
                    final c"EmittersCount" => AudioSystemExt::emitters_count,
                    final c"Duration" => AudioSystemExt::duration,
                    final c"IsDebug" => AudioSystemExt::is_debug,
                    final c"SemanticVersion" => AudioSystemExt::semantic_version,
                ])
                .build(),
        g!(c"Audioware.OnGameSessionBeforeStart",   Audioware::on_game_session_before_start),
        g!(c"Audioware.OnGameSessionStart",         Audioware::on_game_session_start),
        g!(c"Audioware.OnGameSessionReady",         Audioware::on_game_session_ready),
        g!(c"Audioware.OnGameSessionPause",         Audioware::on_game_session_pause),
        g!(c"Audioware.OnGameSessionResume",        Audioware::on_game_session_resume),
        g!(c"Audioware.OnGameSessionBeforeEnd",     Audioware::on_game_session_before_end),
        g!(c"Audioware.OnGameSessionEnd",           Audioware::on_game_session_end),
        g!(c"Audioware.OnGameSystemAttach",         Audioware::on_game_system_attach),
        g!(c"Audioware.OnGameSystemDetach",         Audioware::on_game_system_detach),
        g!(c"Audioware.OnGameSystemPlayerAttach",   Audioware::on_game_system_player_attach),
        g!(c"Audioware.OnGameSystemPlayerDetach",   Audioware::on_game_system_player_detach),
        g!(c"Audioware.OnUIMenu",                   Audioware::on_ui_menu),
        g!(c"Audioware.SetReverbMix",               Audioware::on_reverb_mix),
        g!(c"Audioware.SetPreset",                  Audioware::on_preset),
        g!(c"Audioware.SetVolume",                  Audioware::set_volume),
        g!(c"Audioware.SetPlayerGender",            Audioware::set_player_gender),
        g!(c"Audioware.UnsetPlayerGender",          Audioware::unset_player_gender),
        g!(c"Audioware.SetGameLocales",             Audioware::set_game_locales),
        g!(c"Audioware.DefineSubtitles",            Audioware::define_subtitles),
        g!(c"Audioware.SupportedLanguages",         Audioware::supported_languages),
    ];
    #[cfg(not(feature = "hot-reload"))]
    {exp}
    #[cfg(feature = "hot-reload")]
    {
        use crate::abi::debug::HotReload;
        exports![exp, g!(c"HotReload", Audioware::hot_reload)]
    }
}

/// On RTTI registration.
unsafe extern "C" fn register() {
    lifecycle!("on RTTI register");
}

/// Once RTTI registered.
unsafe extern "C" fn post_register() {
    lifecycle!("on RTTI post register");
    Audioware::once_rtti_registered();
}

/// Once plugin initialized.
unsafe extern "C" fn on_exit_initialization(_: &GameApp) {
    lifecycle!("on plugin exit initialization");
    Audioware::once_exit_initialization();
}

/// Unload [Plugin][super::Plugin].
unsafe extern "C" fn on_exit_running(_: &GameApp) {
    queue::notify(Lifecycle::Terminate);
}

pub trait GameSessionLifecycle {
    fn on_game_session_before_start();
    fn on_game_session_start();
    fn on_game_session_ready();
    fn on_game_session_pause();
    fn on_game_session_resume();
    fn on_game_session_before_end();
    fn on_game_session_end();
}

pub trait GameSystemLifecycle {
    fn on_game_system_attach();
    fn on_game_system_detach();
    fn on_game_system_player_attach();
    fn on_game_system_player_detach();
}

pub trait BlackboardLifecycle {
    fn on_ui_menu(value: bool);
    fn on_reverb_mix(value: f32);
    fn on_preset(value: Preset);
}

pub trait CodewareLifecycle {
    fn set_player_gender(gender: PlayerGender);
    fn unset_player_gender();
    fn set_game_locales(spoken: CName, written: CName);
    fn define_subtitles(package: Ref<LocalizationPackage>);
    fn supported_languages() -> Vec<CName>;
}

pub trait ListenerLifecycle {
    fn set_volume(setting: CName, value: f64);
}

impl GameSessionLifecycle for Audioware {
    fn on_game_session_before_start() {
        queue::notify(Lifecycle::Session(Session::BeforeStart));
    }

    fn on_game_session_start() {
        queue::notify(Lifecycle::Session(Session::Start));
    }

    fn on_game_session_ready() {
        queue::notify(Lifecycle::Session(Session::Ready));
    }

    fn on_game_session_pause() {
        queue::notify(Lifecycle::Session(Session::Pause));
    }

    fn on_game_session_resume() {
        queue::notify(Lifecycle::Session(Session::Resume));
    }

    fn on_game_session_before_end() {
        queue::notify(Lifecycle::Session(Session::BeforeEnd));
    }

    fn on_game_session_end() {
        queue::notify(Lifecycle::Session(Session::End));
    }
}

impl GameSystemLifecycle for Audioware {
    fn on_game_system_attach() {
        queue::notify(Lifecycle::System(System::Attach));
    }

    fn on_game_system_detach() {
        queue::notify(Lifecycle::System(System::Detach));
    }

    fn on_game_system_player_attach() {
        queue::notify(Lifecycle::System(System::PlayerAttach));
    }

    fn on_game_system_player_detach() {
        queue::notify(Lifecycle::System(System::PlayerDetach));
    }
}

impl BlackboardLifecycle for Audioware {
    fn on_ui_menu(value: bool) {
        queue::notify(Lifecycle::Board(Board::UIMenu(value)));
    }

    fn on_reverb_mix(value: f32) {
        queue::notify(Lifecycle::Board(Board::ReverbMix(value)));
    }

    fn on_preset(value: Preset) {
        queue::notify(Lifecycle::Board(Board::Preset(value)));
    }
}

impl ListenerLifecycle for Audioware {
    fn set_volume(setting: CName, value: f64) {
        queue::notify(Lifecycle::SetVolume { setting, value });
    }
}

impl CodewareLifecycle for Audioware {
    fn set_player_gender(gender: PlayerGender) {
        state::PlayerGender::set(gender);
    }

    fn unset_player_gender() {
        state::PlayerGender::unset();
    }

    fn set_game_locales(spoken: CName, written: CName) {
        match Locale::try_from(spoken) {
            Ok(spoken) => state::SpokenLocale::set(spoken),
            Err(e) => fails!("failed to set spoken locale: {e}"),
        };
        match Locale::try_from(written) {
            Ok(written) => state::WrittenLocale::set(written),
            Err(e) => fails!("failed to set written locale: {e}"),
        };
    }

    fn define_subtitles(package: Ref<LocalizationPackage>) {
        use crate::types::Subtitle;
        use audioware_bank::BankSubtitles;
        let written = state::WrittenLocale::get();
        lifecycle!("define localization package subtitles for {written}");
        if let Some(banks) = Engine::<CpalBackend>::banks().as_ref() {
            let subtitles = banks.subtitles(written);
            for (key, (value_f, value_m)) in subtitles.iter() {
                package.subtitle(key.as_str(), value_f.as_str(), value_m.as_str());
            }
        } else {
            warns!("banks aren't initialized yet, skipping subtitles definition");
        }
    }

    fn supported_languages() -> Vec<CName> {
        Engine::<CpalBackend>::supported_languages()
    }
}

pub trait SceneLifecycle {
    fn register_emitter(
        &self,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Opt<CName>,
        emitter_settings: Ref<EmitterSettings>,
    ) -> bool;
    fn unregister_emitter(&self, entity_id: EntityId, tag_name: CName) -> bool;
    fn is_registered_emitter(&self, entity_id: EntityId, tag_name: Opt<CName>) -> bool;
    fn on_emitter_dies(&self, entity_id: EntityId);
    fn on_emitter_incapacitated(&self, entity_id: EntityId);
    fn on_emitter_defeated(&self, entity_id: EntityId);
    fn emitters_count(&self) -> i32;
}

impl SceneLifecycle for AudioSystemExt {
    fn register_emitter(
        &self,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Opt<CName>,
        emitter_settings: Ref<EmitterSettings>,
    ) -> bool {
        let tag_name = match TagName::try_new(tag_name) {
            Ok(tag_name) => tag_name,
            Err(e) => {
                warns!("{e}");
                return false;
            }
        };
        let entity_id = match TargetId::try_new(entity_id) {
            Ok(entity_id) => entity_id,
            Err(e) => {
                warns!("{e}");
                return false;
            }
        };
        let emitter_settings = match TargetFootprint::try_new(emitter_settings, *entity_id) {
            Ok(emitter_settings) => emitter_settings,
            Err(e) => {
                warns!(
                    "{}",
                    e.iter()
                        .map(|e| format!("{e}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                return false;
            }
        };
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::RegisterEmitter {
            tag_name,
            entity_id,
            emitter_name: emitter_name.into_option(),
            emitter_settings,
            sender,
        });
        if let Ok(registered) = receiver.recv() {
            return registered;
        }
        false
    }

    fn unregister_emitter(&self, entity_id: EntityId, tag_name: CName) -> bool {
        let tag_name = match TagName::try_new(tag_name) {
            Ok(tag_name) => tag_name,
            Err(e) => {
                warns!("{e}");
                return false;
            }
        };
        let entity_id = match TargetId::try_new(entity_id) {
            Ok(entity_id) => entity_id,
            Err(e) => {
                warns!("{e}");
                return false;
            }
        };
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::UnregisterEmitter {
            entity_id,
            tag_name,
            sender,
        });
        if let Ok(unregistered) = receiver.recv() {
            return unregistered;
        }
        false
    }

    fn is_registered_emitter(&self, entity_id: EntityId, tag_name: Opt<CName>) -> bool {
        Engine::<CpalBackend>::is_registered_emitter(entity_id, tag_name.into_option())
    }

    fn on_emitter_dies(&self, entity_id: EntityId) {
        queue::notify(Lifecycle::OnEmitterDies { entity_id });
    }

    fn on_emitter_incapacitated(&self, entity_id: EntityId) {
        queue::notify(Lifecycle::OnEmitterIncapacitated { entity_id });
    }

    fn on_emitter_defeated(&self, entity_id: EntityId) {
        queue::notify(Lifecycle::OnEmitterDefeated { entity_id });
    }

    fn emitters_count(&self) -> i32 {
        Engine::<CpalBackend>::emitters_count()
    }
}

pub trait ExtCommand {
    fn play(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    );
    fn play_over_the_phone(&self, event_name: CName, emitter_name: CName, gender: CName);
    fn stop(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    );
    /// Play sound on audio emitter with optional [tween][Tween].
    fn play_on_emitter(
        &self,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        ext: Ref<AudioSettingsExt>,
    );
    fn stop_on_emitter(
        &self,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        tween: Ref<Tween>,
    );
    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_ext: Ref<AudioSettingsExt>,
    );
}

impl ExtCommand for AudioSystemExt {
    fn play(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    ) {
        let ext = ext.into_settings();
        if let Some(Err(e)) = ext.as_ref().map(Validate::validate) {
            warns!("invalid audio settings: {:#?}", e);
            return;
        }
        queue::send(Command::Play {
            event_name,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            line_type: line_type.into_option(),
            ext,
        });
    }

    fn stop(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        queue::send(Command::Stop {
            event_name,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            tween: tween.into_tween(),
        });
    }

    fn play_on_emitter(
        &self,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        ext: Ref<AudioSettingsExt>,
    ) {
        let ext = ext.into_settings();
        let tag_name = match TagName::try_new(tag_name) {
            Ok(tag_name) => tag_name,
            Err(e) => {
                warns!("{e}");
                return;
            }
        };
        let entity_id = match TargetId::try_new(entity_id) {
            Ok(entity_id) => entity_id,
            Err(e) => {
                warns!("{e}");
                return;
            }
        };
        if let Some(Err(e)) = ext.as_ref().map(Validate::validate) {
            warns!("invalid audio settings: {:#?}", e);
            return;
        }
        queue::send(Command::PlayOnEmitter {
            event_name,
            entity_id,
            tag_name,
            ext,
        });
    }

    fn stop_on_emitter(
        &self,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        tween: Ref<Tween>,
    ) {
        let tag_name = match TagName::try_new(tag_name) {
            Ok(tag_name) => tag_name,
            Err(e) => {
                warns!("{e}");
                return;
            }
        };
        let entity_id = match TargetId::try_new(entity_id) {
            Ok(entity_id) => entity_id,
            Err(e) => {
                warns!("{e}");
                return;
            }
        };
        queue::send(Command::StopOnEmitter {
            event_name,
            entity_id,
            tag_name,
            tween: tween.into_tween(),
        });
    }

    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_ext: Ref<AudioSettingsExt>,
    ) {
        let switch_value_ext = switch_value_ext.into_settings();
        if let Some(Err(e)) = switch_value_ext.as_ref().map(Validate::validate) {
            warns!("invalid audio settings: {:#?}", e);
            return;
        }
        queue::send(Command::Switch {
            switch_name,
            switch_value,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            switch_name_tween: switch_name_tween.into_tween(),
            switch_value_settings: switch_value_ext,
        });
    }

    fn play_over_the_phone(&self, event_name: CName, emitter_name: CName, gender: CName) {
        match PlayerGender::try_from(gender) {
            Ok(gender) => {
                queue::send(Command::PlayOverThePhone {
                    event_name,
                    emitter_name,
                    gender,
                });
            }
            Err(e) => {
                warns!("invalid gender: {e}");
            }
        }
    }
}

#[cfg(feature = "hot-reload")]
mod debug {
    pub trait HotReload {
        fn hot_reload();
    }

    impl HotReload for crate::Audioware {
        fn hot_reload() {
            crate::queue::notify(crate::abi::Lifecycle::HotReload);
        }
    }
}
