use kira::{Tween, effect::filter::FilterMode};

use crate::ControlId;

pub trait SetControlledFilter {
    fn set_controlled_mode(&mut self, id: ControlId, mode: FilterMode);
    fn set_controlled_cutoff(&mut self, id: ControlId, cutoff: f32, tween: Tween);
    fn set_controlled_resonance(&mut self, id: ControlId, resonance: f32, tween: Tween);
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween);
}
