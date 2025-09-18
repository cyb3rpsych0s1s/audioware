use std::hash::Hash;

use dashmap::mapref::multiple::RefMutMulti;
use kira::Tween;

use crate::engine::{
    AffectedByTimeDilation, DilationUpdate,
    traits::{DualHandles, Handle, Handles, playback::SetPlaybackRate},
};

pub trait Comparable<K> {
    fn compare(&self, rhs: &K) -> bool;
}

pub trait SyncDilation {
    fn sync_dilation(&mut self, rate: f64, tween: Tween);
}

pub trait SyncDilationBy<F> {
    fn sync_dilation_by(&mut self, key: &F, update: &DilationUpdate);
}

impl<K, V, O> SyncDilation for Handle<K, V, O>
where
    V: SetPlaybackRate,
    O: AffectedByTimeDilation,
{
    #[inline]
    fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        if self.options.affected_by_time_dilation() {
            self.handle.value.set_playback_rate(rate, tween);
        }
    }
}

impl<K, V, O, F> SyncDilationBy<F> for Handle<K, V, O>
where
    V: SetPlaybackRate,
    K: Eq + Hash,
    O: AffectedByTimeDilation + Comparable<F>,
{
    #[inline]
    fn sync_dilation_by(&mut self, key: &F, update: &DilationUpdate) {
        if self.options.compare(key) && self.options.affected_by_time_dilation() {
            self.handle
                .value
                .set_playback_rate(update.dilation(), update.tween_curve());
        }
    }
}

impl<K, V, O, F> SyncDilationBy<F> for Handles<K, V, O>
where
    V: SetPlaybackRate,
    K: Eq + Hash,
    O: AffectedByTimeDilation + Comparable<F>,
{
    #[inline]
    fn sync_dilation_by(&mut self, key: &F, update: &DilationUpdate) {
        self.0
            .iter_mut()
            .for_each(|x| x.sync_dilation_by(key, update));
    }
}

impl<K, O, E, F> SyncDilationBy<F> for DualHandles<K, O, E>
where
    K: Eq + Hash,
    O: AffectedByTimeDilation + Comparable<F>,
{
    #[inline]
    fn sync_dilation_by(&mut self, key: &F, update: &DilationUpdate) {
        self.statics.sync_dilation_by(key, update);
        self.streams.sync_dilation_by(key, update);
    }
}

impl<K, V, O> SyncDilation for Handles<K, V, O>
where
    V: SetPlaybackRate,
    O: AffectedByTimeDilation,
{
    #[inline]
    fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.0.iter_mut().for_each(|x| x.sync_dilation(rate, tween));
    }
}

impl<K, O, E> SyncDilation for DualHandles<K, O, E>
where
    O: AffectedByTimeDilation,
{
    #[inline]
    fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.statics.sync_dilation(rate, tween);
        self.streams.sync_dilation(rate, tween);
    }
}

impl<K, V> SyncDilation for RefMutMulti<'_, K, V>
where
    V: SyncDilation,
    K: Eq + Hash,
{
    #[inline]
    fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.value_mut().sync_dilation(rate, tween);
    }
}
