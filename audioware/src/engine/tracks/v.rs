use audioware_manifest::Source;
use kira::{
    manager::{backend::Backend, AudioManager},
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
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        ambience: &Ambience,
        tweener_dialogue: &DialogueVolume,
        tweener_sfx: &SfxVolume,
    ) -> Result<Self, Error> {
        let vocal = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(tweener_dialogue.try_effect()?),
        )?;
        let mental = manager
            .add_sub_track(TrackBuilder::new().with_effect(tweener_dialogue.try_effect()?))?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new()
                .routes(
                    TrackRoutes::parent(ambience.environmental())
                        .with_route(ambience.reverb(), 0.25),
                )
                .with_effect(tweener_sfx.try_effect()?),
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
