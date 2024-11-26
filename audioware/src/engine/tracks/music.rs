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

pub struct Music(TrackHandle);

impl Music {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        #[allow(unused_variables, reason = "check routing")] ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                .routes(TrackRoutes::new())
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

impl<'a> From<&'a Music> for OutputDestination {
    fn from(value: &'a Music) -> Self {
        (&value.0).into()
    }
}
