use std::sync::Mutex;

use ambience::AmbienceTrack;
use kira::{manager::AudioManager, track::TrackHandle};
use once_cell::sync::OnceCell;
use snafu::OptionExt;

use audioware_core::UninitializedSnafu;

use crate::modulator::{Parameter, VolumeModulator};

mod ambience;

use super::{effect::EQ, error::Error};

static TRACKS: OnceCell<Tracks> = OnceCell::new();

#[inline(always)]
pub fn maybe_tracks<'cell>() -> Result<&'cell Tracks, Error> {
    Ok(TRACKS
        .get()
        .context(UninitializedSnafu { which: "tracks" })?)
}

pub struct Tracks {
    pub reverb: TrackHandle,
    pub v: V,
    pub holocall: Holocall,
}

pub struct V {
    pub main: TrackHandle,
    pub vocal: TrackHandle,
    pub mental: TrackHandle,
    pub environmental: TrackHandle,
}

pub struct Holocall {
    pub main: TrackHandle,
    pub eq: Mutex<EQ>,
}

impl Tracks {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        VolumeModulator::init(manager)?;
        AmbienceTrack::init(manager)?;
        Ok(())
    }
}
