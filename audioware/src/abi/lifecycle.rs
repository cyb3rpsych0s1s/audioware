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
    SetListenerDilation {
        value: f32,
        reason: CName,
        ease_in_curve: CName,
    },
    UnsetListenerDilation {
        reason: CName,
        ease_out_curve: CName,
    },
    SetEmitterDilation {
        reason: CName,
        entity_id: EntityId,
        value: f32,
        ease_in_curve: CName,
    },
    UnsetEmitterDilation {
        entity_id: EntityId,
        ease_out_curve: CName,
    },
    Session(Session),
    System(System),
    Board(Board),
    Codeware(Codeware),
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
            Lifecycle::SetListenerDilation { value: dilation, reason, ease_in_curve } => {
                write!(f, "set listener dilation {dilation}, reason: {reason}, curve: {ease_in_curve}")
            }
            Lifecycle::UnsetListenerDilation { reason, ease_out_curve } => write!(f, "unset listener dilation, reason: {reason}, curve: {ease_out_curve}"),
            Lifecycle::SetEmitterDilation {
                reason,
                entity_id,
                value: dilation,
                ease_in_curve,
            } => write!(f, "set emitter dilation {dilation}, reason: {reason}, curve: {ease_in_curve} [{entity_id:?}]"),
            Lifecycle::UnsetEmitterDilation { entity_id,  ease_out_curve } => {
                write!(f, "unset emitter dilation, curve: {ease_out_curve} [{entity_id:?}]")
            }
        }
    }
}
