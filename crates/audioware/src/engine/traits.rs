use audioware_core::Amplitude;
use kira::{
    Panning, PlaybackRate, Tween,
    sound::{PlaybackState, static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::{
    ControlId,
    engine::{traits::stop::Stop, tweens::IMMEDIATELY},
};

pub mod clear;
pub mod dilation;
pub mod panning;
pub mod pause;
pub mod playback;
pub mod position;
pub mod reclaim;
pub mod resume;
pub mod seek;
pub mod stop;
pub mod store;
pub mod volume;

#[derive(Default)]
pub struct Handles<K, V, O>(Vec<Handle<K, V, O>>);

pub struct DualHandles<K, O, E> {
    pub statics: Handles<K, StaticSoundHandle, O>,
    pub streams: Handles<K, StreamingSoundHandle<E>, O>,
}

impl<K, O, E> Default for DualHandles<K, O, E> {
    fn default() -> Self {
        Self {
            statics: Handles(Vec::new()),
            streams: Handles(Vec::new()),
        }
    }
}

pub struct RawHandle<K, V> {
    key: K,
    value: V,
}

impl<K, V> RawHandle<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct Handle<K, V, O> {
    handle: RawHandle<K, V>,
    options: O,
    control_id: Option<ControlId>,
}

impl<K, V, O> Handle<K, V, O> {
    pub fn new(key: K, value: V, options: O, control_id: Option<ControlId>) -> Self {
        Self {
            handle: RawHandle::new(key, value),
            options,
            control_id,
        }
    }
}

impl<K, O> Handle<K, StaticSoundHandle, O> {
    pub fn set_volume(&mut self, value: Amplitude, tween: Tween) {
        self.handle.value.set_volume(value.as_decibels(), tween);
    }
    pub fn set_playback_rate(&mut self, value: PlaybackRate, tween: Tween) {
        self.handle.value.set_playback_rate(value, tween);
    }
    pub fn set_panning(&mut self, value: Panning, tween: Tween) {
        self.handle.value.set_panning(value, tween);
    }
}

impl<K, O, E> Handle<K, StreamingSoundHandle<E>, O> {
    pub fn set_volume(&mut self, value: Amplitude, tween: Tween) {
        self.handle.value.set_volume(value.as_decibels(), tween);
    }
    pub fn set_playback_rate(&mut self, value: PlaybackRate, tween: Tween) {
        self.handle.value.set_playback_rate(value, tween);
    }
    pub fn set_panning(&mut self, value: Panning, tween: Tween) {
        self.handle.value.set_panning(value, tween);
    }
}

impl<K> RawHandle<K, StaticSoundHandle> {
    #[inline]
    pub fn any_playing_handle(&self) -> bool {
        self.value.state() == PlaybackState::Playing
    }
}

impl<K, E> RawHandle<K, StreamingSoundHandle<E>> {
    #[inline]
    pub fn any_playing_handle(&self) -> bool {
        self.value.state() == PlaybackState::Playing
    }
}

impl<K, O, E> DualHandles<K, O, E> {
    #[inline]
    pub fn any_playing_handle(&self) -> bool {
        self.statics.0.iter().any(|x| x.handle.any_playing_handle())
            || self.streams.0.iter().any(|x| x.handle.any_playing_handle())
    }
    #[inline]
    pub fn any_handle(&self) -> bool {
        !self.statics.0.is_empty() || !self.streams.0.is_empty()
    }
}

impl<K, O, E> Drop for DualHandles<K, O, E> {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.stop(IMMEDIATELY);
    }
}
