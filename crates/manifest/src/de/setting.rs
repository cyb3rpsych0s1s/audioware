use std::time::Duration;

use audioware_core::{Amplitude, With};
use either::Either;
use kira::{
    sound::{
        static_sound::StaticSoundData, streaming::StreamingSoundData, EndPosition, FromFileError,
        IntoOptionalRegion, PlaybackPosition,
    },
    Decibels, PlaybackRate, Semitones, StartTime, Tween,
};
use serde::Deserialize;

use crate::error::ValidationError;

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

pub trait ValidateFor<T> {
    fn validate_for(&self, rhs: &T) -> Result<(), Vec<ValidationError>>;
}

/// Deserialization type
/// for [kira::sound::static_sound::StaticSoundSettings]
/// and [kira::sound::streaming::StreamingSoundSettings].
#[derive(Debug, Default, Deserialize, Clone)]
pub struct Settings {
    #[serde(with = "humantime_serde", default)]
    pub start_time: Option<Duration>,
    #[serde(with = "humantime_serde", default)]
    pub start_position: Option<Duration>,
    pub volume: Option<f32>,
    pub panning: Option<f32>,
    #[serde(rename = "loop")]
    pub r#loop: Option<bool>,
    pub region: Option<self::Region>,
    #[serde(deserialize_with = "factor_or_semitones", default)]
    pub playback_rate: Option<PlaybackRate>,
    pub fade_in_tween: Option<Interpolation>,
    pub affected_by_time_dilation: Option<bool>,
}

macro_rules! impl_with {
    ($self:expr, $settings:expr) => {{
        if let Some(x) = $settings.start_time.map(StartTime::Delayed) {
            $self = $self.start_time(x);
        }
        if let Some(x) = $settings
            .start_position
            .as_ref()
            .map(Duration::as_secs_f64)
            .map(PlaybackPosition::Seconds)
        {
            $self = $self.start_position(x);
        }
        if let Some(x) = $settings.volume {
            $self = $self.volume(x);
        }
        if let Some(x) = $settings.panning {
            $self = $self.panning(x);
        }
        if $settings.region.is_some() || $settings.r#loop.unwrap_or(false) {
            if let Some(x) = $settings.region {
                if $settings.r#loop.unwrap_or(false) {
                    $self = $self.loop_region(x);
                } else {
                    $self = $self.slice(x);
                }
            } else {
                $self = $self.loop_region(kira::sound::Region {
                    start: PlaybackPosition::Seconds(0.),
                    end: EndPosition::EndOfAudio,
                });
            }
        }
        if let Some(x) = $settings.playback_rate {
            $self = $self.playback_rate(x);
        }
        if let Some(x) = $settings.fade_in_tween.map(Into::<Tween>::into) {
            $self = $self.fade_in_tween(x);
        }
        $self
    }};
}

impl With<Settings> for StaticSoundData {
    fn with(mut self, settings: Settings) -> Self
    where
        Self: Sized,
    {
        impl_with!(self, settings)
    }
}

impl<T> With<Settings> for StreamingSoundData<T>
where
    T: Send + 'static,
{
    fn with(mut self, settings: Settings) -> Self
    where
        Self: Sized,
    {
        impl_with!(self, settings)
    }
}

impl<T> With<Settings> for Either<StaticSoundData, StreamingSoundData<T>>
where
    T: Send + 'static,
{
    fn with(self, settings: Settings) -> Self
    where
        Self: Sized,
    {
        self.map_either_with(
            settings,
            |settings, x| x.with(settings),
            |settings, x| x.with(settings),
        )
    }
}

/// Deserialization type
/// for [kira::sound::Region].
#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    #[serde(with = "humantime_serde", default)]
    pub starts: Option<Duration>,
    #[serde(with = "humantime_serde", default)]
    pub ends: Option<Duration>,
}

impl Region {
    pub fn starts(&self) -> Option<PlaybackPosition> {
        self.starts
            .as_ref()
            .map(Duration::as_secs_f64)
            .map(PlaybackPosition::Seconds)
    }
    pub fn ends(&self) -> Option<EndPosition> {
        self.ends
            .as_ref()
            .map(Duration::as_secs_f64)
            .map(PlaybackPosition::Seconds)
            .map(EndPosition::Custom)
    }
}

impl IntoOptionalRegion for self::Region {
    fn into_optional_region(self) -> Option<kira::sound::Region> {
        if self.starts.is_none() && self.ends.is_none() {
            return None;
        }
        Some(kira::sound::Region {
            start: PlaybackPosition::Seconds(
                self.starts
                    .as_ref()
                    .map(Duration::as_secs_f64)
                    .unwrap_or(0.),
            ),
            end: self
                .ends
                .as_ref()
                .map(Duration::as_secs_f64)
                .map(PlaybackPosition::Seconds)
                .map(EndPosition::Custom)
                .unwrap_or(EndPosition::EndOfAudio),
        })
    }
}

fn factor_or_semitones<'de, D>(deserializer: D) -> Result<Option<PlaybackRate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        let s = s.trim();
        if s.starts_with('x') || s.starts_with('X') {
            return Ok(Some(PlaybackRate(
                s[1..].trim().parse().map_err(serde::de::Error::custom)?,
            )));
        }
        if s.ends_with('♯') {
            return Ok(Some(
                Semitones(
                    s[..(s.len() - '♯'.len_utf8())]
                        .trim()
                        .parse()
                        .map_err(serde::de::Error::custom)?,
                )
                .into(),
            ));
        }
        if s.ends_with('♭') {
            return Ok(Some(
                Semitones(
                    -s[..(s.len() - '♭'.len_utf8())]
                        .trim()
                        .parse()
                        .map_err(serde::de::Error::custom)?,
                )
                .into(),
            ));
        }
        return Err(serde::de::Error::custom(format!(
            "invalid factor or semitone: {s}"
        )));
    }
    Ok(None)
}

/// Deserialization type for [kira::Tween].
#[derive(Debug, Deserialize, Clone)]
pub struct Interpolation {
    #[serde(with = "humantime_serde", default)]
    pub start_time: Option<Duration>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    #[serde(flatten)]
    pub easing: kira::Easing,
}

impl From<Interpolation> for Tween {
    fn from(value: Interpolation) -> Self {
        Self {
            start_time: value
                .start_time
                .map(|x| match x {
                    x if x == Duration::default() => StartTime::Immediate,
                    x => StartTime::Delayed(x),
                })
                .unwrap_or(StartTime::Immediate),
            duration: value.duration,
            easing: value.easing,
        }
    }
}

macro_rules! impl_from_settings {
    ($into:path) => {
        impl From<self::Settings> for $into {
            fn from(value: self::Settings) -> Self {
                Self {
                    start_time: value
                        .start_time
                        .map(::kira::StartTime::Delayed)
                        .unwrap_or_default(),
                    start_position: value
                        .start_position
                        .map(|x| ::kira::sound::PlaybackPosition::Seconds(x.as_secs_f64()))
                        .unwrap_or_default(),
                    volume: value
                        .volume
                        .map(|x| {
                            ::kira::Value::<::kira::Decibels>::Fixed(
                                Amplitude::try_from(x).unwrap().as_decibels(),
                            )
                        })
                        .unwrap_or_default(),
                    panning: value
                        .panning
                        .map(|x| ::kira::Panning(x))
                        .map(Into::into)
                        .unwrap_or_default(),
                    fade_in_tween: value.fade_in_tween.map(Into::into),
                    loop_region: if value.r#loop.unwrap_or_default() {
                        value
                            .region
                            .and_then(IntoOptionalRegion::into_optional_region)
                    } else {
                        None
                    },
                    playback_rate: value.playback_rate.map(Into::into).unwrap_or_default(),
                    ..Default::default()
                }
            }
        }
    };
}

impl_from_settings!(::kira::sound::static_sound::StaticSoundSettings);
impl_from_settings!(::kira::sound::streaming::StreamingSoundSettings);

impl Validate for Settings {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors: Vec<_> = vec![];
        if let Some(panning) = self.panning {
            if !(0.0..=1.0).contains(&panning) {
                errors.push(ValidationError {
                    which: "panning",
                    why: "must be a value between 0.0 and 1.0 (inclusive)",
                });
            }
        }
        if let Some(volume) = self.volume {
            match Amplitude::try_from(volume) {
                Ok(volume) => {
                    if volume.as_decibels() > Decibels(85.0) {
                        errors.push(ValidationError {
                            which: "volume",
                            why: "audio should not be louder than 85.0 dB",
                        });
                    }
                }
                Err(_) => {
                    errors.push(ValidationError {
                        which: "volume",
                        why: "cannot be negative",
                    });
                }
            };
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
}

impl Validate for Option<&Settings> {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        self.map(Validate::validate).unwrap_or(Ok(()))
    }
}

impl ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>> for Settings {
    fn validate_for(
        &self,
        audio: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> Result<(), Vec<ValidationError>> {
        let mut errors = vec![];
        if let Some(ref region) = self.region {
            let total_duration = match audio {
                Either::Left(x) => x.total_duration(),
                Either::Right(x) => x.total_duration(),
            }
            .as_secs_f64();
            let start: f64 = match (region.starts(), audio) {
                (Some(PlaybackPosition::Seconds(seconds)), _) => seconds,
                (Some(PlaybackPosition::Samples(samples)), Either::Left(data)) => {
                    samples as f64 / data.sample_rate as f64
                }
                // no sample rate method yet
                (Some(PlaybackPosition::Samples(_)), Either::Right(_)) => {
                    errors.push(ValidationError {
                        which: "region.starts",
                        why: "samples unit is not supported with streaming sound yet",
                    });
                    return Err(errors);
                }
                // none implicitly means beginning of the audio
                (None, _) => 0.0,
            };
            let end: f64 = match (region.ends(), audio) {
                (Some(EndPosition::Custom(PlaybackPosition::Seconds(x))), _) => x,
                (
                    Some(EndPosition::Custom(PlaybackPosition::Samples(samples))),
                    Either::Left(data),
                ) => samples as f64 / data.sample_rate as f64,
                // no sample rate method yet
                (Some(EndPosition::Custom(PlaybackPosition::Samples(_))), Either::Right(_)) => {
                    errors.push(ValidationError { which: "region.ends", why: "samples unit is not supported with streaming sound yet" });
                    return Err(errors);
                }
                (Some(EndPosition::EndOfAudio), Either::Left(_)) |
                (Some(EndPosition::EndOfAudio), Either::Right(_)) |
                // none implicitly means end of the audio
                (None, _) => total_duration,
            };
            if start < 0.
                || end <= 0.
                || start >= total_duration
                || end > total_duration
                || start >= end
            {
                errors.push(ValidationError {
                    which: "region",
                    why: "must be within audio duration and starts before it ends",
                });
            }
            if let Some(start_position) = self.start_position.map(|x| x.as_secs_f64()) {
                if start_position >= end {
                    errors.push(ValidationError {
                        which: "start_position",
                        why: "greater than audio duration",
                    });
                }
            }
        } else if let Some(start_position) = self.start_position.map(|x| x.as_secs_f64()) {
            let duration = match audio {
                Either::Left(x) => x.duration(),
                Either::Right(x) => x.duration(),
            }
            .as_secs_f64();
            if start_position >= duration {
                errors.push(ValidationError {
                    which: "start_position",
                    why: "greater than audio duration",
                });
            }
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
}

impl ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>> for Option<&Settings> {
    fn validate_for(
        &self,
        rhs: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> Result<(), Vec<ValidationError>> {
        self.map(|x| x.validate_for(rhs)).unwrap_or(Ok(()))
    }
}

impl ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>> for Tween {
    #[inline]
    fn validate_for(
        &self,
        _: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> Result<(), Vec<ValidationError>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Settings;

    mod duration {
        use test_case::test_case;

        #[test_case(r##"120ms"##; "valid milliseconds duration")]
        fn valid_ms(yaml: &str) {
            let settings = humantime::parse_duration(yaml);
            dbg!("{}", &settings);
            assert!(settings.is_ok());
        }

        #[test_case(r##"9s"##; "valid seconds duration")]
        fn valid_s(yaml: &str) {
            let settings = humantime::parse_duration(yaml);
            dbg!("{}", &settings);
            assert!(settings.is_ok());
        }

        #[test_case(r##"1.2s"## ; "invalid duration")]
        fn invalid(yaml: &str) {
            let settings = humantime::parse_duration(yaml);
            dbg!("{}", &settings);
            assert!(settings.is_err());
        }
    }

    #[test_case(r##"settings:
    start_time: 120ms"## ; "start time")]
    #[test_case(r##"settings:
    playback_rate: 2♯"## ; "playback rate with sharp semitones")]
    #[test_case(r##"settings:
    playback_rate: 2♭"## ; "playback rate with flat semitones")]
    #[test_case(r##"settings:
    region:
        ends: 8s"## ; "region with only ends specified")]
    #[test_case(r##"settings:
    region:
        starts: 120ms
        ends: 8s
    loop: true"## ; "region with both starts and ends + loop specified")]
    #[test_case(r##"settings:
    start_time: 120ms
    volume: 0.5"## ; "start time + volume")]
    #[test_case(r##"settings:
    fade_in_tween:
        duration: 1s
        Linear:"## ; "linear fade-in tween")]
    #[test_case(r##"settings:
    fade_in_tween:
        duration: 1s
        InPowf: 0.5"## ; "elastic fade-in tween")]
    #[test_case(r##"settings:
    start_time: 5s
    fade_in_tween:
        duration: 9s
        InPowi: 2"## ; "complex settings")]
    fn settings(yaml: &str) {
        let settings = serde_yaml::from_str::<HashMap<String, Settings>>(yaml);
        dbg!("{}", &settings);
        assert!(settings.is_ok());
    }
}
