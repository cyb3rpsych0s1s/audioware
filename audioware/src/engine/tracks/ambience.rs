use kira::{
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle},
};

use crate::{
    engine::{
        eq::{HighPass, LowPass, EQ},
        modulators::{Modulators, Parameter},
    },
    error::Error,
};

/// Sub-track to provide reverb and environmental effects.
pub struct Ambience {
    eq: EQ,
    reverb: TrackHandle,
    environmental: TrackHandle,
}

impl Ambience {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let low: FilterHandle;
        let high: FilterHandle;
        let reverb = manager
            .add_sub_track(TrackBuilder::new().with_effect(modulators.reverb_mix.try_effect()?))?;
        let environmental = manager.add_sub_track({
            let mut builder = TrackBuilder::new().with_effect(modulators.reverb_mix.try_effect()?);
            low = builder.add_effect(FilterBuilder::default().mix(0.));
            high = builder.add_effect(FilterBuilder::default().mode(FilterMode::HighPass).mix(0.));
            builder
        })?;
        Ok(Self {
            reverb,
            environmental,
            eq: EQ {
                lowpass: LowPass(low),
                highpass: HighPass(high),
            },
        })
    }
    pub fn reverb(&self) -> &TrackHandle {
        &self.reverb
    }
    pub fn environmental(&self) -> &TrackHandle {
        &self.environmental
    }
    pub fn equalizer(&mut self) -> &mut EQ {
        &mut self.eq
    }
}
