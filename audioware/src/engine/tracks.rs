use std::sync::OnceLock;

use ambience::Ambience;
use holocall::Holocall;
use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};
use v::V;

use crate::error::{Error, InternalError};

use super::modulators::{Parameter, ReverbMix};

mod ambience;
mod holocall;
mod v;

static TRACKS: OnceLock<Tracks> = OnceLock::new();

pub struct Tracks {
    pub reverb: TrackHandle,
    pub v: V,
    pub holocall: Holocall,
    pub ambience: Ambience,
}

impl Tracks {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        // TODO: AmbienceTrack::init(manager)?;

        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbMix::effect()?);
            builder
        })?;

        let v = V::setup(manager, &reverb)?;
        let holocall = Holocall::setup(manager)?;
        let ambience = Ambience::setup(manager)?;

        TRACKS
            .set(Tracks {
                reverb,
                v,
                holocall,
                ambience,
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
