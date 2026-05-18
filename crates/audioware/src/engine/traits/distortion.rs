use audioware_core::Decibels;
use kira::{Tween, effect::distortion::DistortionKind};

use crate::ControlId;

pub trait SetControlledDistortion {
    fn set_controlled_kind(&mut self, id: ControlId, kind: DistortionKind);
    fn set_controlled_drive(&mut self, id: ControlId, drive: Decibels, tween: Tween);
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween);
}
