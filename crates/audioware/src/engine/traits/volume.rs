use audioware_core::Amplitude;
use kira::{
    Tween,
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
};

use crate::{
    ControlId,
    engine::traits::{DualHandles, Handles},
};

pub trait SetVolume {
    fn set_controlled_volume(&mut self, id: ControlId, amplitude: Amplitude, tween: Tween);
}

impl<K, O, E> SetVolume for DualHandles<K, O, E> {
    fn set_controlled_volume(&mut self, id: ControlId, amplitude: Amplitude, tween: Tween) {
        self.statics.set_controlled_volume(id, amplitude, tween);
        self.streams.set_controlled_volume(id, amplitude, tween);
    }
}

impl<K, O> SetVolume for Handles<K, StaticSoundHandle, O> {
    fn set_controlled_volume(&mut self, id: ControlId, amplitude: Amplitude, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| x.set_volume(amplitude, tween));
    }
}

impl<K, O, E> SetVolume for Handles<K, StreamingSoundHandle<E>, O> {
    fn set_controlled_volume(&mut self, id: ControlId, amplitude: Amplitude, tween: Tween) {
        self.0
            .iter_mut()
            .filter(|x| x.control_id.map(|x| x == id).unwrap_or(false))
            .for_each(|x| x.set_volume(amplitude, tween));
    }
}
