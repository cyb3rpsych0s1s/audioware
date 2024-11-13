use kira::{
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};

use crate::{
    engine::modulators::{Parameter, RadioportVolume},
    error::Error,
};

pub struct Radioport(TrackHandle);

impl Radioport {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        tweener: &RadioportVolume,
    ) -> Result<Self, Error> {
        let track =
            manager.add_sub_track(TrackBuilder::new().with_effect(tweener.try_effect()?))?;
        Ok(Self(track))
    }
}

impl AsRef<TrackHandle> for Radioport {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a Radioport> for OutputDestination {
    fn from(value: &'a Radioport) -> Self {
        (&value.0).into()
    }
}
