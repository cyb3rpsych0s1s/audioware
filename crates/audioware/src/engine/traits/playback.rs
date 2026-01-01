use kira::{
    PlaybackRate, Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handles},
};

pub trait SetPlaybackRate {
    fn set_playback_rate(&mut self, rate: f64, tween: Tween);
}

pub trait SetControlledPlaybackRate {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween);
}

impl<K, O, E> SetControlledPlaybackRate for DualHandles<K, O, E> {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.statics.set_controlled_playback_rate(id, rate, tween);
        self.streams.set_controlled_playback_rate(id, rate, tween);
    }
}

impl<K, O> SetControlledPlaybackRate for Handles<K, StaticSoundHandle, O> {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.set_playback_rate(PlaybackRate::from(rate), tween);
            });
    }
}

impl<K, O, E> SetControlledPlaybackRate for Handles<K, StreamingSoundHandle<E>, O> {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.set_playback_rate(PlaybackRate::from(rate), tween);
            });
    }
}

impl SetPlaybackRate for StaticSoundHandle {
    #[inline]
    fn set_playback_rate(&mut self, rate: f64, tween: Tween) {
        self.set_playback_rate(rate, tween);
    }
}

impl<E> SetPlaybackRate for StreamingSoundHandle<E> {
    #[inline]
    fn set_playback_rate(&mut self, rate: f64, tween: Tween) {
        self.set_playback_rate(rate, tween);
    }
}
