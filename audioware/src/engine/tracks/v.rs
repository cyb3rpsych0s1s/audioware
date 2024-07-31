use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};

use crate::{
    engine::modulators::{DialogueVolume, Parameter, SfxVolume},
    error::Error,
};

pub struct V {
    pub main: TrackHandle,
    pub vocal: TrackHandle,
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
}

impl V {
    pub fn setup(manager: &mut AudioManager, reverb: &TrackHandle) -> Result<Self, Error> {
        let main = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(reverb, 0.25)),
        )?;

        let vocal = manager.add_sub_track(
            TrackBuilder::new()
                .routes(TrackRoutes::parent(&main))
                .with_effect(DialogueVolume::effect()?),
        )?;
        let mental = manager.add_sub_track(
            TrackBuilder::new()
                .routes(TrackRoutes::parent(&main))
                .with_effect(SfxVolume::effect()?),
        )?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new()
                .routes(TrackRoutes::parent(&main))
                .with_effect(SfxVolume::effect()?),
        )?;

        Ok(V {
            main,
            vocal,
            mental,
            emissive,
        })
    }
}
