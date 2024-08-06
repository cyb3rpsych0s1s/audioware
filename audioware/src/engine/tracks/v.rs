use audioware_manifest::Source;
use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};

use crate::{
    engine::modulators::{DialogueVolume, Parameter, SfxVolume},
    error::Error,
};

use super::ambience::Ambience;

pub struct V {
    pub vocal: TrackHandle,
    #[allow(dead_code)]
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
}

impl V {
    pub fn setup(manager: &mut AudioManager, ambience: &Ambience) -> Result<Self, Error> {
        let vocal = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(DialogueVolume::effect()?),
        )?;
        let mental =
            manager.add_sub_track(TrackBuilder::new().with_effect(DialogueVolume::effect()?))?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(SfxVolume::effect()?),
        )?;

        Ok(V {
            vocal,
            mental,
            emissive,
        })
    }
}

impl V {
    pub fn output_destination(&self, source: &Source) -> Option<OutputDestination> {
        match source {
            Source::Sfx => return Some((&self.emissive).into()),
            Source::Ono | Source::Voices => return Some((&self.vocal).into()),
            _ => {}
        };
        None
    }
}
