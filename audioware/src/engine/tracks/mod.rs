use ambience::Ambience;
use car_radio::CarRadio;
use dialogue::Dialogue;
use holocall::Holocall;
use kira::manager::{backend::Backend, AudioManager};
use music::Music;
use radioport::Radioport;
use sfx::Sfx;
use v::V;

use crate::error::Error;

use super::modulators::Modulators;

mod ambience;
mod car_radio;
mod dialogue;
mod holocall;
mod music;
mod radioport;
mod sfx;
mod v;

pub struct Tracks {
    // vanilla tracks
    pub sfx: Sfx,
    pub radioport: Radioport,
    pub music: Music,
    pub dialogue: Dialogue,
    pub car_radio: CarRadio,
    // audioware tracks
    pub v: V,
    pub holocall: Holocall,
    // tracks affected by reverb mix + preset (e.g. underwater)
    pub ambience: Ambience,
}

impl Tracks {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let ambience = Ambience::try_new(manager, modulators)?;
        let v = V::try_new(manager, &ambience, modulators)?;
        let holocall = Holocall::try_new(manager, modulators)?;
        let sfx = Sfx::try_new(manager, &ambience, modulators)?;
        let radioport = Radioport::try_new(manager, &ambience, modulators)?;
        let music = Music::try_new(manager, &ambience, modulators)?;
        let dialogue = Dialogue::try_new(manager, &ambience, modulators)?;
        let car_radio = CarRadio::try_new(manager, &ambience, modulators)?;
        Ok(Self {
            ambience,
            v,
            holocall,
            sfx,
            radioport,
            music,
            dialogue,
            car_radio,
        })
    }
}
