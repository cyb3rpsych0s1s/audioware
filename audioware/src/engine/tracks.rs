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

use crate::error::{Error, InternalError};

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
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        let ambience = Ambience::setup(manager)?;

        let sfx = Sfx::setup(manager, &ambience)?;
        let radioport = Radioport::setup(manager)?;
        let music = Music::setup(manager)?;
        let dialogue = Dialogue::setup(manager, &ambience)?;
        let car_radio = CarRadio::setup(manager)?;

        let v = V::setup(manager, &ambience)?;
        let holocall = Holocall::setup(manager)?;

        TRACKS
            .set(Tracks {
                ambience,
                sfx,
                radioport,
                music,
                dialogue,
                car_radio,
                v,
                holocall,
            })
            .map_err(|_| {
                Error::from(InternalError::Init {
                    origin: "main tracks",
                })
            })?;

        Ok(())
    }
    pub fn get() -> &'static Tracks {
        TRACKS.get().expect("tracks should be initialized")
    }
    pub fn holocall_destination() -> OutputDestination {
        (&Tracks::get().holocall).into()
    }
}
