use std::time::Duration;

use kira::{
    sound::{IntoOptionalRegion, PlaybackPosition, PlaybackRate},
    tween::{Tween, Value},
    OutputDestination, StartTime, Volume,
};

mod data;
mod settings;

/// Consumes `T`, absorbing its data.
pub trait With<T> {
    fn with(self, settings: T) -> Self
    where
        Self: Sized;
}

pub trait AudioDuration {
    /// Current audio duration, based on its slice.
    ///
    /// Equivalent to [kira] duration.
    fn slice_duration(&self) -> Duration;

    /// Total duration, regardless of slice and settings.
    ///
    /// Requires `self` for [StreamingSoundData][kira::sound::streaming::StreamingSoundData].
    fn total_duration(self) -> Duration;
}

/// Any audio whose sample rate is known.
pub trait SampleRate {
    fn sample_rate(&self) -> u32;
}

/// Audio settings.
pub trait AudioSettings {
    fn start_time(&self) -> StartTime;
    fn start_position(&self) -> PlaybackPosition;
    fn region(&self) -> impl IntoOptionalRegion;
    fn r#loop(&self) -> bool;
    fn volume(&self) -> Value<Volume>;
    fn playback_rate(&self) -> Value<PlaybackRate>;
    fn panning(&self) -> Value<f64>;
    fn output_destination(&self) -> OutputDestination;
    fn fade_in_tween(&self) -> Option<Tween>;
}

impl<T, U> With<Option<U>> for T
where
    T: With<U>,
{
    /// Consumes `T`, absorbing its data if any.
    #[inline]
    fn with(mut self, settings: Option<U>) -> Self
    where
        Self: Sized,
    {
        if let Some(settings) = settings {
            self = self.with(settings);
        }
        self
    }
}
