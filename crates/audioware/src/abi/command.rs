use audioware_bank::error::registry::ErrorDisplay;
use audioware_manifest::{PlayerGender, ScnDialogLineType, Settings};
use kira::Tween;
use red4ext_rs::types::{CName, Cruid, EntityId};

use crate::ControlId;

use super::{TagName, TargetId};

/// Sound inner command.
#[derive(Clone)]
pub enum Command {
    PlayVanilla {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    },
    Play {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
        ext: Option<Settings>,
    },
    EnqueueAndPlay {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
        ext: Option<Settings>,
        control_id: ControlId,
    },
    PlayOnEmitter {
        event_name: CName,
        entity_id: TargetId,
        tag_name: TagName,
        ext: Option<Settings>,
    },
    EnqueueAndPlayOnEmitter {
        event_name: CName,
        entity_id: TargetId,
        tag_name: TagName,
        ext: Option<Settings>,
        control_id: ControlId,
    },
    PlayOverThePhone {
        event_name: CName,
        emitter_name: CName,
        gender: PlayerGender,
    },
    PlaySceneDialog {
        string_id: Cruid,
        entity_id: EntityId,
        is_player: bool,
        is_holocall: bool,
        is_rewind: bool,
        seek_time: f32,
    },
    StopSceneDialog {
        string_id: Cruid,
        fade_out: f32,
    },
    StopOnEmitter {
        event_name: CName,
        entity_id: TargetId,
        tag_name: TagName,
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
                event_name: sound_name,
                entity_id,
                emitter_name,
                line_type,
                ..
            } => write!(
                f,
                "Command::Play {{ sound_name: {sound_name}, entity_id: {}, emitter_name: {}, line_type: {:?}, .. }}",
                entity_id
                    .map(|x| format!("Some({x})"))
                    .unwrap_or("None".into()),
                emitter_name.map(|x| x.as_str()).unwrap_or("None"),
                line_type
            ),
            Command::PlayOnEmitter {
                event_name,
                entity_id,
                tag_name,
                ..
            } => write!(
                f,
                "Command::PlayOnEmitter {{ event_name: {event_name}, entity_id: {entity_id}, emitter_name: {}, .. }}",
                tag_name.as_str()
            ),
            Command::PlaySceneDialog {
                string_id,
                entity_id,
                ..
            } => write!(
                f,
                "Command::PlaySceneDialog {{ string_id: {}, entity_id: {entity_id}, .. }}",
                string_id.error_display()
            ),
            Command::StopSceneDialog { string_id, .. } => write!(
                f,
                "Command::StopSceneDialog {{ string_id: {}, .. }}",
                string_id.error_display()
            ),
            Command::StopOnEmitter {
                event_name,
                entity_id,
                tag_name,
                ..
            } => write!(
                f,
                "Command::StopOnEmitter {{ event_name: {event_name}, entity_id: {entity_id}, emitter_name: {}, .. }}",
                tag_name.as_str()
            ),
            Command::Stop {
                event_name,
                entity_id,
                emitter_name,
                ..
            } => write!(
                f,
                "Command::Stop {{ event_name: {event_name}, entity_id: {}, emitter_name: {}, .. }}",
                entity_id
                    .map(|x| format!("Some({x})"))
                    .unwrap_or("None".into()),
                emitter_name.map(|x| x.as_str()).unwrap_or("None")
            ),
            Command::Switch {
                switch_name,
                switch_value,
                entity_id,
                emitter_name,
                ..
            } => write!(
                f,
                "Command::Switch {{ switch_name: {switch_name}, switch_value: {switch_value}, entity_id: {}, emitter_name: {}, .. }}",
                entity_id
                    .map(|x| format!("Some({x})"))
                    .unwrap_or("None".into()),
                emitter_name.map(|x| x.as_str()).unwrap_or("None")
            ),
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
                Command::PlaySceneDialog { .. } => "play scene dialog",
                Command::StopSceneDialog { .. } => "stop scene dialog",
                Command::StopOnEmitter { .. } => "stop on emitter",
                Command::StopVanilla { .. } => "stop vanilla",
                Command::Stop { .. } => "stop",
                Command::Switch { .. } => "switch",
                Command::SwitchVanilla { .. } => "switch vanilla",
                Command::EnqueueAndPlay { .. } => "enqueue and play",
                Command::EnqueueAndPlayOnEmitter { .. } => "enqueue and play on emitter",
            }
        )
    }
}
