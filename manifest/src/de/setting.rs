use std::time::Duration;

use kira::{
    sound::{EndPosition, IntoOptionalRegion, PlaybackPosition, PlaybackRate},
    tween::Tween,
    StartTime, Volume,
};
use serde::Deserialize;

/// Interop type
/// for [kira::sound::static_sound::StaticSoundSettings]
/// and [kira::sound::streaming::StreamingSoundSettings].
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(with = "humantime_serde", default)]
    pub start_time: Option<Duration>,
    #[serde(with = "humantime_serde", default)]
    pub start_position: Option<Duration>,
    pub volume: Option<f64>,
    pub panning: Option<f64>,
    #[serde(rename = "loop")]
    pub r#loop: Option<bool>,
    pub region: Option<self::Region>,
    #[serde(deserialize_with = "factor_or_semitones", default)]
    pub playback_rate: Option<PlaybackRate>,
    pub fade_in_tween: Option<Interpolation>,
}

/// Interop type
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
            .map(|x| PlaybackPosition::Seconds(x.as_secs_f64()))
    }
    pub fn ends(&self) -> Option<EndPosition> {
        self.ends
            .map(|x| PlaybackPosition::Seconds(x.as_secs_f64()))
            .map(EndPosition::Custom)
    }
}

impl IntoOptionalRegion for self::Region {
    fn into_optional_region(self) -> Option<kira::sound::Region> {
        match (self.starts, self.ends) {
            (None, None) => None,
            (None, Some(ends)) => Some(kira::sound::Region {
                start: PlaybackPosition::Seconds(0.),
                end: EndPosition::Custom(PlaybackPosition::Seconds(ends.as_secs_f64())),
            }),
            (Some(starts), None) => Some(kira::sound::Region {
                start: PlaybackPosition::Seconds(starts.as_secs_f64()),
                end: EndPosition::EndOfAudio,
            }),
            (Some(starts), Some(ends)) => Some(kira::sound::Region {
                start: PlaybackPosition::Seconds(starts.as_secs_f64()),
                end: EndPosition::Custom(PlaybackPosition::Seconds(ends.as_secs_f64())),
            }),
        }
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
            return Ok(Some(PlaybackRate::Factor(
                s[1..].trim().parse().map_err(serde::de::Error::custom)?,
            )));
        }
        if s.ends_with('♯') {
            return Ok(Some(PlaybackRate::Semitones(
                s[..s.len() - 1]
                    .trim()
                    .parse()
                    .map_err(serde::de::Error::custom)?,
            )));
        }
        if s.ends_with('♭') {
            return Ok(Some(PlaybackRate::Semitones(
                -s[1..].trim().parse().map_err(serde::de::Error::custom)?,
            )));
        }
        return Err(serde::de::Error::custom(format!(
            "invalid factor or semitone: {s}"
        )));
    }
    Ok(None)
}

/// Interop type for [kira::tween::Tween].
#[derive(Debug, Deserialize, Clone)]
pub struct Interpolation {
    #[serde(with = "humantime_serde", default)]
    pub start_time: Option<Duration>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    #[serde(flatten)]
    pub easing: kira::tween::Easing,
}

impl From<Interpolation> for Tween {
    fn from(value: Interpolation) -> Self {
        Self {
            start_time: value
                .start_time
                .map(StartTime::Delayed)
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
                        .map(|x| ::kira::tween::Value::<Volume>::Fixed(Volume::Amplitude(x)))
                        .unwrap_or_default(),
                    panning: value
                        .panning
                        .map(f64::from)
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

impl_from_settings!(kira::sound::static_sound::StaticSoundSettings);
impl_from_settings!(kira::sound::streaming::StreamingSoundSettings);

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
