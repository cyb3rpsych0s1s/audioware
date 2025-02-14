use kira::{
    backend::Backend,
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    track::{SendTrackBuilder, SendTrackHandle},
    AudioManager, Mix,
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
    reverb: SendTrackHandle,
    environmental: SendTrackHandle,
}

impl Ambience {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let low: FilterHandle;
        let high: FilterHandle;
        let reverb = manager.add_send_track(
            SendTrackBuilder::new().with_effect(modulators.reverb_mix.try_effect()?),
        )?;
        let environmental = manager.add_send_track({
            let mut builder = SendTrackBuilder::new();
            low = builder.add_effect(FilterBuilder::default().mix(Mix::DRY));
            high = builder.add_effect(
                FilterBuilder::default()
                    .mode(FilterMode::HighPass)
                    .mix(Mix::WET),
            );
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
    pub fn reverb(&self) -> &SendTrackHandle {
        &self.reverb
    }
    pub fn environmental(&self) -> &SendTrackHandle {
        &self.environmental
    }
    pub fn equalizer(&mut self) -> &mut EQ {
        &mut self.eq
    }
}
