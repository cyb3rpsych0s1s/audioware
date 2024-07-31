use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};

use crate::{
    engine::modulators::{DialogueVolume, Parameter},
    error::Error,
};

pub struct Dialogue(TrackHandle);

impl Dialogue {
    pub fn setup(manager: &mut AudioManager, reverb: &TrackHandle) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                .routes(TrackRoutes::new().with_route(reverb, 0.25))
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
