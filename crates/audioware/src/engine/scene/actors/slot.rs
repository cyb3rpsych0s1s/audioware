use either::Either;
use kira::{
    PlaySoundError,
    sound::{FromFileError, static_sound::StaticSoundData, streaming::StreamingSoundData},
};
use red4ext_rs::types::Cruid;

use crate::{
    Vector4,
    engine::{
        tracks::Spatial,
        traits::{DualHandles, Handle, store::Store},
    },
};

/// Underlying handle to the actor.
pub struct ActorSlot {
    pub handle: Spatial,
    pub handles: DualHandles<Cruid, (), FromFileError>,
    pub last_known_position: Vector4,
}

type PlayResult = Result<(), Either<PlaySoundError<()>, PlaySoundError<FromFileError>>>;

impl ActorSlot {
    pub fn any_playing_handle(&self) -> bool {
        self.handles.any_playing_handle()
    }
    pub fn new(handle: Spatial, last_known_position: Vector4) -> Self {
        Self {
            handle,
            handles: DualHandles::default(),
            last_known_position,
        }
    }
    pub fn play_and_store(
        &mut self,
        event_name: Cruid,
        data: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> PlayResult {
        match data {
            Either::Left(data) => {
                let handle = self.handle.play(data).map_err(Either::Left)?;
                self.handles.store(Handle::new(event_name, handle, ()));
                Ok(())
            }
            Either::Right(data) => {
                let handle = self.handle.play(data).map_err(Either::Right)?;
                self.handles.store(Handle::new(event_name, handle, ()));
                Ok(())
            }
        }
    }
}
