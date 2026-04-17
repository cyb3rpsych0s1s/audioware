use audioware_core::Decibels;
use kira::{Tween, effect::eq_filter::EqFilterKind};

use crate::ControlId;

pub trait SetControlledEq {
    fn set_controlled_kind(&mut self, id: ControlId, kind: EqFilterKind);
    fn set_controlled_frequency(&mut self, id: ControlId, frequency: f64, tween: Tween);
    fn set_controlled_gain(&mut self, id: ControlId, gain: Decibels, tween: Tween);
    fn set_controlled_q(&mut self, id: ControlId, q: f64, tween: Tween);
}
