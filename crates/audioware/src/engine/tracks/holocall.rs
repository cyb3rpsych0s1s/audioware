use kira::{
    backend::Backend,
    effect::filter::{FilterBuilder, FilterMode},
    track::{TrackBuilder, TrackHandle},
    AudioManager, Decibels,
};

use crate::{
    engine::{
        eq::{EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF, EQ_RESONANCE},
        modulators::{Modulators, Parameter},
    },
    error::Error,
};

use super::ambience::Ambience;

pub struct Holocall(TrackHandle);

impl Holocall {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let track = manager.add_sub_track({
            let mut builder = TrackBuilder::new()
                // reverb used to require to be set otherwise sound switched to mono, what now?
                .with_send(ambience.reverb(), Decibels::SILENCE);
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

impl<'a> From<&'a Holocall> for &'a TrackHandle {
    fn from(value: &'a Holocall) -> Self {
        &value.0
    }
}
