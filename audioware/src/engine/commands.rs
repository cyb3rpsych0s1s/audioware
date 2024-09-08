//! Sound commands, e.g. `Play`, `Stop`, etc.

use audioware_manifest::{PlayerGender, ScnDialogLineType};
use red4ext_rs::types::{CName, EntityId, Opt, Ref};

use crate::types::{EmitterSettings, RedRef, Tween};

use super::{AudioSettingsExt, Engine, Preset};

pub trait OuterCommandOps {
    /// Whether command can be cancelled or not.
    fn cancelable(&self) -> bool;
    /// Consume and return inner command.
    fn into_inner(self) -> Command;
}
pub(crate) trait CommandOps {
    fn execute(self);
}

/// Command requested by game and crate users alike.
#[derive(Clone)]
pub(super) enum OuterCommand {
    /// Command which can be cancelled if ever there's any contention.
    Cancellable(Command),
    /// Command which should be executed no matter contention.
    NonCancellable(Command),
}

impl OuterCommandOps for OuterCommand {
    fn cancelable(&self) -> bool {
        match self {
            Self::Cancellable(_) => true,
            Self::NonCancellable(_) => false,
        }
    }

    fn into_inner(self) -> Command {
        match self {
            Self::Cancellable(command) | Self::NonCancellable(command) => command,
        }
    }
}

impl CommandOps for Command {
    fn execute(self) {
        match self {
            Command::PlayVanilla {
                event_name,
                entity_id,
                emitter_name,
            } => Engine::play(
                event_name,
                entity_id,
                emitter_name,
                Opt::Default,
                Ref::default(),
            ),
            Command::Play {
                sound_name,
                entity_id,
                emitter_name,
                line_type,
                tween,
            } => Engine::play(sound_name, entity_id, emitter_name, line_type, tween.into()),
            Command::PlayExt {
                sound_name,
                entity_id,
                emitter_name,
                line_type,
                ext,
            } => Engine::play_with(sound_name, entity_id, emitter_name, line_type, ext.into()),
            Command::PlayOnEmitter {
                sound_name,
                entity_id,
                emitter_name,
                tween,
            } => Engine::play_on_emitter(sound_name, entity_id, emitter_name, tween.into()),
            Command::PlayOverThePhone {
                event_name,
                emitter_name,
                gender,
            } => Engine::play_over_the_phone(
                event_name,
                emitter_name,
                if gender == PlayerGender::Male {
                    CName::new("Male")
                } else {
                    CName::new("Female")
                },
            ),
            Command::StopOnEmitter {
                event_name,
                entity_id,
                emitter_name,
                tween,
            } => Engine::stop_on_emitter(event_name, entity_id, emitter_name, tween.into()),
            Command::Pause { tween } => Engine::pause(tween.into()),
            Command::Resume { tween } => Engine::resume(tween.into()),
            Command::StopVanilla {
                event_name,
                entity_id,
                emitter_name,
            } => Engine::stop(event_name, entity_id, emitter_name, Ref::default()),
            Command::Stop {
                event_name,
                entity_id,
                emitter_name,
                tween,
            } => Engine::stop(event_name, entity_id, emitter_name, tween.into()),
            Command::StopFor { entity_id } => Engine::stop_for(entity_id),
            Command::Switch {
                switch_name,
                switch_value,
                entity_id,
                emitter_name,
                switch_name_tween,
                switch_value_settings,
            } => Engine::switch(
                switch_name,
                switch_value,
                entity_id,
                emitter_name,
                switch_name_tween.into(),
                switch_value_settings.into(),
            ),
            Command::SetPreset { value } => Engine::set_preset(value),
            Command::SetReverbMix { value } => Engine::set_reverb_mix(value),
            Command::SetVolume { setting, value } => Engine::set_volume(setting, value),
        }
    }
}

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
        ext: RedRef<AudioSettingsExt>,
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
        switch_value_settings: RedRef<AudioSettingsExt>,
    },
    SetVolume {
        setting: CName,
        value: f64,
    },
    SetPreset {
        value: Preset,
    },
    SetReverbMix {
        value: f32,
    },
}

impl Command {
    pub(super) fn cancelable(self) -> OuterCommand {
        OuterCommand::Cancellable(self)
    }
    pub(super) fn non_cancelable(self) -> OuterCommand {
        OuterCommand::NonCancellable(self)
    }
}

#[derive(Debug)]
pub enum Lifecycle {
    RegisterEmitter {
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
    },
    UnregisterEmitter {
        entity_id: EntityId,
    },
    SyncScene,
    Reclaim,
    Shutdown,
    Terminate,
}

impl CommandOps for Lifecycle {
    fn execute(self) {
        match self {
            Lifecycle::RegisterEmitter {
                entity_id,
                emitter_name,
                emitter_settings,
            } => {
                Engine::register_emitter(entity_id, emitter_name, emitter_settings);
            }
            Lifecycle::UnregisterEmitter { entity_id } => {
                Engine::unregister_emitter(entity_id);
            }
            Lifecycle::SyncScene => {
                Engine::sync_listener();
                Engine::sync_emitters();
            }
            Lifecycle::Reclaim => Engine::reclaim(),
            Lifecycle::Shutdown => Engine::shutdown(),
            Lifecycle::Terminate => Engine::terminate(),
        }
    }
}
