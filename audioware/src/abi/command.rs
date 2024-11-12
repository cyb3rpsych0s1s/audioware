use audioware_manifest::{PlayerGender, ScnDialogLineType};
use kira::tween::Tween;
use red4ext_rs::types::{CName, EntityId};

use super::AudioSettingsExt;

/// Sound inner command.
#[derive(Clone)]
pub enum Command {
    PlayVanilla {
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    },
    #[allow(dead_code)]
    Play {
        sound_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
        tween: Option<Tween>,
    },
    PlayExt {
        sound_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
        ext: Option<AudioSettingsExt>,
    },
    PlayOnEmitter {
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Option<Tween>,
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
    Pause {
        tween: Option<Tween>,
    },
    Resume {
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
    #[allow(dead_code)]
    StopFor {
        entity_id: EntityId,
    },
    Switch {
        switch_name: CName,
        switch_value: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        switch_name_tween: Option<Tween>,
        // switch_value_settings: Option<AudioSettingsExt>,
    },
    SetVolume {
        setting: CName,
        value: f64,
    },
    SetPreset {
        // value: Preset,
    },
    SetReverbMix {
        value: f32,
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
            Command::PlayExt {
                sound_name,
                entity_id,
                emitter_name,
                line_type,
                ..
            } => write!(f, "Command::PlayExt {{ sound_name: {:?}, entity_id: {:?}, emitter_name: {:?}, line_type: {:?}, .. }}", sound_name, entity_id, emitter_name, line_type),
            Command::PlayOnEmitter {
                sound_name,
                entity_id,
                emitter_name,
                ..
            } => write!(f, "Command::PlayOnEmitter {{ sound_name: {:?}, entity_id: {:?}, emitter_name: {:?}, .. }}", sound_name, entity_id, emitter_name),
            Command::StopOnEmitter {
                event_name,
                entity_id,
                emitter_name,
                ..
            } => write!(f, "Command::StopOnEmitter {{ event_name: {:?}, entity_id: {:?}, emitter_name: {:?}, .. }}", event_name, entity_id, emitter_name),
            Command::Pause { .. } => write!(f, "Command::Pause {{ .. }}"),
            Command::Resume { .. } => write!(f, "Command::Resume {{ .. }}"),
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
                Command::PlayExt { .. } => "play ext",
                Command::PlayOnEmitter { .. } => "play on emitter",
                Command::PlayOverThePhone { .. } => "play over the phone",
                Command::StopOnEmitter { .. } => "stop on emitter",
                Command::Pause { .. } => "pause",
                Command::Resume { .. } => "resume",
                Command::StopVanilla { .. } => "stop vanilla",
                Command::Stop { .. } => "stop",
                Command::StopFor { .. } => "stop for",
                Command::Switch { .. } => "switch",
                Command::SetVolume { .. } => "set volume",
                Command::SetPreset { .. } => "set preset",
                Command::SetReverbMix { .. } => "set reverb mix",
            }
        )
    }
}
