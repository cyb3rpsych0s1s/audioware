use either::Either;
use kira::{
    PlaySoundError, Tween,
    sound::{FromFileError, static_sound::StaticSoundData, streaming::StreamingSoundData},
};
use red4ext_rs::types::CName;

use crate::engine::traits::{
    compressor::SetControlledCompressor, delay::SetControlledDelay, dilation::SyncDilation,
    distortion::SetControlledDistortion, eq::SetControlledEq, filter::SetControlledFilter,
    pause::Pause, reverb::SetControlledReverb,
};
use crate::engine::{AffectedByTimeDilation, traits::resume::Resume};
use crate::engine::{
    tracks::Spatial,
    traits::{DualHandles, Handle, store::Store},
};
use crate::{ControlId, engine::traits::stop::Stop};

pub struct EmitterEntryOptions {
    pub affected_by_time_dilation: bool,
}

impl AffectedByTimeDilation for EmitterEntryOptions {
    fn affected_by_time_dilation(&self) -> bool {
        self.affected_by_time_dilation
    }
}

/// Underlying handle to the emitter,
/// and whether sound(s) should persist until they finish playing.
pub struct EmitterSlot {
    pub handle: Spatial,
    pub tag_name: Option<CName>,
    pub emitter_name: Option<CName>,
    pub persist_until_sounds_finish: bool,
    pub handles: DualHandles<CName, EmitterEntryOptions, FromFileError>,
}

type PlayResult =
    Result<(f32, Option<CName>), Either<PlaySoundError<()>, PlaySoundError<FromFileError>>>;

impl EmitterSlot {
    pub fn any_playing_handle(&self) -> bool {
        self.handles.any_playing_handle()
    }
    pub fn new(
        handle: Spatial,
        tag_name: CName,
        emitter_name: Option<CName>,
        persist_until_sounds_finish: bool,
    ) -> Self {
        Self {
            handle,
            tag_name: Some(tag_name),
            emitter_name,
            persist_until_sounds_finish,
            handles: DualHandles::default(),
        }
    }
    pub fn play_and_store(
        &mut self,
        event_name: CName,
        affected_by_time_dilation: bool,
        data: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
        control_id: Option<ControlId>,
    ) -> PlayResult {
        match data {
            Either::Left(data) => {
                let duration = data.duration().as_secs_f32();
                let handle = self.handle.play(data).map_err(Either::Left)?;
                self.handles.store(Handle::new(
                    event_name,
                    handle,
                    EmitterEntryOptions {
                        affected_by_time_dilation,
                    },
                    control_id,
                ));
                Ok((duration, self.emitter_name))
            }
            Either::Right(data) => {
                let duration = data.duration().as_secs_f32();
                let handle = self.handle.play(data).map_err(Either::Right)?;
                self.handles.store(Handle::new(
                    event_name,
                    handle,
                    EmitterEntryOptions {
                        affected_by_time_dilation,
                    },
                    control_id,
                ));
                Ok((duration, self.emitter_name))
            }
        }
    }
    pub fn stop(&mut self, tween: Tween) {
        self.handles.stop(tween);
    }
    pub fn pause(&mut self, tween: Tween) {
        self.handles.pause(tween);
    }
    pub fn resume(&mut self, tween: Tween) {
        self.handles.resume(tween);
    }
    pub fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.handles.sync_dilation(rate, tween);
    }
    pub fn occluded(&self) -> bool {
        self.handle.occluded()
    }
}

impl SetControlledEq for EmitterSlot {
    #[inline]
    fn set_controlled_kind(&mut self, id: ControlId, kind: kira::effect::eq_filter::EqFilterKind) {
        SetControlledEq::set_controlled_kind(&mut self.handle, id, kind);
    }

    #[inline]
    fn set_controlled_frequency(&mut self, id: ControlId, frequency: f64, tween: Tween) {
        self.handle.set_controlled_frequency(id, frequency, tween);
    }

    #[inline]
    fn set_controlled_gain(&mut self, id: ControlId, gain: audioware_core::Decibels, tween: Tween) {
        self.handle.set_controlled_gain(id, gain, tween);
    }

    #[inline]
    fn set_controlled_q(&mut self, id: ControlId, q: f64, tween: Tween) {
        self.handle.set_controlled_q(id, q, tween);
    }
}

impl SetControlledDistortion for EmitterSlot {
    #[inline]
    fn set_controlled_kind(
        &mut self,
        id: ControlId,
        kind: kira::effect::distortion::DistortionKind,
    ) {
        SetControlledDistortion::set_controlled_kind(&mut self.handle, id, kind);
    }

    #[inline]
    fn set_controlled_drive(
        &mut self,
        id: ControlId,
        drive: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.handle.set_controlled_drive(id, drive, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        SetControlledDistortion::set_controlled_mix(&mut self.handle, id, mix, tween);
    }
}

impl SetControlledDelay for EmitterSlot {
    #[inline]
    fn set_controlled_feedback(
        &mut self,
        id: ControlId,
        feedback: audioware_core::Decibels,
        tween: Tween,
    ) {
        SetControlledDelay::set_controlled_feedback(&mut self.handle, id, feedback, tween);
    }
    #[inline]
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        SetControlledDelay::set_controlled_mix(&mut self.handle, id, mix, tween);
    }
}

impl SetControlledCompressor for EmitterSlot {
    #[inline]
    fn set_controlled_threshold(
        &mut self,
        id: ControlId,
        threshold: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.handle.set_controlled_threshold(id, threshold, tween);
    }

    #[inline]
    fn set_controlled_ratio(&mut self, id: ControlId, ratio: f32, tween: Tween) {
        self.handle.set_controlled_ratio(id, ratio, tween);
    }

    #[inline]
    fn set_controlled_attack_duration(
        &mut self,
        id: ControlId,
        attack_duration: std::time::Duration,
        tween: Tween,
    ) {
        self.handle
            .set_controlled_attack_duration(id, attack_duration, tween);
    }

    #[inline]
    fn set_controlled_release_duration(
        &mut self,
        id: ControlId,
        release_duration: std::time::Duration,
        tween: Tween,
    ) {
        self.handle
            .set_controlled_release_duration(id, release_duration, tween);
    }

    #[inline]
    fn set_controlled_makeup_gain(
        &mut self,
        id: ControlId,
        makeup_gain: audioware_core::Decibels,
        tween: Tween,
    ) {
        self.handle
            .set_controlled_makeup_gain(id, makeup_gain, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        SetControlledCompressor::set_controlled_mix(&mut self.handle, id, mix, tween);
    }
}

impl SetControlledFilter for EmitterSlot {
    #[inline]
    fn set_controlled_mode(&mut self, id: ControlId, mode: kira::effect::filter::FilterMode) {
        self.handle.set_controlled_mode(id, mode);
    }

    #[inline]
    fn set_controlled_cutoff(&mut self, id: ControlId, cutoff: f32, tween: Tween) {
        self.handle.set_controlled_cutoff(id, cutoff, tween);
    }

    #[inline]
    fn set_controlled_resonance(&mut self, id: ControlId, resonance: f32, tween: Tween) {
        self.handle.set_controlled_resonance(id, resonance, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        SetControlledFilter::set_controlled_mix(&mut self.handle, id, mix, tween);
    }
}

impl SetControlledReverb for EmitterSlot {
    #[inline]
    fn set_controlled_feedback(&mut self, id: ControlId, feedback: f32, tween: Tween) {
        SetControlledReverb::set_controlled_feedback(&mut self.handle, id, feedback, tween);
    }

    #[inline]
    fn set_controlled_damping(&mut self, id: ControlId, damping: f32, tween: Tween) {
        self.handle.set_controlled_damping(id, damping, tween);
    }

    #[inline]
    fn set_controlled_stereo_width(&mut self, id: ControlId, stereo_width: f32, tween: Tween) {
        self.handle
            .set_controlled_stereo_width(id, stereo_width, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        SetControlledReverb::set_controlled_mix(&mut self.handle, id, mix, tween);
    }
}
