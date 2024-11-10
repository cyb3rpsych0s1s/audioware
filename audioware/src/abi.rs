use lifecycle::{Board, Lifecycle, Session, System};

use crate::{queue, Audioware};

pub mod command;
pub mod lifecycle;

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

// #[allow(dead_code)]
// fn register_emitter(
//     entity_id: EntityId,
//     emitter_name: Opt<CName>,
//     emitter_settings: Opt<EmitterSettings>,
// ) -> bool {
//     if let Some(x) = LIFECYCLE.get() {
//         if let Some(Some(x)) = x.try_write().as_deref_mut() {
//             let (sender, receiver) = bounded(0);
//             if x.try_send(Lifecycle::RegisterEmitter {
//                 entity_id,
//                 emitter_name,
//                 emitter_settings,
//                 sender,
//             })
//             .is_ok()
//             {
//                 let handle = std::thread::spawn(move || match receiver.recv() {
//                     Ok(x) => x,
//                     Err(e) => {
//                         log::error!(
//                             Audioware::env(),
//                             "failed to get register emitter callback response: {}",
//                             e
//                         );
//                         return false;
//                     }
//                 });
//                 match handle.join() {
//                     Ok(x) => return x,
//                     Err(_) => {
//                         log::error!(
//                             Audioware::env(),
//                             "failed to join register emitter callback thread"
//                         );
//                     }
//                 }
//             } else {
//                 log::error!(Audioware::env(), "failed to notify register emitter");
//             }
//         }
//     }
//     false
// }
