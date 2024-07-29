use std::sync::{Mutex, OnceLock};

use holocall::Holocall;
use kira::{
    effect::reverb::ReverbBuilder,
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};
use v::V;

use crate::error::{Error, InternalError};

use super::modulators::{Parameter, VolumeModulator};

mod holocall;
mod v;

static TRACKS: OnceLock<Tracks> = OnceLock::new();

pub struct Tracks {
    pub reverb: Mutex<TrackHandle>,
    pub v: V,
    pub holocall: Holocall,
}

impl Tracks {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        VolumeModulator::setup(manager)?;
        // TODO: AmbienceTrack::init(manager)?;

        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().mix(1.0));
            builder
        })?;

        let v = V::setup(manager, &reverb)?;
        let holocall = Holocall::setup(manager)?;

        TRACKS
            .set(Tracks {
                reverb: Mutex::new(reverb),
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
        (&Tracks::get().holocall.main).into()
    }
}
