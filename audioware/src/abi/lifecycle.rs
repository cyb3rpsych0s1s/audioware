use crossbeam::channel::Sender;
use red4ext_rs::types::{CName, EntityId, Opt};

use crate::EmitterSettings;

mod board;
mod session;
mod system;
pub use board::Board;
pub use session::Session;
pub use system::System;

/// Engine inner lifecycle.
#[derive(Debug)]
pub enum Lifecycle {
    RegisterEmitter {
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
        sender: Sender<bool>,
    },
    UnregisterEmitter {
        entity_id: EntityId,
        sender: Sender<bool>,
    },
    SyncScene,
    Reclaim,
    Shutdown,
    Terminate,
    Session(Session),
    System(System),
    Board(Board),
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifecycle::RegisterEmitter { entity_id, .. } => {
                write!(f, "register emitter [{:?}]", entity_id)
            }
            Lifecycle::UnregisterEmitter { entity_id, .. } => {
                write!(f, "unregister emitter [{:?}]", entity_id)
            }
            Lifecycle::SyncScene => write!(f, "sync scene"),
            Lifecycle::Reclaim => write!(f, "reclaim"),
            Lifecycle::Shutdown => write!(f, "shutdown"),
            Lifecycle::Terminate => write!(f, "terminate"),
            Lifecycle::Session(x) => write!(f, "{x}"),
            Lifecycle::System(x) => write!(f, "{x}"),
            Lifecycle::Board(x) => write!(f, "{x}"),
        }
    }
}
