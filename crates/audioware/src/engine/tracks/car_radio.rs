use audioware_core::{Amplitude, amplitude};
use kira::{
    track::{TrackBuilder, TrackHandle},
    {AudioManager, backend::Backend},
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct CarRadio(TrackHandle);

impl CarRadio {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track(
            TrackBuilder::new()
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.75).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.25).as_decibels())
                .with_effect(modulators.car_radio_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl std::ops::Deref for CarRadio {
    type Target = TrackHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CarRadio {
    fn deref_mut(&mut self) -> &mut TrackHandle {
        &mut self.0
    }
}

impl AsRef<TrackHandle> for CarRadio {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a CarRadio> for &'a TrackHandle {
    fn from(value: &'a CarRadio) -> Self {
        &value.0
    }
}
