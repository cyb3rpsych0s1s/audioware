use kira::Tween;
use red4ext_rs::types::CName;

use crate::{
    ControlId, Vector4,
    engine::{
        scene::dilation::Dilation,
        traits::{
            compressor::SetControlledCompressor,
            delay::SetControlledDelay,
            distortion::SetControlledDistortion,
            eq::SetControlledEq,
            filter::SetControlledFilter,
            pause::PauseControlled,
            playback::SetControlledPlaybackRate,
            position::PositionControlled,
            reclaim::Reclaim,
            resume::{ResumeControlled, ResumeControlledAt},
            reverb::SetControlledReverb,
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

impl SetControlledEq for EmitterSlots {
    fn set_controlled_kind(&mut self, id: ControlId, kind: kira::effect::eq_filter::EqFilterKind) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledEq::set_controlled_kind(x, id, kind);
        });
    }

    fn set_controlled_frequency(&mut self, id: ControlId, frequency: f64, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_frequency(id, frequency, tween);
        });
    }

    fn set_controlled_gain(&mut self, id: ControlId, gain: audioware_core::Decibels, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_gain(id, gain, tween);
        });
    }

    fn set_controlled_q(&mut self, id: ControlId, q: f64, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_q(id, q, tween);
        });
    }
}

impl SetControlledDistortion for EmitterSlots {
    fn set_controlled_kind(
        &mut self,
        id: ControlId,
        kind: kira::effect::distortion::DistortionKind,
    ) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledDistortion::set_controlled_kind(x, id, kind);
        });
    }

    fn set_controlled_drive(
        &mut self,
        id: ControlId,
        drive: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_drive(id, drive, tween);
        });
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledDistortion::set_controlled_mix(x, id, mix, tween);
        });
    }
}

impl SetControlledDelay for EmitterSlots {
    fn set_controlled_feedback(
        &mut self,
        id: ControlId,
        feedback: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledDelay::set_controlled_feedback(x, id, feedback, tween);
        });
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledDelay::set_controlled_mix(x, id, mix, tween);
        });
    }
}

impl SetControlledCompressor for EmitterSlots {
    fn set_controlled_threshold(
        &mut self,
        id: ControlId,
        threshold: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_threshold(id, threshold, tween);
        });
    }

    fn set_controlled_ratio(&mut self, id: ControlId, ratio: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_ratio(id, ratio, tween);
        });
    }

    fn set_controlled_attack_duration(
        &mut self,
        id: ControlId,
        attack_duration: std::time::Duration,
        tween: Tween,
    ) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_attack_duration(id, attack_duration, tween);
        });
    }

    fn set_controlled_release_duration(
        &mut self,
        id: ControlId,
        release_duration: std::time::Duration,
        tween: Tween,
    ) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_release_duration(id, release_duration, tween);
        });
    }

    fn set_controlled_makeup_gain(
        &mut self,
        id: ControlId,
        makeup_gain: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_makeup_gain(id, makeup_gain, tween);
        });
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledCompressor::set_controlled_mix(x, id, mix, tween);
        });
    }
}

impl SetControlledFilter for EmitterSlots {
    fn set_controlled_mode(&mut self, id: ControlId, mode: kira::effect::filter::FilterMode) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_mode(id, mode);
        });
    }

    fn set_controlled_cutoff(&mut self, id: ControlId, cutoff: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_cutoff(id, cutoff, tween);
        });
    }

    fn set_controlled_resonance(&mut self, id: ControlId, resonance: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_resonance(id, resonance, tween);
        });
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledFilter::set_controlled_mix(x, id, mix, tween);
        });
    }
}

impl SetControlledReverb for EmitterSlots {
    fn set_controlled_feedback(&mut self, id: ControlId, feedback: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledReverb::set_controlled_feedback(x, id, feedback, tween);
        });
    }

    fn set_controlled_damping(&mut self, id: ControlId, damping: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_damping(id, damping, tween);
        });
    }

    fn set_controlled_stereo_width(&mut self, id: ControlId, stereo_width: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            x.set_controlled_stereo_width(id, stereo_width, tween);
        });
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        self.slots.iter_mut().for_each(|x| {
            SetControlledReverb::set_controlled_mix(x, id, mix, tween);
        });
    }
}
