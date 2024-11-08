use std::sync::OnceLock;

use ambience::Ambience;
use car_radio::CarRadio;
use dialogue::Dialogue;
use holocall::Holocall;
use kira::{manager::AudioManager, OutputDestination};
use music::Music;
use radioport::Radioport;
use sfx::Sfx;
use v::V;

use crate::error::Error;

mod ambience;

mod car_radio;
mod dialogue;
mod music;
mod radioport;
mod sfx;

mod holocall;
mod v;

static TRACKS: OnceLock<Tracks> = OnceLock::new();

pub struct Tracks {
    // tracks affected by reverb mix + preset (e.g. underwater)
    pub ambience: Ambience,
    // audioware tracks
    pub v: V,
    pub holocall: Holocall,
    // vanilla tracks
    pub sfx: Sfx,
    pub radioport: Radioport,
    pub music: Music,
    pub dialogue: Dialogue,
    pub car_radio: CarRadio,
}

impl Tracks {
    pub(super) fn try_new(manager: &mut AudioManager) -> Result<Self, Error> {
        let ambience = Ambience::setup(manager)?;

        let sfx = Sfx::setup(manager, &ambience)?;
        let radioport = Radioport::setup(manager)?;
        let music = Music::setup(manager)?;
        let dialogue = Dialogue::setup(manager, &ambience)?;
        let car_radio = CarRadio::setup(manager)?;

        let v = V::setup(manager, &ambience)?;
        let holocall = Holocall::setup(manager)?;
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
    pub fn holocall_destination(&self) -> OutputDestination {
        (&self.holocall).into()
    }
}
