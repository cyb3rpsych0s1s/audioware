use kira::sound::{
    PlaybackState, static_sound::StaticSoundHandle, streaming::StreamingSoundHandle,
};

use crate::engine::traits::{DualHandles, Handles};

pub trait Reclaim {
    fn reclaim(&mut self);
}

impl<K, O> Reclaim for Handles<K, StaticSoundHandle, O> {
    #[inline]
    fn reclaim(&mut self) {
        self.0
            .retain(|x| x.handle.value.state() != PlaybackState::Stopped);
    }
}

impl<K, O, E> Reclaim for Handles<K, StreamingSoundHandle<E>, O> {
    #[inline]
    fn reclaim(&mut self) {
        self.0
            .retain(|x| x.handle.value.state() != PlaybackState::Stopped);
    }
}

impl<K, O, E> Reclaim for DualHandles<K, O, E> {
    #[inline]
    fn reclaim(&mut self) {
        self.statics.reclaim();
        self.streams.reclaim();
    }
}
