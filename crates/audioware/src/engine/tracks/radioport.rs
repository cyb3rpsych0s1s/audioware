use kira::{
    AudioManager, Decibels,
    backend::Backend,
    track::{TrackBuilder, TrackHandle},
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct Radioport(TrackHandle);

impl Radioport {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                // reverb used to require to be set otherwise sound switched to mono, what now?
                .with_send(ambience.reverb(), Decibels::SILENCE)
                .with_effect(modulators.radioport_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl std::ops::Deref for Radioport {
    type Target = TrackHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Radioport {
    fn deref_mut(&mut self) -> &mut TrackHandle {
        &mut self.0
    }
}

impl AsRef<TrackHandle> for Radioport {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a Radioport> for &'a TrackHandle {
    fn from(value: &'a Radioport) -> Self {
        &value.0
    }
}
