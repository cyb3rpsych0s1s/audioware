use kira::{sound::FromFileError, track::SpatialTrackHandle};
use red4ext_rs::types::Cruid;

use crate::{
    Vector4,
    engine::{
        tracks::Spatial,
        traits::{
            DualHandles,
            stop::{Stop, StopBy},
        },
        tweens::IMMEDIATELY,
    },
};

/// Underlying handle to the actor.
pub struct ActorSlot {
    pub handle: Spatial,
    pub last_known_position: Vector4,
    pub handles: DualHandles<Cruid, (), FromFileError>,
}

impl std::fmt::Display for ActorSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "actor slot last known position: {}",
            self.last_known_position
        )
    }
}

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
    pub fn set_emitter_position(&mut self, position: Vector4) {
        if self.last_known_position != position {
            self.last_known_position = position;
            self.handle
                .set_position(self.last_known_position, IMMEDIATELY);
        }
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
