#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../README.md")]

include!(concat!(env!("OUT_DIR"), "/version.rs"));

use crate::abi::{BlackboardLifecycle, GameSessionLifecycle, GameSystemLifecycle};
use abi::lifecycle::Lifecycle;
use red4ext_rs::{
    export_plugin_symbols, exports, global, log::error, wcstr, Exportable, GameApp, GlobalExport,
    Plugin, RttiRegistrator, SdkEnv, SemVer, StateListener, StateType, U16CStr,
};
pub use types::*;
use utils::lifecycle;

mod abi;
mod config;
mod engine;
mod error;
mod queue;
mod states;
mod types;
mod utils;

/// Audio [Plugin] for Cyberpunk 2077.
pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    /// Initialize plugin.
    fn on_init(env: &SdkEnv) {
        Self::register_listeners(env);
        if let Err(e) = queue::spawn(env) {
            error!(env, "Error: {e}");
        }
    }

    /// Register types in [RTTI][RttiSystem].
    #[allow(clippy::transmute_ptr_to_ref)] // upstream lint
    fn exports() -> impl Exportable {
        exports![
            GlobalExport(global!(
                c"Audioware.OnGameSessionBeforeStart",
                Audioware::on_game_session_before_start
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSessionStart",
                Audioware::on_game_session_start
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSessionReady",
                Audioware::on_game_session_ready
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSessionPause",
                Audioware::on_game_session_pause
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSessionResume",
                Audioware::on_game_session_resume
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSessionBeforeEnd",
                Audioware::on_game_session_before_end
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSessionEnd",
                Audioware::on_game_session_end
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSystemAttach",
                Audioware::on_game_system_attach
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSystemDetach",
                Audioware::on_game_system_detach
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSystemPlayerAttach",
                Audioware::on_game_system_player_attach
            )),
            GlobalExport(global!(
                c"Audioware.OnGameSystemPlayerDetach",
                Audioware::on_game_system_player_detach
            )),
            GlobalExport(global!(c"Audioware.OnUIMenu", Audioware::on_ui_menu)),
        ]
    }
}

impl Audioware {
    /// Register [plugin][Plugin] lifecycle listeners.
    fn register_listeners(env: &SdkEnv) {
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
}

export_plugin_symbols!(Audioware);

/// On RTTI registration.
unsafe extern "C" fn register() {
    lifecycle!("on RTTI register");
}

/// Once RTTI registered.
unsafe extern "C" fn post_register() {
    lifecycle!("on RTTI post register");
}

/// Once plugin initialized.
unsafe extern "C" fn on_exit_initialization(_game: &GameApp) {
    lifecycle!("on plugin exit initialization");
}

/// Unload [Plugin].
unsafe extern "C" fn on_exit_running(_game: &GameApp) {
    queue::notify(Lifecycle::Terminate);
}
