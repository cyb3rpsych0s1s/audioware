use audioware_core::Decibels;
use kira::Tween;

use crate::ControlId;

pub trait SetControlledDelay {
    fn set_controlled_feedback(&mut self, id: ControlId, feedback: Decibels, tween: Tween);
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween);
}
