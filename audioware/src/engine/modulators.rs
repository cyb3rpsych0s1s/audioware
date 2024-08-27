use kira::{effect::EffectBuilder, manager::AudioManager, tween::Tween};

use crate::error::Error;

mod volume;
pub use volume::*;
mod reverb;
pub use reverb::*;

pub trait Parameter {
    type Value;
    fn setup(manager: &mut AudioManager) -> Result<(), Error>;
    fn effect() -> Result<impl EffectBuilder, Error>;
    fn update(value: Self::Value, tween: Tween) -> Result<bool, Error>;
}

pub struct Modulators;

impl Modulators {
    pub(super) fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        ReverbMix::setup(manager)?;
        SfxVolume::setup(manager)?;
        DialogueVolume::setup(manager)?;
        MusicVolume::setup(manager)?;
        CarRadioVolume::setup(manager)?;
        RadioportVolume::setup(manager)?;
        Ok(())
    }
}
