use audioware_manifest::{PlayerGender, ScnDialogLineType, Settings};
use kira::tween::Tween;
use red4ext_rs::types::{CName, EntityId};

/// Sound inner command.
#[derive(Clone)]
pub enum Command {
    PlayVanilla {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    },
    Play {
        sound_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
        ext: Option<Settings>,
    },
    PlayOnEmitter {
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        ext: Option<Settings>,
    },
    PlayOverThePhone {
        event_name: CName,
        emitter_name: CName,
        gender: PlayerGender,
    },
    StopOnEmitter {
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Option<Tween>,
    },
    StopVanilla {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    },
    Stop {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        tween: Option<Tween>,
    },
    Switch {
        switch_name: CName,
        switch_value: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        switch_name_tween: Option<Tween>,
        switch_value_settings: Option<Settings>,
    },
    SwitchVanilla {
        switch_name: CName,
        switch_value: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    },
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Play {
                sound_name,
                entity_id,
                emitter_name,
                line_type,
                ..
            } => write!(f, "Command::Play {{ sound_name: {:?}, entity_id: {:?}, emitter_name: {:?}, line_type: {:?}, .. }}", sound_name, entity_id, emitter_name, line_type),
            Command::PlayOnEmitter {
                event_name,
                entity_id,
                emitter_name,
                ..
            } => write!(f, "Command::PlayOnEmitter {{ event_name: {:?}, entity_id: {:?}, emitter_name: {:?}, .. }}", event_name, entity_id, emitter_name),
            Command::StopOnEmitter {
                event_name,
                entity_id,
                emitter_name,
                ..
            } => write!(f, "Command::StopOnEmitter {{ event_name: {:?}, entity_id: {:?}, emitter_name: {:?}, .. }}", event_name, entity_id, emitter_name),
            Command::Stop {
                event_name,
                entity_id,
                emitter_name,
                ..
            } => write!(f, "Command::Stop {{ event_name: {:?}, entity_id: {:?}, emitter_name: {:?}, .. }}", event_name, entity_id, emitter_name),
            Command::Switch {
                switch_name,
                switch_value,
                entity_id,
                emitter_name,
                ..
            } => write!(f, "Command::Switch {{ switch_name: {:?}, switch_value: {:?}, entity_id: {:?}, emitter_name: {:?}, .. }}", switch_name, switch_value, entity_id, emitter_name),
            x => write!(f, "{x:?}"),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::PlayVanilla { .. } => "play vanilla",
                Command::Play { .. } => "play",
                Command::PlayOnEmitter { .. } => "play on emitter",
                Command::PlayOverThePhone { .. } => "play over the phone",
                Command::StopOnEmitter { .. } => "stop on emitter",
                Command::StopVanilla { .. } => "stop vanilla",
                Command::Stop { .. } => "stop",
                Command::Switch { .. } => "switch",
                Command::SwitchVanilla { .. } => "switch vanilla",
            }
        )
    }
}
