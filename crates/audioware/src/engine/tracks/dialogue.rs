use audioware_core::{amplitude, Amplitude};
use kira::{
    track::{TrackBuilder, TrackHandle},
    {backend::Backend, AudioManager},
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct Dialogue(TrackHandle);

impl Dialogue {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.75).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.25).as_decibels())
                .with_effect(modulators.dialogue_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl AsRef<TrackHandle> for Dialogue {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a Dialogue> for &'a TrackHandle {
    fn from(value: &'a Dialogue) -> Self {
        &value.0
    }
}
