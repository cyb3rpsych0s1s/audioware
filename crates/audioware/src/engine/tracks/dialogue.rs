use kira::{
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
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
                .routes(
                    // sum must be 1.0 otherwise sounds crackle
                    TrackRoutes::empty()
                        .with_route(ambience.environmental(), 0.75)
                        .with_route(ambience.reverb(), 0.25),
                )
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

impl<'a> From<&'a Dialogue> for OutputDestination {
    fn from(value: &'a Dialogue) -> Self {
        (&value.0).into()
    }
}
