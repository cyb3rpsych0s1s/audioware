use crossbeam::channel::Sender;
use kira::spatial::emitter::EmitterSettings;
use red4ext_rs::types::{CName, EntityId};

mod board;
mod codeware;
mod session;
mod system;
pub use board::Board;
pub use codeware::Codeware;
pub use session::Session;
pub use system::System;

/// Engine inner lifecycle.
#[derive(Debug)]
pub enum Lifecycle {
    IsRegisteredEmitter {
        entity_id: EntityId,
        sender: Sender<bool>,
    },
    RegisterEmitter {
        entity_id: EntityId,
        emitter_name: Option<CName>,
        emitter_settings: Option<EmitterSettings>,
        sender: Sender<bool>,
    },
    UnregisterEmitter {
        entity_id: EntityId,
        sender: Sender<bool>,
    },
    OnEmitterDies {
        entity_id: EntityId,
    },
    OnEmitterIncapacitated {
        entity_id: EntityId,
    },
    OnEmitterDefeated {
        entity_id: EntityId,
    },
    Terminate,
    SetVolume {
        setting: CName,
        value: f64,
    },
    Session(Session),
    System(System),
    Board(Board),
    Codeware(Codeware),
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifecycle::IsRegisteredEmitter { entity_id, .. } => {
                write!(f, "is emitter registered? [{:?}]", entity_id)
            }
            Lifecycle::RegisterEmitter { entity_id, .. } => {
                write!(f, "register emitter [{:?}]", entity_id)
            }
            Lifecycle::UnregisterEmitter { entity_id, .. } => {
                write!(f, "unregister emitter [{:?}]", entity_id)
            }
            Lifecycle::OnEmitterDies { entity_id } => {
                write!(f, "on emitter dies [{:?}]", entity_id)
            }
            Lifecycle::OnEmitterIncapacitated { entity_id } => {
                write!(f, "on emitter incapacitated [{:?}]", entity_id)
            }
            Lifecycle::OnEmitterDefeated { entity_id } => {
                write!(f, "on emitter defeated [{:?}]", entity_id)
            }
            Lifecycle::Terminate => write!(f, "terminate"),
            Lifecycle::Session(x) => write!(f, "{x}"),
            Lifecycle::System(x) => write!(f, "{x}"),
            Lifecycle::Board(x) => write!(f, "{x}"),
            Lifecycle::SetVolume { setting, value } => {
                write!(f, "set volume {} {value}", setting.as_str())
            }
            Lifecycle::Codeware(x) => write!(f, "{x}"),
        }
    }
}
