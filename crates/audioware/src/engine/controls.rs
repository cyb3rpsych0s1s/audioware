use std::sync::{LazyLock, atomic::AtomicUsize};

use audioware_core::Amplitude;
use kira::{Tween, backend::Backend};

use crate::{
    ControlId,
    engine::{Engine, traits::volume::SetVolume},
};

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

pub fn next_control_id() -> ControlId {
    ControlId::new(&COUNTER)
}

impl<B: Backend> SetVolume for Engine<B> {
    fn set_controlled_volume(&mut self, id: ControlId, amplitude: Amplitude, tween: Tween) {
        self.tracks
            .handles
            .set_controlled_volume(id, amplitude, tween);
    }
}
