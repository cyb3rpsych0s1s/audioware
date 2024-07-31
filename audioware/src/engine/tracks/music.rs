use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
    OutputDestination,
};

use crate::{
    engine::modulators::{MusicVolume, Parameter},
    error::Error,
};

pub struct Music(pub TrackHandle);

impl Music {
    pub fn setup(manager: &mut AudioManager) -> Result<Self, Error> {
        let track =
            manager.add_sub_track(TrackBuilder::new().with_effect(MusicVolume::effect()?))?;
        Ok(Self(track))
    }
}

impl<'a> From<&'a Music> for OutputDestination {
    fn from(value: &'a Music) -> Self {
        (&value.0).into()
    }
}
