use std::sync::{Mutex, OnceLock};

use kira::{
    effect::{
        filter::{FilterBuilder, FilterHandle, FilterMode},
        reverb::ReverbBuilder,
    },
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};

use crate::error::{Error, InternalError};

use super::{
    eq::{
        HighPass, LowPass, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF, EQ_RESONANCE,
    },
    modulators::{Parameter, VolumeModulator},
};

static TRACKS: OnceLock<Tracks> = OnceLock::new();

pub struct Tracks {
    pub reverb: TrackHandle,
    pub v: V,
    pub holocall: Holocall,
}

pub struct V {
    pub main: TrackHandle,
    pub vocal: TrackHandle,
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
    pub eq: Mutex<EQ>,
}

pub struct Holocall {
    pub main: TrackHandle,
    pub eq: Mutex<EQ>,
}

impl Tracks {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        VolumeModulator::setup(manager)?;
        // TODO: AmbienceTrack::init(manager)?;

        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().mix(1.0));
            builder
        })?;

        let player_lowpass: FilterHandle;
        let player_highpass: FilterHandle;
        let main = manager.add_sub_track(
            {
                let mut builder = TrackBuilder::new();
                player_lowpass = builder.add_effect(FilterBuilder::default().mix(0.));
                player_highpass =
                    builder.add_effect(FilterBuilder::default().mode(FilterMode::HighPass).mix(0.));
                builder
            }
            .routes(TrackRoutes::new().with_route(&reverb, 0.)),
        )?;

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
            builder
        })?;

        let vocal =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let mental =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let emissive =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;

        TRACKS
            .set(Tracks {
                reverb,
                v: V {
                    main,
                    vocal,
                    mental,
                    emissive,
                    eq: Mutex::new(EQ {
                        lowpass: LowPass(player_lowpass),
                        highpass: HighPass(player_highpass),
                    }),
                },
                holocall: Holocall {
                    main: holocall,
                    eq: Mutex::new(EQ {
                        lowpass: LowPass(holocall_lowpass),
                        highpass: HighPass(holocall_highpass),
                    }),
                },
            })
            .map_err(|_| {
                Error::from(InternalError::Init {
                    origin: "main tracks",
                })
            })?;

        Ok(())
    }
}
