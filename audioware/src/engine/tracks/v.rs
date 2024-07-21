use std::sync::Mutex;

use kira::{
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};

use crate::{
    engine::eq::{HighPass, LowPass, EQ},
    error::Error,
};

pub struct V {
    pub main: TrackHandle,
    pub vocal: TrackHandle,
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
    pub eq: Mutex<EQ>,
}

impl V {
    pub fn setup(manager: &mut AudioManager, reverb: &TrackHandle) -> Result<Self, Error> {
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
            .routes(TrackRoutes::new().with_route(reverb, 0.)),
        )?;

        let vocal =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let mental =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let emissive =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;

        Ok(V {
            main,
            vocal,
            mental,
            emissive,
            eq: Mutex::new(EQ {
                lowpass: LowPass(player_lowpass),
                highpass: HighPass(player_highpass),
            }),
        })
    }
}
