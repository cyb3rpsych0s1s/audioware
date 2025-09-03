use std::fmt::Debug;

use kira::{
    Tween,
    sound::{
        FromFileError, PlaybackState, static_sound::StaticSoundHandle,
        streaming::StreamingSoundHandle,
    },
};

use crate::engine::tweens::IMMEDIATELY;

#[derive(Debug, Default)]
pub struct Handles<K> {
    pub statics: Vec<Handle<K, StaticSoundHandle>>,
    pub streams: Vec<Handle<K, StreamingSoundHandle<FromFileError>>>,
}

impl<K: PartialEq> Handles<K> {
    pub fn stop_by_event_name(&mut self, event_name: K, tween: Tween) {
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

impl<K> Drop for Handles<K> {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.iter_mut().for_each(|x| {
            x.handle.stop(IMMEDIATELY);
        });
    }
}

pub struct Handle<K, V> {
    pub event_name: K,
    pub handle: V,
    pub affected_by_time_dilation: bool,
}

impl<K, V> std::fmt::Debug for Handle<K, V>
where
    K: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("event_name", &self.event_name)
            .field("affected_by_time_dilation", &self.affected_by_time_dilation)
            .finish_non_exhaustive()
    }
}

impl<K> Handles<K> {
    pub fn store_static(
        &mut self,
        event_name: K,
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
        event_name: K,
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
