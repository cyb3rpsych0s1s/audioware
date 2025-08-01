use kira::{
    Tween,
    effect::EffectBuilder,
    {AudioManager, backend::Backend},
};

use crate::error::Error;

mod volume;
pub use volume::*;
mod reverb;
pub use reverb::*;

pub trait Parameter {
    type Value;
    fn try_new<B: Backend>(manager: &mut AudioManager<B>) -> Result<Self, Error>
    where
        Self: Sized;
    fn try_effect(&self) -> Result<impl EffectBuilder, Error>;
    fn update(&mut self, value: Self::Value, tween: Tween);
}

pub struct Modulators {
    pub reverb_mix: ReverbMix,
}

impl Modulators {
    pub fn try_new<B: Backend>(manager: &mut AudioManager<B>) -> Result<Self, Error> {
        let reverb_mix = ReverbMix::try_new(manager)?;
        Ok(Self { reverb_mix })
    }
}
