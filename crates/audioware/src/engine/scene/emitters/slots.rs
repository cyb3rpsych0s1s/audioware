use kira::Tween;
use red4ext_rs::types::CName;

use crate::{
    ControlId, Vector4,
    engine::{
        scene::dilation::Dilation,
        traits::{
            pause::PauseControlled,
            playback::SetControlledPlaybackRate,
            position::PositionControlled,
            reclaim::Reclaim,
            resume::{ResumeControlled, ResumeControlledAt},
            seek::{SeekControlledBy, SeekControlledTo},
            stop::{StopBy, StopControlled},
            volume::SetControlledVolume,
        },
        tweens::IMMEDIATELY,
    },
};

use super::slot::EmitterSlot;

/// Identify active [EmitterSlot] handles.
/// These handles can be shared by multiple mods.
pub struct EmitterSlots {
    pub slots: Vec<EmitterSlot>,
    pub marked_for_death: bool,
    pub busy: bool,
    pub last_known_position: Vector4,
    pub dilation: Dilation,
}

impl EmitterSlots {
    pub fn new(
        slot: EmitterSlot,
        dilation: Option<f32>,
        busy: bool,
        last_known_position: Vector4,
    ) -> Self {
        Self {
            slots: vec![slot],
            marked_for_death: false,
            busy,
            last_known_position,
            dilation: Dilation::new(dilation.unwrap_or(1.0)),
        }
    }
    pub fn insert(&mut self, slot: EmitterSlot) {
        self.slots.push(slot);
    }
    pub fn exists_tag(&self, tag_name: &CName) -> bool {
        self.slots.iter().any(|x| x.tag_name == Some(*tag_name))
    }
    pub fn any_playing_handle(&self) -> bool {
        self.slots.iter().any(|x| x.any_playing_handle())
    }
    pub fn set_emitter_position(&mut self, position: Vector4) {
        self.slots.iter_mut().for_each(|x| {
            x.handle.set_position(position, IMMEDIATELY);
        });
    }
    pub fn set_emitter_occlusion(&mut self, factor: f32) {
        self.slots.iter_mut().for_each(|x| {
            x.handle.set_occlusion(factor);
        });
    }
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn stop_on_emitter(&mut self, event_name: CName, tag_name: CName, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            if x.tag_name == Some(tag_name) {
                x.handles.stop_by(&event_name, tween);
            }
        });
    }

    pub fn stop(&mut self, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.stop(tween);
        });
    }

    pub fn pause(&mut self, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.pause(tween);
        });
    }

    pub fn resume(&mut self, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.resume(tween);
        });
    }

    pub fn unregister_emitter(&mut self, tag_name: &CName) -> bool {
        let before = self.slots.len();
        self.slots.retain(|x| x.tag_name != Some(*tag_name));
        before != self.slots.len()
    }
    pub fn get_mut<'a>(&'a mut self, tag_name: &CName) -> Option<&'a mut EmitterSlot> {
        self.slots
            .iter_mut()
            .find(|x| x.tag_name == Some(*tag_name))
    }
    pub fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.sync_dilation(rate, tween);
        });
    }
    pub fn any_occluded(&self) -> bool {
        self.slots.iter().any(|x| x.occluded())
    }
}

impl Reclaim for EmitterSlots {
    fn reclaim(&mut self) {
        self.slots.iter_mut().for_each(|x| x.handles.reclaim());
    }
}

impl SetControlledVolume for EmitterSlots {
    fn set_controlled_volume(
        &mut self,
        id: ControlId,
        amplitude: audioware_core::Amplitude,
        tween: Tween,
    ) {
        self.slots
            .iter_mut()
            .for_each(|x| x.handles.set_controlled_volume(id, amplitude, tween));
    }
}

impl SetControlledPlaybackRate for EmitterSlots {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.set_controlled_playback_rate(id, rate, tween);
        })
    }
}

impl PositionControlled for EmitterSlots {
    fn position_controlled(&mut self, id: ControlId, sender: crossbeam::channel::Sender<f32>) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.position_controlled(id, sender.clone());
        })
    }
}

impl StopControlled for EmitterSlots {
    fn stop_controlled(&mut self, id: ControlId, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.stop_controlled(id, tween);
        })
    }
}

impl PauseControlled for EmitterSlots {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.pause_controlled(id, tween);
        })
    }
}

impl ResumeControlled for EmitterSlots {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.resume_controlled(id, tween);
        })
    }
}

impl ResumeControlledAt for EmitterSlots {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.resume_controlled_at(id, delay, tween);
        })
    }
}

impl SeekControlledTo for EmitterSlots {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.seek_controlled_to(id, position);
        })
    }
}

impl SeekControlledBy for EmitterSlots {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.slots.iter_mut().for_each(|x| {
            x.handles.seek_controlled_by(id, amount);
        })
    }
}
