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

pub struct Sfx(TrackHandle);

impl Sfx {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.5),
                )
                .with_effect(modulators.sfx_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl AsRef<TrackHandle> for Sfx {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a Sfx> for OutputDestination {
    fn from(value: &'a Sfx) -> Self {
        (&value.0).into()
    }
}
