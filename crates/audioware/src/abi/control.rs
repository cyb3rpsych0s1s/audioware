use std::time::Duration;

use audioware_core::{Amplitude, Panning};
use crossbeam::channel::{Sender, bounded};
use humantime::format_duration;
use kira::Tween;
use red4ext_rs::types::Ref;

use crate::{
    ControlId, ToTween,
    abi::{command::Command, types::DynamicSoundEvent},
    engine::{next_control_id, queue},
    utils::warns,
};

#[derive(Debug)]
pub enum Control {
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

impl DynamicSoundEvent {
    pub fn enqueue_and_play(&self) -> bool {
        if let Err(control_id) = self.id.set(next_control_id()) {
            warns!("dynamic sound already initialized ({control_id})");
            return false;
        }
        queue::send(Command::EnqueueAndPlay {
            event_name: *self.name.get(),
            entity_id: None,
            emitter_name: None,
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
        queue::control(Control::SetVolume {
            id: *self.id.get().unwrap(),
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_playback_rate(&self, value: f32, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::SetPlaybackRate {
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
        queue::control(Control::SetPanning {
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
        queue::control(Control::Position {
            id: *self.id.get().unwrap(),
            output: s,
        });
        r.recv_timeout(Duration::from_millis(30)).unwrap_or(-1.)
    }
    pub fn stop(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::Stop {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn pause(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::Pause {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn resume(&self, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::Resume {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
        });
    }
    pub fn resume_at(&self, value: f32, tween: Ref<crate::Tween>) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::ResumeAt {
            id: *self.id.get().unwrap(),
            tween: tween.into_tween(),
            value: value.into(),
        });
    }
    pub fn seek_to(&self, value: f32) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::SeekTo {
            id: *self.id.get().unwrap(),
            value: value.into(),
        });
    }
    pub fn seek_by(&self, value: f32) {
        if self.id.get().is_none() {
            return;
        }
        queue::control(Control::SeekBy {
            id: *self.id.get().unwrap(),
            value: value.into(),
        });
    }
}

impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Control::SetVolume { id, .. } => write!(f, "set dynamic sound volume ({id})"),
            Control::SetPlaybackRate { id, .. } => {
                write!(f, "set dynamic sound playback rate ({id})")
            }
            Control::SetPanning { id, .. } => {
                write!(f, "set dynamic sound panning ({id})")
            }
            Control::Pause { id, .. } => {
                write!(f, "pause dynamic sound ({id})")
            }
            Control::Resume { id, .. } => {
                write!(f, "resume dynamic sound ({id})")
            }
            Control::ResumeAt { id, value, .. } => {
                write!(
                    f,
                    "resume dynamic sound at {} ({id})",
                    format_duration(Duration::from_secs_f64(*value))
                )
            }
            Control::Stop { id, .. } => write!(f, "stop dynamic sound ({id})"),
            Control::SeekTo { id, value } => write!(
                f,
                "seek dynamic sound to {} ({id})",
                format_duration(Duration::from_secs_f64(*value))
            ),
            Control::SeekBy { id, value } => write!(
                f,
                "seek dynamic sound by {} ({id})",
                format_duration(Duration::from_secs_f64(*value))
            ),
            Control::Position { id, .. } => write!(f, "get dynamic sound position ({id})"),
        }
    }
}
