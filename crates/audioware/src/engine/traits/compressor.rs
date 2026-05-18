use std::time::Duration;

use audioware_core::Decibels;
use kira::Tween;

use crate::ControlId;

pub trait SetControlledCompressor {
    fn set_controlled_threshold(&mut self, id: ControlId, threshold: Decibels, tween: Tween);
    fn set_controlled_ratio(&mut self, id: ControlId, ratio: f32, tween: Tween);
    fn set_controlled_attack_duration(
        &mut self,
        id: ControlId,
        attack_duration: Duration,
        tween: Tween,
    );
    fn set_controlled_release_duration(
        &mut self,
        id: ControlId,
        release_duration: Duration,
        tween: Tween,
    );
    fn set_controlled_makeup_gain(&mut self, id: ControlId, makeup_gain: Decibels, tween: Tween);
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween);
}
