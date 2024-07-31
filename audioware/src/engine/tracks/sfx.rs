use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};

use crate::{
    engine::modulators::{Parameter, SfxVolume},
    error::Error,
};

pub struct Sfx(pub TrackHandle);

impl Sfx {
    pub fn setup(manager: &mut AudioManager, reverb: &TrackHandle) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                .routes(TrackRoutes::new().with_route(reverb, 0.5))
                .with_effect(SfxVolume::effect()?),
        )?;
        Ok(Self(track))
    }
}

impl<'a> From<&'a Sfx> for OutputDestination {
    fn from(value: &'a Sfx) -> Self {
        (&value.0).into()
    }
}
