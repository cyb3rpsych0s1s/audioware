use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};

use crate::{
    engine::modulators::{DialogueVolume, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct Dialogue(TrackHandle);

impl Dialogue {
    pub fn setup(manager: &mut AudioManager, ambience: &Ambience) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(DialogueVolume::effect()?),
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
