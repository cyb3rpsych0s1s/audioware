use std::sync::Mutex;

use kira::{
    effect::{
        filter::{FilterBuilder, FilterHandle, FilterMode},
        reverb::ReverbBuilder,
    },
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    tween::{ModulatorMapping, Value},
};
use once_cell::sync::OnceCell;
use snafu::OptionExt;

use audioware_core::UninitializedSnafu;

use super::{
    effect::{
        HighPass, LowPass, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_DEFAULT_FREQUENCES,
        EQ_LOW_PASS_PHONE_CUTOFF, EQ_LOW_PASS_UNDERWATER_CUTOFF, EQ_RESONANCE,
    },
    error::Error,
    manager::{audio_manager, audio_modulator},
};

pub static TRACKS: OnceCell<Tracks> = OnceCell::new();

#[inline(always)]
pub fn maybe_tracks<'cell>() -> Result<&'cell Tracks, Error> {
    Ok(TRACKS
        .get()
        .context(UninitializedSnafu { which: "tracks" })?)
}

pub struct Tracks {
    pub reverb: TrackHandle,
    pub v: V,
    pub holocall: Holocall,
}

pub struct V {
    pub main: TrackHandle,
    pub vocal: TrackHandle,
    pub mental: TrackHandle,
    pub environmental: TrackHandle,
}

pub struct Holocall {
    pub main: TrackHandle,
    pub eq: Mutex<EQ>,
}

impl Tracks {
    pub fn setup() -> Result<(), Error> {
        let mut manager = audio_manager().lock()?;
        let modulator = audio_modulator().lock()?;
        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().mix(1.0));
            builder
        })?;
        let holocall_lowpass: FilterHandle;
        let holocall_highpass: FilterHandle;
        let main = manager.add_sub_track(
            {
                let mut builder = TrackBuilder::new();
                builder.add_effect(FilterBuilder::new().cutoff(Value::from_modulator(
                    &*modulator,
                    ModulatorMapping {
                        input_range: (0.0, 100.0),
                        output_range: (
                            EQ_LOW_PASS_DEFAULT_FREQUENCES,
                            EQ_LOW_PASS_UNDERWATER_CUTOFF,
                        ),
                        ..Default::default()
                    },
                )));
                builder
            }
            .routes(TrackRoutes::new().with_route(&reverb, 0.)),
        )?;
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
            builder
        })?;
        let vocal =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let mental =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let environmental =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        TRACKS
            .set(Tracks {
                reverb,
                v: V {
                    main,
                    vocal,
                    mental,
                    environmental,
                },
                holocall: Holocall {
                    main: holocall,
                    eq: Mutex::new(EQ {
                        lowpass: LowPass(holocall_lowpass),
                        highpass: HighPass(holocall_highpass),
                    }),
                },
            })
            .map_err(|_| Error::Internal {
                source: audioware_core::Error::CannotSet { which: "tracks" },
            })?;
        Ok(())
    }
}
