use audioware_core::Amplitude;
use kira::Tween;

pub trait SetVolume {
    fn set_volume(&mut self, amplitude: Amplitude, tween: Tween);
}
