use kira::{
    AudioManager, Decibels,
    backend::Backend,
    track::{TrackBuilder, TrackHandle},
};

use crate::error::Error;

use super::ambience::Ambience;

pub struct Music(TrackHandle);

impl Music {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                // reverb used to require to be set otherwise sound switched to mono, what now?
                .with_send(ambience.reverb(), Decibels::SILENCE),
        )?;
        Ok(Self(track))
    }
}

impl std::ops::Deref for Music {
    type Target = TrackHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Music {
    fn deref_mut(&mut self) -> &mut TrackHandle {
        &mut self.0
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
