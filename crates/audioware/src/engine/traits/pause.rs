use std::hash::Hash;

use dashmap::mapref::multiple::RefMutMulti;
use kira::{
    Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::engine::traits::{DualHandles, Handle, Handles, RawHandle};

pub trait Pause {
    fn pause(&mut self, tween: Tween);
}

impl Pause for StaticSoundHandle {
    fn pause(&mut self, tween: Tween) {
        self.pause(tween);
    }
}

impl<E> Pause for StreamingSoundHandle<E> {
    fn pause(&mut self, tween: Tween) {
        self.pause(tween);
    }
}

impl<K, V> Pause for RawHandle<K, V>
where
    V: Pause,
{
    fn pause(&mut self, tween: Tween) {
        self.value.pause(tween);
    }
}

impl<K, V, O> Pause for Handle<K, V, O>
where
    V: Pause,
{
    fn pause(&mut self, tween: Tween) {
        self.handle.pause(tween);
    }
}

impl<K, V, O> Pause for Handles<K, V, O>
where
    V: Pause,
{
    fn pause(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.handle.pause(tween));
    }
}

impl<K, O, E> Pause for DualHandles<K, O, E> {
    fn pause(&mut self, tween: Tween) {
        self.statics.pause(tween);
        self.streams.pause(tween);
    }
}

impl<K, V> Pause for RefMutMulti<'_, K, V>
where
    V: Pause,
    K: Eq + Hash,
{
    fn pause(&mut self, tween: Tween) {
        self.value_mut().pause(tween);
    }
}
