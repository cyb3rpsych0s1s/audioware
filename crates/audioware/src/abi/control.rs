use audioware_core::Amplitude;
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
}

impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Control::SetVolume { id, .. } => write!(f, "set dynamic sound volume ({id})"),
        }
    }
}
