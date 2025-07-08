use audioware_core::{Amplitude, amplitude};
use kira::{
    track::{TrackBuilder, TrackHandle},
    {AudioManager, backend::Backend},
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

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
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let vocal = manager.add_sub_track(
            TrackBuilder::new()
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.75).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.25).as_decibels())
                .with_effect(modulators.dialogue_volume.try_effect()?),
        )?;
        let mental = manager.add_sub_track(
            TrackBuilder::new().with_effect(modulators.dialogue_volume.try_effect()?),
        )?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new()
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.75).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.25).as_decibels())
                .with_effect(modulators.sfx_volume.try_effect()?),
        )?;

        Ok(V {
            vocal,
            mental,
            emissive,
        })
    }
}
