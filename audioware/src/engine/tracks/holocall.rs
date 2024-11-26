use kira::{
    effect::filter::{FilterBuilder, FilterMode},
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};

use crate::{
    engine::{
        eq::{EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF, EQ_RESONANCE},
        modulators::{Modulators, Parameter},
    },
    error::Error,
};

pub struct Holocall(TrackHandle);

impl Holocall {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(
                FilterBuilder::default()
                    .cutoff(EQ_LOW_PASS_PHONE_CUTOFF)
                    .resonance(EQ_RESONANCE),
            );
            builder.add_effect(
                FilterBuilder::default()
                    .mode(FilterMode::HighPass)
                    .cutoff(EQ_HIGH_PASS_PHONE_CUTOFF)
                    .resonance(EQ_RESONANCE),
            );
            builder.with_effect(modulators.dialogue_volume.try_effect()?)
        })?;
        Ok(Self(track))
    }
}

impl AsRef<TrackHandle> for Holocall {
    fn as_ref(&self) -> &TrackHandle {
        &self.0
    }
}

impl<'a> From<&'a Holocall> for OutputDestination {
    fn from(value: &'a Holocall) -> Self {
        (&value.0).into()
    }
}
