use audioware_core::Amplitude;
use kira::{Tween, backend::Backend};
use portable_atomic::AtomicUsize;
use std::sync::LazyLock;

use crate::{
    ControlId,
    engine::{
        Engine,
        traits::{
            panning::SetControlledPanning,
            pause::PauseControlled,
            playback::SetControlledPlaybackRate,
            position::PositionControlled,
            resume::{ResumeControlled, ResumeControlledAt},
            seek::{SeekControlledBy, SeekControlledTo},
            stop::StopControlled,
            volume::SetControlledVolume,
        },
    },
};

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

pub fn next_control_id() -> ControlId {
    ControlId::new(&COUNTER)
}

impl<B: Backend> SetControlledVolume for Engine<B> {
    fn set_controlled_volume(&mut self, id: ControlId, amplitude: Amplitude, tween: Tween) {
        self.tracks
            .handles
            .set_controlled_volume(id, amplitude, tween);
    }
}

impl<B: Backend> SetControlledPlaybackRate for Engine<B> {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.tracks
            .handles
            .set_controlled_playback_rate(id, rate, tween);
    }
}

impl<B: Backend> SetControlledPanning for Engine<B> {
    fn set_controlled_panning(&mut self, id: ControlId, panning: kira::Panning, tween: Tween) {
        self.tracks
            .handles
            .set_controlled_panning(id, panning, tween);
    }
}

impl<B: Backend> PositionControlled for Engine<B> {
    fn position_controlled(&mut self, id: ControlId, sender: crossbeam::channel::Sender<f32>) {
        self.tracks.handles.position_controlled(id, sender);
    }
}

impl<B: Backend> StopControlled for Engine<B> {
    fn stop_controlled(&mut self, id: ControlId, tween: Tween) {
        self.tracks.handles.stop_controlled(id, tween);
    }
}

impl<B: Backend> PauseControlled for Engine<B> {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.tracks.handles.pause_controlled(id, tween);
    }
}

impl<B: Backend> ResumeControlled for Engine<B> {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.tracks.handles.resume_controlled(id, tween);
    }
}

impl<B: Backend> ResumeControlledAt for Engine<B> {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.tracks.handles.resume_controlled_at(id, delay, tween);
    }
}

impl<B: Backend> SeekControlledTo for Engine<B> {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.tracks.handles.seek_controlled_to(id, position);
    }
}

impl<B: Backend> SeekControlledBy for Engine<B> {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.tracks.handles.seek_controlled_by(id, amount);
    }
}
