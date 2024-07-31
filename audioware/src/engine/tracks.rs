use std::sync::OnceLock;

use car_radio::CarRadio;
use dialogue::Dialogue;
use environment::Environment;
use holocall::Holocall;
use kira::{manager::AudioManager, OutputDestination};
use music::Music;
use radioport::Radioport;
use sfx::Sfx;
use v::V;

use crate::error::{Error, InternalError};

use super::modulators::Parameter;

mod environment;

mod car_radio;
mod dialogue;
mod music;
mod radioport;
mod sfx;

mod holocall;
mod v;

static TRACKS: OnceLock<Tracks> = OnceLock::new();

pub struct Tracks {
    // should be renamed 'environment' ?
    // tracks affected by reverb mix + preset (underwater)
    #[allow(dead_code, reason = "reverb track handle must be held")]
    pub environment: Environment,
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
        let environment = Environment::setup(manager)?;

        let sfx = Sfx::setup(manager, environment.reverb())?;
        let radioport = Radioport::setup(manager)?;
        let music = Music::setup(manager)?;
        let dialogue = Dialogue::setup(manager, environment.reverb())?;
        let car_radio = CarRadio::setup(manager)?;

        let v = V::setup(manager, environment.reverb())?;
        let holocall = Holocall::setup(manager)?;

        TRACKS
            .set(Tracks {
                environment,
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
        TRACKS.get().unwrap()
    }
    pub fn holocall_destination() -> OutputDestination {
        (&Tracks::get().holocall).into()
    }
}
