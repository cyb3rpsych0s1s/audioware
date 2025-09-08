use either::Either;
use kira::{PlaySoundError, sound::FromFileError, track::SpatialTrackHandle};
use red4ext_rs::types::Cruid;

use crate::{
    Vector4,
    engine::{
        tracks::Spatial,
        traits::{
            DualHandles,
            stop::{Stop, StopBy},
        },
    },
};

/// Underlying handle to the actor.
pub struct ActorSlot {
    handle: Spatial,
    last_known_position: Vector4,
    pub handles: DualHandles<Cruid, (), FromFileError>,
}

type PlayResult = Result<(), Either<PlaySoundError<()>, PlaySoundError<FromFileError>>>;

impl ActorSlot {
    pub fn any_playing_handle(&self) -> bool {
        self.handles.any_playing_handle()
    }
    pub fn new(handle: Spatial, last_known_position: Vector4) -> Self {
        Self {
            handle,
            handles: Default::default(),
            last_known_position,
        }
    }
    pub fn track_mut(&mut self) -> &mut SpatialTrackHandle {
        &mut self.handle
    }
}

impl Stop for ActorSlot {
    fn stop(&mut self, tween: kira::Tween) {
        self.handles.stop(tween);
    }
}

impl StopBy<Cruid> for ActorSlot {
    fn stop_by(&mut self, key: &Cruid, tween: kira::Tween) {
        self.handles.stop_by(key, tween);
    }
}
