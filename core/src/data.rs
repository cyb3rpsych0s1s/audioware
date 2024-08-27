// data.rs
use std::time::Duration;

use kira::sound::IntoOptionalRegion;

use crate::{AudioData, SampleRate};

mod r#static;
mod streaming;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Data<T> {
    Vanilla(T),
    Modified(T),
}

#[allow(dead_code)]
impl<T> Data<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self::Vanilla(inner)
    }
}

impl<T> AudioData for Data<T>
where
    T: AudioData,
{
    type Settings = <T as AudioData>::Settings;

    #[inline]
    fn duration(&self) -> Duration {
        match self {
            Data::Vanilla(data) | Data::Modified(data) => data.duration(),
        }
    }

    #[inline]
    fn total_duration(self) -> Duration {
        match self {
            Data::Vanilla(data) | Data::Modified(data) if data.slice().is_none() => {
                data.total_duration()
            }
            Data::Vanilla(data) | Data::Modified(data) => data.with_slice(None).duration(),
        }
    }

    #[inline]
    fn settings(&self) -> &Self::Settings {
        match self {
            Data::Vanilla(data) | Data::Modified(data) => data.settings(),
        }
    }

    #[inline]
    fn slice(&self) -> Option<(usize, usize)> {
        match self {
            Data::Vanilla(data) | Data::Modified(data) => data.slice(),
        }
    }

    #[inline]
    fn with_slice(self, region: impl IntoOptionalRegion) -> Self {
        match self {
            Data::Vanilla(data) | Data::Modified(data) => Data::Modified(data.with_slice(region)),
        }
    }
}

impl<T> SampleRate for Data<T>
where
    T: SampleRate,
{
    #[inline]
    fn sample_rate(&self) -> u32 {
        match self {
            Data::Vanilla(data) | Data::Modified(data) => data.sample_rate(),
        }
    }
}

macro_rules! impl_with {
    ($self:expr, $settings:expr) => {{
        if let Some(x) = $settings.start_time() {
            $self = $self.start_time(x);
        }
        if let Some(x) = $settings.start_position() {
            $self = $self.start_position(x);
        }
        if let Some(x) = $settings.volume() {
            $self = $self.volume(x);
        }
        if let Some(x) = $settings.playback_rate() {
            $self = $self.playback_rate(x);
        }
        if let Some(x) = $settings.panning() {
            $self = $self.panning(x);
        }
        if let Some(x) = $settings.output_destination() {
            $self = $self.output_destination(x);
        }
        if let Some(x) = $settings.fade_in_tween() {
            $self = $self.fade_in_tween(x);
        }
        let (r#loop, region) = ($settings.r#loop().unwrap_or(false), $settings.region());
        let has = region.is_some();
        let region = region
            .and_then(::kira::sound::IntoOptionalRegion::into_optional_region)
            .unwrap_or(::kira::sound::Region {
                start: ::kira::sound::PlaybackPosition::Seconds(0.),
                end: ::kira::sound::EndPosition::EndOfAudio,
            });
        if r#loop {
            $self = $self.loop_region(region);
        } else if has {
            $self = $self.slice(region);
        }
        $self
    }};
}
use impl_with;
