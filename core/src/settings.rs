// settings.rs
use kira::{
    sound::{
        static_sound::StaticSoundSettings, streaming::StreamingSoundSettings, IntoOptionalRegion,
        PlaybackPosition, PlaybackRate,
    },
    tween::{Tween, Value},
    OutputDestination, StartTime, Volume,
};

use crate::AudioSettings;

impl AudioSettings for StaticSoundSettings {
    #[inline]
    fn start_position(&self) -> &PlaybackPosition {
        &self.start_position
    }

    #[inline]
    fn region(&self) -> impl IntoOptionalRegion {
        self.loop_region
    }

    #[inline]
    fn r#loop(&self) -> bool {
        self.loop_region.is_some()
    }

    #[inline]
    fn volume(&self) -> &Value<Volume> {
        &self.volume
    }

    #[inline]
    fn playback_rate(&self) -> Value<PlaybackRate> {
        self.playback_rate
    }

    #[inline]
    fn panning(&self) -> Value<f64> {
        self.panning
    }

    #[inline]
    fn start_time(&self) -> StartTime {
        self.start_time
    }

    #[inline]
    fn output_destination(&self) -> OutputDestination {
        self.output_destination
    }

    #[inline]
    fn fade_in_tween(&self) -> Option<Tween> {
        self.fade_in_tween
    }
}

impl AudioSettings for StreamingSoundSettings {
    #[inline]
    fn start_time(&self) -> StartTime {
        self.start_time
    }

    #[inline]
    fn start_position(&self) -> &PlaybackPosition {
        &self.start_position
    }

    #[inline]
    fn region(&self) -> impl IntoOptionalRegion {
        self.loop_region
    }

    #[inline]
    fn r#loop(&self) -> bool {
        self.loop_region.is_some()
    }

    #[inline]
    fn volume(&self) -> &Value<Volume> {
        &self.volume
    }

    #[inline]
    fn playback_rate(&self) -> Value<PlaybackRate> {
        self.playback_rate
    }

    #[inline]
    fn panning(&self) -> Value<f64> {
        self.panning
    }

    #[inline]
    fn output_destination(&self) -> OutputDestination {
        self.output_destination
    }

    #[inline]
    fn fade_in_tween(&self) -> Option<Tween> {
        self.fade_in_tween
    }
}
