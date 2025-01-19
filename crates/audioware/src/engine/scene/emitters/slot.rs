use either::Either;
use kira::{
    sound::{
        static_sound::StaticSoundData, streaming::StreamingSoundData, FromFileError, SoundData,
    },
    track::SpatialTrackHandle,
    PlaySoundError, Tween,
};
use red4ext_rs::types::CName;

use super::handles::Handles;

/// Underlying handle to the emitter,
/// and whether sound(s) should persist until they finish playing.
pub struct EmitterSlot {
    pub handle: SpatialTrackHandle,
    pub tag_name: Option<CName>,
    pub emitter_name: Option<CName>,
    pub persist_until_sounds_finish: bool,
    pub handles: Handles,
}

impl EmitterSlot {
    pub fn any_playing_handle(&self) -> bool {
        self.handles.any_playing_handle()
    }
    pub fn new(
        handle: SpatialTrackHandle,
        tag_name: CName,
        emitter_name: Option<CName>,
        persist_until_sounds_finish: bool,
    ) -> Self {
        Self {
            handle,
            tag_name: Some(tag_name),
            emitter_name,
            persist_until_sounds_finish,
            handles: Handles::default(),
        }
    }
    pub fn play_and_store(
        &mut self,
        event_name: CName,
        affected_by_time_dilation: bool,
        data: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> Result<(f32, Option<CName>), Either<PlaySoundError<()>, PlaySoundError<FromFileError>>>
    {
        match data {
            Either::Left(data) => {
                let duration = data.duration().as_secs_f32();
                let handle = self.handle.play(data).map_err(Either::Left)?;
                self.handles
                    .store_static(event_name, handle, affected_by_time_dilation);
                Ok((duration, self.emitter_name.clone()))
            }
            Either::Right(data) => {
                let duration = data.duration().as_secs_f32();
                let handle = self.handle.play(data).map_err(Either::Right)?;
                self.handles
                    .store_stream(event_name, handle, affected_by_time_dilation);
                Ok((duration, self.emitter_name.clone()))
            }
        }
    }
    pub fn stop(&mut self, tween: Tween) {
        self.handles.stop(tween);
    }
    pub fn pause(&mut self, tween: Tween) {
        self.handles.pause(tween);
    }
    pub fn resume(&mut self, tween: Tween) {
        self.handles.resume(tween);
    }
    pub fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.handles.sync_dilation(rate, tween);
    }
}
