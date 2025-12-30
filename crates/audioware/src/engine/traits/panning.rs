use kira::{
    Panning, Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handles},
};

pub trait SetControlledPanning {
    fn set_controlled_panning(&mut self, id: ControlId, panning: Panning, tween: Tween);
}

impl<K, O, E> SetControlledPanning for DualHandles<K, O, E> {
    fn set_controlled_panning(&mut self, id: ControlId, panning: Panning, tween: Tween) {
        self.statics.set_controlled_panning(id, panning, tween);
        self.streams.set_controlled_panning(id, panning, tween);
    }
}

impl<K, O> SetControlledPanning for Handles<K, StaticSoundHandle, O> {
    fn set_controlled_panning(&mut self, id: ControlId, panning: Panning, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| x.set_panning(panning, tween));
    }
}

impl<K, O, E> SetControlledPanning for Handles<K, StreamingSoundHandle<E>, O> {
    fn set_controlled_panning(&mut self, id: ControlId, panning: Panning, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| x.set_panning(panning, tween));
    }
}
