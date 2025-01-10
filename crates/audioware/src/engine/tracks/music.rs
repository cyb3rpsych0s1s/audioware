use kira::{
    backend::Backend,
    track::{TrackBuilder, TrackHandle},
    AudioManager, Decibels,
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct Music(TrackHandle);

impl Music {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                // reverb used to require to be set otherwise sound switched to mono, what now?
                .with_send(ambience.reverb(), Decibels::SILENCE)
                .with_effect(modulators.music_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl AsRef<TrackHandle> for Music {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a Music> for &'a TrackHandle {
    fn from(value: &'a Music) -> Self {
        &value.0
    }
}
