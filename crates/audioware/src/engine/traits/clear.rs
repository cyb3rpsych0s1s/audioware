use kira::sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle};

use crate::engine::traits::{DualHandles, Handles};

pub trait Clear {
    fn clear(&mut self);
}

impl<K, O> Clear for Handles<K, StaticSoundHandle, O> {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}

impl<K, O, E> Clear for Handles<K, StreamingSoundHandle<E>, O> {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}

impl<K, O, E> Clear for DualHandles<K, O, E> {
    #[inline]
    fn clear(&mut self) {
        self.statics.clear();
        self.streams.clear();
    }
}
