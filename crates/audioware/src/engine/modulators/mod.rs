use kira::{
    effect::EffectBuilder,
    manager::{backend::Backend, AudioManager},
    tween::Tween,
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
    pub sfx_volume: SfxVolume,
    pub dialogue_volume: DialogueVolume,
    pub music_volume: MusicVolume,
    pub car_radio_volume: CarRadioVolume,
    pub radioport_volume: RadioportVolume,
}

impl Modulators {
    pub fn try_new<B: Backend>(manager: &mut AudioManager<B>) -> Result<Self, Error> {
        let reverb_mix = ReverbMix::try_new(manager)?;
        let sfx_volume = SfxVolume::try_new(manager)?;
        let dialogue_volume = DialogueVolume::try_new(manager)?;
        let music_volume = MusicVolume::try_new(manager)?;
        let car_radio_volume = CarRadioVolume::try_new(manager)?;
        let radioport_volume = RadioportVolume::try_new(manager)?;
        Ok(Self {
            reverb_mix,
            sfx_volume,
            dialogue_volume,
            music_volume,
            car_radio_volume,
            radioport_volume,
        })
    }
}
