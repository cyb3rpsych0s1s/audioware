use kira::Tween;

use crate::ControlId;

pub trait SetControlledReverb {
    fn set_controlled_feedback(&mut self, id: ControlId, feedback: f32, tween: Tween);
    fn set_controlled_damping(&mut self, id: ControlId, damping: f32, tween: Tween);
    fn set_controlled_stereo_width(&mut self, id: ControlId, stereo_width: f32, tween: Tween);
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween);
}
