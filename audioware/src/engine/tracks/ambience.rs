use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
};

use crate::{
    engine::modulators::{Parameter, SfxVolume},
    error::Error,
};

pub struct Ambience(pub TrackHandle);

impl Ambience {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
        let ambience = manager.add_sub_track({
            let builder = TrackBuilder::new();
            builder.with_effect(SfxVolume::effect()?)
        })?;
        Ok(Self(ambience))
    }
}
