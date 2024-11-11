use lifecycle::{Board, Lifecycle, Session, System};
use red4ext_rs::{
    exports, global, Exportable, GameApp, GlobalExport, RttiRegistrator, SdkEnv, StateListener,
    StateType,
};

use crate::{queue, utils::lifecycle, Audioware};

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

pub fn exports() -> impl Exportable {
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
}