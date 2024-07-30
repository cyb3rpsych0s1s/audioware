use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
};

use crate::{
    engine::modulators::{MusicVolume, Parameter},
    error::Error,
};

pub struct Ambience(TrackHandle);

impl Ambience {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
        let ambience = manager.add_sub_track({
            let builder = TrackBuilder::new();
            builder.with_effect(MusicVolume::effect()?)
        })?;
        Ok(Self(ambience))
    }
}
