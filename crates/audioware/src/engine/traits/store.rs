use kira::sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle};

use crate::engine::traits::{DualHandles, Handle, Handles};

pub trait Store<V> {
    fn store(&mut self, entry: V);
}

impl<K, V, O> Store<Handle<K, V, O>> for Handles<K, V, O> {
    #[inline]
    fn store(&mut self, entry: Handle<K, V, O>) {
        self.0.push(entry);
    }
}

impl<K, O, E> Store<Handle<K, StaticSoundHandle, O>> for DualHandles<K, O, E> {
    #[inline]
    fn store(&mut self, entry: Handle<K, StaticSoundHandle, O>) {
        self.statics.0.push(entry);
    }
}

impl<K, O, E> Store<Handle<K, StreamingSoundHandle<E>, O>> for DualHandles<K, O, E> {
    #[inline]
    fn store(&mut self, entry: Handle<K, StreamingSoundHandle<E>, O>) {
        self.streams.0.push(entry);
    }
}
