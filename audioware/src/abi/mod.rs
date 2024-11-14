use audioware_manifest::ScnDialogLineType;
use command::Command;
use crossbeam::channel::bounded;
use lifecycle::{Board, Lifecycle, Session, System};
use red4ext_rs::{
    class_kind::{Native, Scripted},
    exports, methods,
    types::{CName, EntityId, IScriptable, Opt, Ref},
    ClassExport, Exportable, GameApp, RttiRegistrator, ScriptClass, SdkEnv, StateListener,
    StateType, StructExport,
};

use crate::{
    engine::eq::Preset, queue, utils::lifecycle, Audioware, EmitterDistances, EmitterSettings,
    ToTween, Tween,
};

pub mod command;
pub mod lifecycle;

/// Register [plugin][Plugin] lifecycle listeners.
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

/// Register types in [RTTI][RttiSystem].
#[allow(clippy::transmute_ptr_to_ref)] // upstream lint
#[rustfmt::skip]
pub fn exports() -> impl Exportable {
    exports![
        StructExport::<EmitterDistances>::builder().build(),
        StructExport::<EmitterSettings>::builder().build(),
        ClassExport::<AudioSystemExt>::builder()
                .base(IScriptable::NAME)
                .methods(methods![
                    final c"Play" => AudioSystemExt::play,
                    final c"Stop" => AudioSystemExt::stop,
                    final c"PlayOnEmitter" => AudioSystemExt::play_on_emitter,
                    final c"RegisterEmitter" => AudioSystemExt::register_emitter,
                    final c"UnregisterEmitter" => AudioSystemExt::unregister_emitter,
                    final c"IsRegisteredEmitter" => AudioSystemExt::is_registered_emitter,
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
    ]
}

/// On RTTI registration.
unsafe extern "C" fn register() {
    lifecycle!("on RTTI register");
}

/// Once RTTI registered.
unsafe extern "C" fn post_register() {
    lifecycle!("on RTTI post register");
}

/// Once plugin initialized.
unsafe extern "C" fn on_exit_initialization(_: &GameApp) {
    lifecycle!("on plugin exit initialization");
}

/// Unload [Plugin].
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

pub trait SceneLifecycle {
    fn register_emitter(
        &self,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
    ) -> bool;
    fn unregister_emitter(&self, entity_id: EntityId) -> bool;
    fn is_registered_emitter(&self, entity_id: EntityId) -> bool;
}

impl SceneLifecycle for AudioSystemExt {
    fn register_emitter(
        &self,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
    ) -> bool {
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::RegisterEmitter {
            entity_id,
            emitter_name: emitter_name.into_option(),
            emitter_settings: emitter_settings.into_option().map(Into::into),
            sender,
        });
        if let Ok(registered) = receiver.recv() {
            return registered;
        }
        false
    }

    fn unregister_emitter(&self, entity_id: EntityId) -> bool {
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::UnregisterEmitter { entity_id, sender });
        if let Ok(unregistered) = receiver.recv() {
            return unregistered;
        }
        false
    }

    fn is_registered_emitter(&self, entity_id: EntityId) -> bool {
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::IsRegisteredEmitter { entity_id, sender });
        if let Ok(registered) = receiver.recv() {
            return registered;
        }
        false
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSettingsExt {
    start_position: f32,
}

unsafe impl ScriptClass for AudioSettingsExt {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.AudioSettingsExt";
}

/// Interop type for [Ext.reds](https://github.com/cyb3rpsych0s1s/audioware/blob/main/audioware/reds/Ext.reds).
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSystemExt {
    base: IScriptable,
}

unsafe impl ScriptClass for AudioSystemExt {
    type Kind = Native;
    const NAME: &'static str = "AudioSystemExt";
}

pub trait ExtCommand {
    fn play(
        &self,
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    );
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
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    );
}

impl ExtCommand for AudioSystemExt {
    fn play(
        &self,
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        _ext: Ref<AudioSettingsExt>, // TODO:
    ) {
        queue::send(Command::PlayExt {
            sound_name,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            line_type: line_type.into_option(),
            ext: None,
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
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        queue::send(Command::PlayOnEmitter {
            sound_name,
            entity_id,
            emitter_name,
            tween: tween.clone().into_tween(),
        });
    }
}
