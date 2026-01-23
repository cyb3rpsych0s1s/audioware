use kira::sound::streaming::StreamingSoundHandle;

use crate::engine::{
    traits::{DualHandles, Handles, clear::Clear, stop::Stop},
    tweens::IMMEDIATELY,
};

pub trait Terminate {
    fn terminate(&mut self);
}

impl<K, O, E> Terminate for DualHandles<K, O, E> {
    fn terminate(&mut self) {
        self.streams.stop(IMMEDIATELY);
        self.streams.clear();
    }
}

impl<K, O, E> Terminate for Handles<K, StreamingSoundHandle<E>, O> {
    fn terminate(&mut self) {
        self.stop(IMMEDIATELY);
        self.clear();
    }
}
