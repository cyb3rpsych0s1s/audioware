use kira::sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handles},
};

pub trait SeekControlledTo {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64);
}

pub trait SeekControlledBy {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64);
}

impl<K, O, E> SeekControlledTo for DualHandles<K, O, E> {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.statics.seek_controlled_to(id, position);
        self.streams.seek_controlled_to(id, position);
    }
}

impl<K, O, E> SeekControlledBy for DualHandles<K, O, E> {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.statics.seek_controlled_by(id, amount);
        self.streams.seek_controlled_by(id, amount);
    }
}

impl<K, O> SeekControlledTo for Handles<K, StaticSoundHandle, O> {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.seek_to(position);
            });
    }
}

impl<K, O, E> SeekControlledTo for Handles<K, StreamingSoundHandle<E>, O> {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.seek_to(position);
            });
    }
}

impl<K, O> SeekControlledBy for Handles<K, StaticSoundHandle, O> {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.seek_by(amount);
            });
    }
}

impl<K, O, E> SeekControlledBy for Handles<K, StreamingSoundHandle<E>, O> {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| {
                x.handle.value.seek_by(amount);
            });
    }
}
