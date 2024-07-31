use kira::{
    effect::filter::{FilterBuilder, FilterMode},
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};

use crate::{
    engine::{
        eq::{EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF, EQ_RESONANCE},
        modulators::{DialogueVolume, Parameter},
    },
    error::Error,
};

pub struct Holocall(pub TrackHandle);

impl Holocall {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
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
            builder.with_effect(DialogueVolume::effect()?)
        })?;
        Ok(Self(track))
    }
}

impl<'a> From<&'a Holocall> for OutputDestination {
    fn from(value: &'a Holocall) -> Self {
        (&value.0).into()
    }
}
