use std::{collections::HashMap, hash::Hash, sync::Mutex};

use ambience::AmbienceTrack;
use audioware_macros::Repr;
use kira::{manager::AudioManager, track::TrackHandle};
use once_cell::sync::OnceCell;
use red4ext_rs::{conv::FromRepr, types::CName};
use snafu::OptionExt;

use audioware_core::UninitializedSnafu;

use crate::modulator::{Parameter, VolumeModulator};

mod ambience;

use super::{effect::EQ, error::Error};

static TRACKS: OnceCell<Tracks> = OnceCell::new();

#[derive(Debug, Clone, PartialEq, Eq, Repr)]
#[repr(transparent)]
pub struct TrackName(CName);

pub fn maybe_custom_tracks() -> &'static Mutex<HashMap<TrackName, TrackHandle>> {
    static INSTANCE: OnceCell<Mutex<HashMap<TrackName, TrackHandle>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

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
