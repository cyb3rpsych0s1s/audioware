use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle},
};
use once_cell::sync::OnceCell;

use crate::Error;

static INSTANCE: OnceCell<TrackHandle> = OnceCell::new();

pub struct AmbienceTrack;
impl AmbienceTrack {
    pub(super) fn init(manager: &mut AudioManager) -> Result<(), Error> {
        let handle = manager.add_sub_track(TrackBuilder::new())?;
        INSTANCE
            .set(handle)
            .expect("store ambience track handle once");
        Ok(())
    }
}
