use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};

use crate::{
    engine::modulators::{Parameter, SfxVolume},
    error::Error,
};

use super::ambience::Ambience;

pub struct Sfx(TrackHandle);

impl Sfx {
    pub(super) fn setup(manager: &mut AudioManager, ambience: &Ambience) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.5),
                )
                .with_effect(SfxVolume::effect()?),
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
