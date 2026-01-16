use either::Either;
use kira::{
    PlaySoundError, Tween,
    sound::{FromFileError, static_sound::StaticSoundData, streaming::StreamingSoundData},
};
use red4ext_rs::types::CName;

use crate::engine::traits::{dilation::SyncDilation, pause::Pause};
use crate::engine::{AffectedByTimeDilation, traits::resume::Resume};
use crate::engine::{
    tracks::Spatial,
    traits::{DualHandles, Handle, store::Store},
};
use crate::{ControlId, engine::traits::stop::Stop};

pub struct EmitterEntryOptions {
    pub affected_by_time_dilation: bool,
}

impl AffectedByTimeDilation for EmitterEntryOptions {
    fn affected_by_time_dilation(&self) -> bool {
        self.affected_by_time_dilation
    }
}

/// Underlying handle to the emitter,
/// and whether sound(s) should persist until they finish playing.
pub struct EmitterSlot {
    pub handle: Spatial,
    pub tag_name: Option<CName>,
    pub emitter_name: Option<CName>,
    pub persist_until_sounds_finish: bool,
    pub handles: DualHandles<CName, EmitterEntryOptions, FromFileError>,
}

type PlayResult =
    Result<(f32, Option<CName>), Either<PlaySoundError<()>, PlaySoundError<FromFileError>>>;

impl EmitterSlot {
    pub fn any_playing_handle(&self) -> bool {
        self.handles.any_playing_handle()
    }
    pub fn new(
        handle: Spatial,
        tag_name: CName,
        emitter_name: Option<CName>,
        persist_until_sounds_finish: bool,
    ) -> Self {
        Self {
            handle,
            tag_name: Some(tag_name),
            emitter_name,
            persist_until_sounds_finish,
            handles: DualHandles::default(),
        }
    }
    pub fn play_and_store(
        &mut self,
        event_name: CName,
        affected_by_time_dilation: bool,
        data: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
        control_id: Option<ControlId>,
    ) -> PlayResult {
        match data {
            Either::Left(data) => {
                let duration = data.duration().as_secs_f32();
                let handle = self.handle.play(data).map_err(Either::Left)?;
                self.handles.store(Handle::new(
                    event_name,
                    handle,
                    EmitterEntryOptions {
                        affected_by_time_dilation,
                    },
                    control_id,
                ));
                Ok((duration, self.emitter_name))
            }
            Either::Right(data) => {
                let duration = data.duration().as_secs_f32();
                let handle = self.handle.play(data).map_err(Either::Right)?;
                self.handles.store(Handle::new(
                    event_name,
                    handle,
                    EmitterEntryOptions {
                        affected_by_time_dilation,
                    },
                    control_id,
                ));
                Ok((duration, self.emitter_name))
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
    pub fn occluded(&self) -> bool {
        self.handle.occluded()
    }
}
