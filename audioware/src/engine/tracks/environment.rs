use std::sync::{Mutex, MutexGuard};

use kira::{
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
};

use crate::{
    engine::{
        eq::{HighPass, LowPass, EQ},
        modulators::{Parameter, ReverbMix},
    },
    error::{Error, InternalError},
};

pub struct Environment {
    reverb: TrackHandle,
    eq: Mutex<EQ>,
}

impl Environment {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
        let low: FilterHandle;
        let high: FilterHandle;
        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new().with_effect(ReverbMix::effect()?);
            low = builder.add_effect(FilterBuilder::default().mix(0.));
            high = builder.add_effect(FilterBuilder::default().mode(FilterMode::HighPass).mix(0.));
            builder
        })?;
        Ok(Self {
            reverb,
            eq: Mutex::new(EQ {
                lowpass: LowPass(low),
                highpass: HighPass(high),
            }),
        })
    }
    pub fn reverb(&self) -> &TrackHandle {
        &self.reverb
    }
    pub fn try_eq(&self) -> Result<MutexGuard<'_, EQ>, Error> {
        self.eq.try_lock().map_err(|_| {
            InternalError::Contention {
                origin: "preset equalizer",
            }
            .into()
        })
    }
}
