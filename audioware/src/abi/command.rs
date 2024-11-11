use audioware_manifest::{PlayerGender, ScnDialogLineType};
use red4ext_rs::types::{CName, EntityId, Opt};

use crate::types::{RedRef, Tween};

/// Sound inner command.
#[derive(Clone, Debug)]
pub enum Command {
    PlayVanilla {
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
    },
    #[allow(dead_code)]
    Play {
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        tween: RedRef<Tween>,
    },
    PlayExt {
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        // ext: RedRef<AudioSettingsExt>,
    },
    PlayOnEmitter {
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: RedRef<Tween>,
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
        tween: RedRef<Tween>,
    },
    Pause {
        tween: RedRef<Tween>,
    },
    Resume {
        tween: RedRef<Tween>,
    },
    StopVanilla {
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
    },
    Stop {
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: RedRef<Tween>,
    },
    #[allow(dead_code)]
    StopFor {
        entity_id: EntityId,
    },
    Switch {
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: RedRef<Tween>,
        // switch_value_settings: RedRef<AudioSettingsExt>,
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

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{self:?}")
        write!(f, "{}", match self {
            Command::PlayVanilla { .. } => "play vanilla",
            Command::Play { .. } => "play",
            Command::PlayExt { ..} => "play ext",
            Command::PlayOnEmitter { ..} => "play on emitter",
            Command::PlayOverThePhone { ..} => "play over the phone",
            Command::StopOnEmitter { ..} => "stop on emitter",
            Command::Pause { ..} => "pause",
            Command::Resume { ..} => "resume",
            Command::StopVanilla { ..} => "stop vanilla",
            Command::Stop { ..} => "stop",
            Command::StopFor { ..} => "stop for",
            Command::Switch { ..} => "switch",
            Command::SetVolume { ..} => "set volume",
            Command::SetPreset { ..} => "set preset",
            Command::SetReverbMix { ..} => "set reverb mix",
        })
    }
}
