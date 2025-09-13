use std::hash::Hash;

use dashmap::mapref::multiple::RefMutMulti;
use kira::{
    Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};
use red4ext_rs::types::{CName, Cruid, EntityId};

use crate::engine::{
    tracks::TrackEntryOptions,
    traits::{DualHandles, Handle, Handles, RawHandle},
};

pub trait Stop {
    fn stop(&mut self, tween: Tween);
}

pub trait StopBy<K> {
    fn stop_by(&mut self, key: &K, tween: Tween);
}

impl Stop for StaticSoundHandle {
    fn stop(&mut self, tween: Tween) {
        self.stop(tween);
    }
}

impl<E> Stop for StreamingSoundHandle<E> {
    fn stop(&mut self, tween: Tween) {
        self.stop(tween);
    }
}

impl<K, V, O> Stop for Handle<K, V, O>
where
    V: Stop,
{
    fn stop(&mut self, tween: Tween) {
        self.handle.value.stop(tween);
    }
}

impl<K, V, O> StopBy<K> for Handle<K, V, O>
where
    K: PartialEq,
    V: Stop,
{
    fn stop_by(&mut self, key: &K, tween: Tween) {
        self.handle.stop_by(key, tween);
    }
}

impl<V> StopBy<(CName, Option<EntityId>, Option<CName>)> for Handles<CName, V, TrackEntryOptions>
where
    V: Stop,
{
    fn stop_by(&mut self, key: &(CName, Option<EntityId>, Option<CName>), tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| {
                x.handle.key == key.0
                    && x.options.entity_id == key.1
                    && x.options.emitter_name == key.2
            })
            .for_each(|x| x.stop(tween));
    }
}

impl<E> StopBy<(CName, Option<EntityId>, Option<CName>)>
    for DualHandles<CName, TrackEntryOptions, E>
{
    fn stop_by(&mut self, key: &(CName, Option<EntityId>, Option<CName>), tween: Tween) {
        self.statics.stop_by(key, tween);
        self.streams.stop_by(key, tween);
    }
}

impl<E> StopBy<Cruid> for DualHandles<Cruid, (), E> {
    fn stop_by(&mut self, key: &Cruid, tween: Tween) {
        self.statics.stop_by(key, tween);
        self.streams.stop_by(key, tween);
    }
}

impl<K, V> Stop for RawHandle<K, V>
where
    V: Stop,
{
    fn stop(&mut self, tween: Tween) {
        self.value.stop(tween);
    }
}

impl<K, V> StopBy<K> for RawHandle<K, V>
where
    K: PartialEq,
    V: Stop,
{
    fn stop_by(&mut self, key: &K, tween: Tween) {
        if *key == self.key {
            self.value.stop(tween);
        }
    }
}

impl<K, V, O> Stop for Handles<K, V, O>
where
    V: Stop,
{
    fn stop(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.stop(tween));
    }
}

impl<K, V, O> StopBy<K> for Handles<K, V, O>
where
    K: PartialEq,
    V: Stop,
{
    fn stop_by(&mut self, key: &K, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.stop_by(key, tween));
    }
}

impl<K, O, E> Stop for DualHandles<K, O, E> {
    fn stop(&mut self, tween: Tween) {
        self.statics.stop(tween);
        self.streams.stop(tween);
    }
}

impl<O, E> StopBy<CName> for DualHandles<CName, O, E> {
    fn stop_by(&mut self, key: &CName, tween: Tween) {
        self.statics.stop_by(key, tween);
        self.streams.stop_by(key, tween);
    }
}

impl<K, V> Stop for RefMutMulti<'_, K, V>
where
    V: Stop,
    K: Eq + Hash,
{
    fn stop(&mut self, tween: Tween) {
        self.value_mut().stop(tween);
    }
}
