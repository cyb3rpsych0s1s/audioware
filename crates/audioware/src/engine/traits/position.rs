use crossbeam::channel::Sender;
use kira::sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handles},
    utils::warns,
};

pub trait PositionControlled {
    fn position_controlled(&mut self, id: ControlId, sender: Sender<f32>);
}

impl<K, O, E> PositionControlled for DualHandles<K, O, E> {
    fn position_controlled(&mut self, id: ControlId, sender: Sender<f32>) {
        if let Some(x) = self
            .statics
            .position_controlled(id)
            .or(self.streams.position_controlled(id))
            && let Err(e) = sender.send(x)
        {
            warns!("unable to send dynamic sound position ({id}): {e}");
        }
    }
}

impl<K, O> Handles<K, StaticSoundHandle, O> {
    fn position_controlled(&mut self, id: ControlId) -> Option<f32> {
        self.0
            .iter()
            .find(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .map(|x| x.handle.value.position() as f32)
    }
}

impl<K, O, E> Handles<K, StreamingSoundHandle<E>, O> {
    fn position_controlled(&mut self, id: ControlId) -> Option<f32> {
        self.0
            .iter()
            .find(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .map(|x| x.handle.value.position() as f32)
    }
}
