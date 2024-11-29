use kira::{
    effect::filter::{FilterBuilder, FilterMode},
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
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
        let main = manager.main_track().id();
        let track = manager.add_sub_track({
            let mut builder = TrackBuilder::new().routes(
                // sum must be 1.0 otherwise sounds crackle
                TrackRoutes::empty()
                    .with_route(main, 1.)
                    .with_route(ambience.reverb(), 0.),
            );
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
