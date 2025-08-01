use audioware_core::{Amplitude, amplitude};
use kira::{
    track::{TrackBuilder, TrackHandle},
    {AudioManager, backend::Backend},
};

use crate::error::Error;

use super::ambience::Ambience;

pub struct V {
    pub vocal: TrackHandle,
    #[allow(dead_code, reason = "todo")]
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
}

impl V {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
    ) -> Result<Self, Error> {
        let vocal = manager.add_sub_track(
            TrackBuilder::new()
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.75).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.25).as_decibels()),
        )?;
        let mental = manager.add_sub_track(TrackBuilder::new())?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new()
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.75).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.25).as_decibels()),
        )?;

        Ok(V {
            vocal,
            mental,
            emissive,
        })
    }
}
