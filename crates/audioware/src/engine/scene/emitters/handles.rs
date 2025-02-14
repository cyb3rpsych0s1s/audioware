use kira::{
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    Tween,
};
use red4ext_rs::types::CName;

use crate::engine::tweens::IMMEDIATELY;

#[derive(Debug, Default)]
pub struct Handles {
    pub statics: Vec<Handle<StaticSoundHandle>>,
    pub streams: Vec<Handle<StreamingSoundHandle<FromFileError>>>,
}

impl Handles {
    pub fn stop_by_event_name(&mut self, event_name: CName, tween: Tween) {
        self.statics
            .iter_mut()
            .filter(|x| x.event_name == event_name)
            .for_each(|x| x.handle.stop(tween));
        self.streams
            .iter_mut()
            .filter(|x| x.event_name == event_name)
            .for_each(|x| x.handle.stop(tween));
    }
    pub fn stop(&mut self, tween: Tween) {
        self.statics.iter_mut().for_each(|x| x.handle.stop(tween));
        self.streams.iter_mut().for_each(|x| x.handle.stop(tween));
    }
    pub fn pause(&mut self, tween: Tween) {
        self.statics.iter_mut().for_each(|x| x.handle.pause(tween));
        self.streams.iter_mut().for_each(|x| x.handle.pause(tween));
    }
    pub fn resume(&mut self, tween: Tween) {
        self.statics.iter_mut().for_each(|x| x.handle.resume(tween));
        self.streams.iter_mut().for_each(|x| x.handle.resume(tween));
    }
    pub fn reclaim(&mut self) {
        self.statics
            .retain(|x| x.handle.state() != PlaybackState::Stopped);
        self.streams
            .retain(|x| x.handle.state() != PlaybackState::Stopped);
    }
    pub fn sync_dilation(&mut self, dilation: f64, tween: Tween) {
        self.statics
            .iter_mut()
            .filter(|x| x.affected_by_time_dilation)
            .for_each(|x| x.handle.set_playback_rate(dilation, tween));
        self.streams
            .iter_mut()
            .filter(|x| x.affected_by_time_dilation)
            .for_each(|x| x.handle.set_playback_rate(dilation, tween));
    }
}

impl Drop for Handles {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.iter_mut().for_each(|x| {
            x.handle.stop(IMMEDIATELY);
        });
    }
}

pub struct Handle<T> {
    pub event_name: CName,
    pub handle: T,
    pub affected_by_time_dilation: bool,
}

impl<T> std::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("event_name", &self.event_name)
            .field("affected_by_time_dilation", &self.affected_by_time_dilation)
            .finish_non_exhaustive()
    }
}

impl Handles {
    pub fn store_static(
        &mut self,
        event_name: CName,
        handle: StaticSoundHandle,
        affected_by_time_dilation: bool,
    ) {
        self.statics.push(Handle {
            event_name,
            handle,
            affected_by_time_dilation,
        });
    }
    pub fn store_stream(
        &mut self,
        event_name: CName,
        handle: StreamingSoundHandle<FromFileError>,
        affected_by_time_dilation: bool,
    ) {
        self.streams.push(Handle {
            event_name,
            handle,
            affected_by_time_dilation,
        });
    }
    pub fn any_playing_handle(&self) -> bool {
        self.statics
            .iter()
            .any(|x| x.handle.state() == PlaybackState::Playing)
            || self
                .streams
                .iter()
                .any(|x| x.handle.state() == PlaybackState::Playing)
    }
}
