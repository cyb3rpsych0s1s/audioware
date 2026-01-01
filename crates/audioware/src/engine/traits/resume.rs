use std::{hash::Hash, time::Duration};

use dashmap::{DashMap, mapref::multiple::RefMutMulti};
use kira::{
    StartTime, Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handle, Handles, RawHandle},
};

pub trait Resume {
    fn resume(&mut self, tween: Tween);
}

pub trait ResumeControlled {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween);
}

pub trait ResumeControlledAt {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween);
}

impl Resume for StaticSoundHandle {
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.resume(tween);
    }
}

impl<E> Resume for StreamingSoundHandle<E> {
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.resume(tween);
    }
}

impl<K, V> Resume for RawHandle<K, V>
where
    V: Resume,
{
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.value.resume(tween);
    }
}

impl<K, V, O> Resume for Handle<K, V, O>
where
    V: Resume,
{
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.handle.resume(tween);
    }
}

impl<K, V, O> Resume for Handles<K, V, O>
where
    V: Resume,
{
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.handle.resume(tween));
    }
}

impl<K, O, E> Resume for DualHandles<K, O, E> {
    #[inline]
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
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.value_mut().resume(tween);
    }
}

impl<K, V> Resume for DashMap<K, V>
where
    V: Resume,
    K: Eq + Hash,
{
    #[inline]
    fn resume(&mut self, tween: Tween) {
        self.iter_mut().for_each(|mut x| x.resume(tween));
    }
}

impl<K, O, E> ResumeControlled for DualHandles<K, O, E> {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.statics.resume_controlled(id, tween);
        self.streams.resume_controlled(id, tween);
    }
}

impl<K, O, E> ResumeControlledAt for DualHandles<K, O, E> {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.statics.resume_controlled_at(id, delay, tween);
        self.streams.resume_controlled_at(id, delay, tween);
    }
}

impl<K, O> ResumeControlled for Handles<K, StaticSoundHandle, O> {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.resume(tween);
            });
    }
}

impl<K, O, E> ResumeControlled for Handles<K, StreamingSoundHandle<E>, O> {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.resume(tween);
            });
    }
}

impl<K, O> ResumeControlledAt for Handles<K, StaticSoundHandle, O> {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle
                    .value
                    .resume_at(StartTime::Delayed(Duration::from_secs_f64(delay)), tween);
            });
    }
}

impl<K, O, E> ResumeControlledAt for Handles<K, StreamingSoundHandle<E>, O> {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle
                    .value
                    .resume_at(StartTime::Delayed(Duration::from_secs_f64(delay)), tween);
            });
    }
}
