use std::time::Duration;

use kira::{
    Decibels, PlaybackRate, StartTime, Tween, Value,
    sound::{IntoOptionalRegion, PlaybackPosition},
};

mod data;
mod settings;
mod types;

pub use settings::SpatialTrackSettings;
pub use types::{Amplitude, AmplitudeError, Panning, PanningError};

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
    fn volume(&self) -> Value<Decibels>;
    fn playback_rate(&self) -> Value<PlaybackRate>;
    fn panning(&self) -> Value<kira::Panning>;
    fn fade_in_tween(&self) -> Option<Tween>;
}

impl<T, U> With<Option<U>> for T
where
    T: With<U> + Sized,
    U: Sized,
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
