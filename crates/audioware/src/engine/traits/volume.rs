use audioware_core::Amplitude;
use kira::Tween;

use crate::engine::traits::AnyHandle;

pub trait SetVolume {
    fn set_volume(&mut self, amplitude: Amplitude, tween: Tween);
}
