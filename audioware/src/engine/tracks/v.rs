use kira::{
    manager::{backend::Backend, AudioManager},
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct V {
    pub vocal: TrackHandle,
    #[allow(dead_code, reason = "todo")]
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
}

impl V {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let vocal = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    // sum must be 1.0 otherwise sounds crackle
                    TrackRoutes::empty()
                        .with_route(ambience.environmental(), 0.75)
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(modulators.dialogue_volume.try_effect()?),
        )?;
        let mental = manager.add_sub_track(
            TrackBuilder::new().with_effect(modulators.dialogue_volume.try_effect()?),
        )?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    // sum must be 1.0 otherwise sounds crackle
                    TrackRoutes::empty()
                        .with_route(ambience.environmental(), 0.75)
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(modulators.sfx_volume.try_effect()?),
        )?;

        Ok(V {
            vocal,
            mental,
            emissive,
        })
    }
}
