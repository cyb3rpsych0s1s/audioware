use kira::Tween;
use red4ext_rs::types::CName;

use crate::{
    Vector4,
    engine::{
        scene::dilation::Dilation,
        traits::{reclaim::Reclaim, stop::StopBy},
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
}

impl Reclaim for EmitterSlots {
    fn reclaim(&mut self) {
        self.slots.iter_mut().for_each(|x| x.handles.reclaim());
    }
}
