use std::time::Duration;

use kira::{tween::Tween, StartTime, Volume};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(with = "humantime_serde", default)]
    pub start_time: Option<Duration>,
    #[serde(with = "humantime_serde", default)]
    pub start_position: Option<Duration>,
    pub volume: Option<f64>,
    pub panning: Option<f64>,
    pub tween: Option<Interpolation>,
}

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
                        .unwrap_or(::kira::StartTime::Immediate),
                    start_position: value
                        .start_position
                        .map(|x| ::kira::sound::PlaybackPosition::Seconds(x.as_secs_f64()))
                        .unwrap_or_default(),
                    volume: value
                        .volume
                        .map(|x| ::kira::tween::Value::<Volume>::Fixed(Volume::Amplitude(x)))
                        .unwrap_or(::kira::tween::Value::<Volume>::Fixed(Volume::Amplitude(1.))),
                    panning: value
                        .panning
                        .map(f64::from)
                        .map(Into::into)
                        .unwrap_or(::kira::tween::Value::Fixed(0.5)),
                    fade_in_tween: value.tween.map(Into::into),
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
    start_time: 120ms
    volume: 0.5"## ; "start time + volume")]
    #[test_case(r##"settings:
    tween:
        duration: 1s
        InPowf: 0.5"## ; "tween")]
    #[test_case(r##"settings:
    start_time: 5s
    tween:
        duration: 9s
        InPowi: 2"## ; "complex settings")]
    fn settings(yaml: &str) {
        let settings = serde_yaml::from_str::<HashMap<String, Settings>>(yaml);
        dbg!("{}", &settings);
        assert!(settings.is_ok());
    }
}
