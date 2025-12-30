use std::hash::Hash;

use dashmap::{DashMap, mapref::multiple::RefMutMulti};
use kira::{
    Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handle, Handles, RawHandle},
};

pub trait Pause {
    fn pause(&mut self, tween: Tween);
}

pub trait PauseControlled {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween);
}

impl Pause for StaticSoundHandle {
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.pause(tween);
    }
}

impl<E> Pause for StreamingSoundHandle<E> {
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.pause(tween);
    }
}

impl<K, V> Pause for RawHandle<K, V>
where
    V: Pause,
{
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.value.pause(tween);
    }
}

impl<K, V, O> Pause for Handle<K, V, O>
where
    V: Pause,
{
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.handle.pause(tween);
    }
}

impl<K, V, O> Pause for Handles<K, V, O>
where
    V: Pause,
{
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.handle.pause(tween));
    }
}

impl<K, O, E> Pause for DualHandles<K, O, E> {
    #[inline]
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
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.value_mut().pause(tween);
    }
}

impl<K, V> Pause for DashMap<K, V>
where
    V: Pause,
    K: Eq + Hash,
{
    #[inline]
    fn pause(&mut self, tween: Tween) {
        self.iter_mut().for_each(|mut x| x.pause(tween));
    }
}

impl<K, O, E> PauseControlled for DualHandles<K, O, E> {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.statics.pause_controlled(id, tween);
        self.streams.pause_controlled(id, tween);
    }
}

impl<K, O> PauseControlled for Handles<K, StaticSoundHandle, O> {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.pause(tween);
            });
    }
}

impl<K, O, E> PauseControlled for Handles<K, StreamingSoundHandle<E>, O> {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.pause(tween);
            });
    }
}
