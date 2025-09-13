use std::hash::Hash;

use dashmap::{DashMap, mapref::multiple::RefMutMulti};
use kira::{
    Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::engine::traits::{DualHandles, Handle, Handles, RawHandle};

pub trait Resume {
    fn resume(&mut self, tween: Tween);
}

impl Resume for StaticSoundHandle {
    fn resume(&mut self, tween: Tween) {
        self.resume(tween);
    }
}

impl<E> Resume for StreamingSoundHandle<E> {
    fn resume(&mut self, tween: Tween) {
        self.resume(tween);
    }
}

impl<K, V> Resume for RawHandle<K, V>
where
    V: Resume,
{
    fn resume(&mut self, tween: Tween) {
        self.value.resume(tween);
    }
}

impl<K, V, O> Resume for Handle<K, V, O>
where
    V: Resume,
{
    fn resume(&mut self, tween: Tween) {
        self.handle.resume(tween);
    }
}

impl<K, V, O> Resume for Handles<K, V, O>
where
    V: Resume,
{
    fn resume(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.handle.resume(tween));
    }
}

impl<K, O, E> Resume for DualHandles<K, O, E> {
    fn resume(&mut self, tween: Tween) {
        self.statics.resume(tween);
        self.streams.resume(tween);
    }
}

impl<K, V> Resume for RefMutMulti<'_, K, V>
where
    V: Resume,
    K: Eq + Hash,
{
    fn resume(&mut self, tween: Tween) {
        self.value_mut().resume(tween);
    }
}

impl<K, V> Resume for DashMap<K, V>
where
    V: Resume,
    K: Eq + Hash,
{
    fn resume(&mut self, tween: Tween) {
        self.iter_mut().for_each(|mut x| x.resume(tween));
    }
}
