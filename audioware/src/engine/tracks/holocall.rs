use std::sync::Mutex;

use kira::{
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
};

use crate::{
    engine::{
        eq::{
            HighPass, LowPass, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF,
            EQ_RESONANCE,
        },
        modulators::{DialogueVolume, Parameter},
    },
    error::Error,
};

pub struct Holocall {
    pub main: TrackHandle,
    pub eq: Mutex<EQ>,
}

impl Holocall {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
        let holocall_lowpass: FilterHandle;
        let holocall_highpass: FilterHandle;
        let holocall = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            holocall_lowpass = builder.add_effect(
                FilterBuilder::default()
                    .cutoff(EQ_LOW_PASS_PHONE_CUTOFF)
                    .resonance(EQ_RESONANCE),
            );
            holocall_highpass = builder.add_effect(
                FilterBuilder::default()
                    .mode(FilterMode::HighPass)
                    .cutoff(EQ_HIGH_PASS_PHONE_CUTOFF)
                    .resonance(EQ_RESONANCE),
            );
            builder.with_effect(DialogueVolume::effect()?)
        })?;
        Ok(Holocall {
            main: holocall,
            eq: Mutex::new(EQ {
                lowpass: LowPass(holocall_lowpass),
                highpass: HighPass(holocall_highpass),
            }),
        })
    }
}
