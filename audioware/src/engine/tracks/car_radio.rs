use kira::{
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

pub struct CarRadio(TrackHandle);

impl CarRadio {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new().with_effect(modulators.car_radio_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl AsRef<TrackHandle> for CarRadio {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a CarRadio> for OutputDestination {
    fn from(value: &'a CarRadio) -> Self {
        (&value.0).into()
    }
}
