use kira::{
    Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

pub trait SetPlaybackRate {
    fn set_playback_rate(&mut self, rate: f64, tween: Tween);
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
