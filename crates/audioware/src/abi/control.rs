use std::time::Duration;

use audioware_core::{Amplitude, Panning};
use crossbeam::channel::{Sender, bounded};
use humantime::format_duration;
use kira::Tween;
use red4ext_rs::types::{CName, EntityId, Ref};

use crate::{
    ControlId, ToTween,
    abi::{DynamicEmitterEvent, TargetId, command::Command, types::DynamicSoundEvent},
    engine::{next_control_id, queue},
    utils::warns,
};

#[derive(Debug)]
pub enum DynamicSound {
    SetVolume {
        id: ControlId,
        value: Amplitude,
        tween: Option<Tween>,
    },
    SetPlaybackRate {
        id: ControlId,
        value: f64,
        tween: Option<Tween>,
    },
    SetPanning {
        id: ControlId,
        value: kira::Panning,
        tween: Option<Tween>,
    },
    Position {
        id: ControlId,
        output: Sender<f32>,
    },
    Stop {
        id: ControlId,
        tween: Option<Tween>,
    },
    Pause {
        id: ControlId,
        tween: Option<Tween>,
    },
    Resume {
        id: ControlId,
        tween: Option<Tween>,
    },
    ResumeAt {
        id: ControlId,
        value: f64,
        tween: Option<Tween>,
    },
    SeekTo {
        id: ControlId,
        value: f64,
    },
    SeekBy {
        id: ControlId,
        value: f64,
    },
}

#[derive(Debug)]
pub enum DynamicEmitter {
    SetVolume {
        id: ControlId,
        value: Amplitude,
        tween: Option<Tween>,
    },
    SetPlaybackRate {
        id: ControlId,
        value: f64,
        tween: Option<Tween>,
    },
    Position {
        id: ControlId,
        output: Sender<f32>,
    },
    Stop {
        id: ControlId,
        tween: Option<Tween>,
    },
    Pause {
        id: ControlId,
        tween: Option<Tween>,
    },
    Resume {
        id: ControlId,
        tween: Option<Tween>,
    },
    ResumeAt {
        id: ControlId,
        value: f64,
        tween: Option<Tween>,
    },
    SeekTo {
        id: ControlId,
        value: f64,
    },
    SeekBy {
        id: ControlId,
        value: f64,
    },
}

impl DynamicSoundEvent {
    pub fn enqueue_and_play(
        &self,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    ) -> bool {
        if let Err(control_id) = self.id.set(next_control_id()) {
            warns!("dynamic sound already initialized ({control_id})");
            return false;
        }
        queue::send(Command::EnqueueAndPlay {
            event_name: *self.name.get(),
            entity_id,
            emitter_name,
            line_type: None,
            ext: self.ext.borrow().clone(),
            control_id: *self.id.get().unwrap(),
        });
        true
    }
    pub fn set_volume(&self, value: f32, tween: Ref<crate::Tween>) {
        let Ok(value) = Amplitude::try_from(value) else {
            warns!("invalid amplitude ({value})");
            return;
        };
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::SetVolume {
            id: *self.id.get().unwrap(),
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_playback_rate(&self, value: f32, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::SetPlaybackRate {
            id: *self.id.get().unwrap(),
            value: value as f64,
            tween: tween.into_tween(),
        });
    }
    pub fn set_panning(&self, value: f32, tween: Ref<crate::Tween>) {
        let Ok(value) = Panning::try_from(value) else {
            warns!("invalid panning ({value})");
            return;
        };
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::SetPanning {
            id: *self.id.get().unwrap(),
            value: *value,
            tween: tween.into_tween(),
        });
    }
    pub fn position(&self) -> f32 {
        if self.id.get().is_none() {
            return -1.;
        }
        let (s, r) = bounded(0);
        queue::control_sound(DynamicSound::Position {
            id: *self.id.get().unwrap(),
            output: s,
        });
        r.recv_timeout(Duration::from_millis(30)).unwrap_or(-1.)
    }
    pub fn stop(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::Stop {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn pause(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::Pause {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn resume(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::Resume {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn resume_at(&self, value: f32, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::ResumeAt {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
            value: value.into(),
        });
    }
    pub fn seek_to(&self, value: f32) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::SeekTo {
            id: *self.id.get().unwrap(),
            value: value.into(),
        });
    }
    pub fn seek_by(&self, value: f32) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_sound(DynamicSound::SeekBy {
            id: *self.id.get().unwrap(),
            value: value.into(),
        });
    }
}

impl std::fmt::Display for DynamicSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicSound::SetVolume { id, .. } => write!(f, "set dynamic sound volume ({id})"),
            DynamicSound::SetPlaybackRate { id, .. } => {
                write!(f, "set dynamic sound playback rate ({id})")
            }
            DynamicSound::SetPanning { id, .. } => {
                write!(f, "set dynamic sound panning ({id})")
            }
            DynamicSound::Pause { id, .. } => {
                write!(f, "pause dynamic sound ({id})")
            }
            DynamicSound::Resume { id, .. } => {
                write!(f, "resume dynamic sound ({id})")
            }
            DynamicSound::ResumeAt { id, value, .. } => {
                write!(
                    f,
                    "resume dynamic sound at {} ({id})",
                    format_duration(Duration::from_secs_f64(*value))
                )
            }
            DynamicSound::Stop { id, .. } => write!(f, "stop dynamic sound ({id})"),
            DynamicSound::SeekTo { id, value } => write!(
                f,
                "seek dynamic sound to {} ({id})",
                format_duration(Duration::from_secs_f64(*value))
            ),
            DynamicSound::SeekBy { id, value } => write!(
                f,
                "seek dynamic sound by {} ({id})",
                format_duration(Duration::from_secs_f64(*value))
            ),
            DynamicSound::Position { id, .. } => write!(f, "get dynamic sound position ({id})"),
        }
    }
}

impl DynamicEmitterEvent {
    pub fn enqueue_and_play(&self, entity_id: EntityId) -> bool {
        if let Err(control_id) = self.id.set(next_control_id()) {
            warns!(
                "dynamic emitter already initialized for {} ({control_id})",
                self.tag_name.get()
            );
            return false;
        }
        let Ok(entity_id) = TargetId::try_new(entity_id) else {
            warns!("invalid entity id: {entity_id}");
            return false;
        };
        queue::send(Command::EnqueueAndPlayOnEmitter {
            event_name: *self.name.get(),
            entity_id,
            tag_name: self.tag_name.get(),
            ext: self.ext.borrow().clone(),
            control_id: *self.id.get().unwrap(),
        });
        true
    }
    pub fn set_volume(&self, value: f32, tween: Ref<crate::Tween>) {
        let Ok(value) = Amplitude::try_from(value) else {
            warns!("invalid amplitude ({value})");
            return;
        };
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::SetVolume {
            id: *self.id.get().unwrap(),
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_playback_rate(&self, value: f32, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::SetPlaybackRate {
            id: *self.id.get().unwrap(),
            value: value as f64,
            tween: tween.into_tween(),
        });
    }
    pub fn stop(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::Stop {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn pause(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::Pause {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn resume(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::Resume {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn resume_at(&self, value: f32, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::ResumeAt {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
            value: value.into(),
        });
    }
    pub fn position(&self) -> f32 {
        if self.id.get().is_none() {
            return -1.;
        }
        let (s, r) = bounded(0);
        queue::control_emitter(DynamicEmitter::Position {
            id: *self.id.get().unwrap(),
            output: s,
        });
        r.recv_timeout(Duration::from_millis(30)).unwrap_or(-1.)
    }
    pub fn seek_to(&self, value: f32) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::SeekTo {
            id: *self.id.get().unwrap(),
            value: value.into(),
        });
    }
    pub fn seek_by(&self, value: f32) {
        if self.id.get().is_none() {
            return;
        }
        queue::control_emitter(DynamicEmitter::SeekBy {
            id: *self.id.get().unwrap(),
            value: value.into(),
        });
    }
}

impl std::fmt::Display for DynamicEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicEmitter::SetVolume { id, .. } => write!(f, "set dynamic emitter volume ({id})"),
            DynamicEmitter::SetPlaybackRate { id, .. } => {
                write!(f, "set dynamic emitter playback rate ({id})")
            }
            DynamicEmitter::Pause { id, .. } => {
                write!(f, "pause dynamic emitter ({id})")
            }
            DynamicEmitter::Resume { id, .. } => {
                write!(f, "resume dynamic emitter ({id})")
            }
            DynamicEmitter::ResumeAt { id, value, .. } => {
                write!(
                    f,
                    "resume dynamic emitter at {} ({id})",
                    format_duration(Duration::from_secs_f64(*value))
                )
            }
            DynamicEmitter::Stop { id, .. } => write!(f, "stop dynamic emitter ({id})"),
            DynamicEmitter::SeekTo { id, value } => write!(
                f,
                "seek dynamic emitter to {} ({id})",
                format_duration(Duration::from_secs_f64(*value))
            ),
            DynamicEmitter::SeekBy { id, value } => write!(
                f,
                "seek dynamic emitter by {} ({id})",
                format_duration(Duration::from_secs_f64(*value))
            ),
            DynamicEmitter::Position { id, .. } => write!(f, "get dynamic emitter position ({id})"),
        }
    }
}
