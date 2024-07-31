use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};

use crate::{
    engine::modulators::{CarRadioVolume, Parameter},
    error::Error,
};

pub struct CarRadio(pub TrackHandle);

impl CarRadio {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
        let track =
            manager.add_sub_track(TrackBuilder::new().with_effect(CarRadioVolume::effect()?))?;
        Ok(Self(track))
    }
}

impl<'a> From<&'a CarRadio> for OutputDestination {
    fn from(value: &'a CarRadio) -> Self {
        (&value.0).into()
    }
}
