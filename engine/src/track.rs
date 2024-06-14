use std::sync::Mutex;

use kira::{
    effect::{
        filter::{FilterBuilder, FilterHandle, FilterMode},
        reverb::ReverbBuilder,
    },
    manager::AudioManager,
    modulator::tweener::TweenerBuilder,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};
use once_cell::sync::OnceCell;
use snafu::OptionExt;

use audioware_core::UninitializedSnafu;

use crate::modulator::{Parameter, VolumeModulator};

use super::{
    effect::{
        HighPass, LowPass, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF, EQ_RESONANCE,
    },
    error::Error,
    manager::audio_manager,
};

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
    pub fn setup() -> Result<(), Error> {
        Ok(())
    }
}
