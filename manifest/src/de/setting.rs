use std::time::Duration;

use kira::{
    sound::{
        static_sound::StaticSoundSettings, streaming::StreamingSoundSettings, PlaybackPosition,
    },
    tween::{Tween, Value},
    StartTime, Volume,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(with = "humantime_serde", default)]
    pub start_time: Option<Duration>,
    #[serde(with = "humantime_serde", default)]
    pub start_position: Option<Duration>,
    pub volume: Option<f64>,
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

impl From<Settings> for StaticSoundSettings {
    fn from(value: Settings) -> Self {
        Self {
            start_time: value
                .start_time
                .map(StartTime::Delayed)
                .unwrap_or(StartTime::Immediate),
            start_position: value
                .start_position
                .map(|x| PlaybackPosition::Seconds(x.as_secs_f64()))
                .unwrap_or_default(),
            volume: value
                .volume
                .map(|x| Value::<Volume>::Fixed(Volume::Amplitude(x)))
                .unwrap_or(Value::<Volume>::Fixed(Volume::Amplitude(1.))),
            fade_in_tween: value.tween.map(Into::into),
            ..Default::default()
        }
    }
}

impl From<Settings> for StreamingSoundSettings {
    fn from(value: Settings) -> Self {
        Self {
            start_time: value
                .start_time
                .map(StartTime::Delayed)
                .unwrap_or(StartTime::Immediate),
            start_position: value
                .start_position
                .map(|x| PlaybackPosition::Seconds(x.as_secs_f64()))
                .unwrap_or_default(),
            volume: value
                .volume
                .map(|x| Value::<Volume>::Fixed(Volume::Amplitude(x)))
                .unwrap_or(Value::<Volume>::Fixed(Volume::Amplitude(1.))),
            ..Default::default()
        }
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

    #[test_case(r##"id:
    start_time: 120ms"## ; "start time")]
    #[test_case(r##"id:
    start_time: 120ms
    volume: 0.5"## ; "start time + volume")]
    #[test_case(r##"id:
    tween:
        duration: 1s
        InPowf: 0.5"## ; "tween")]
    fn settings(yaml: &str) {
        let settings = serde_yaml::from_str::<HashMap<String, Settings>>(yaml);
        dbg!("{}", &settings);
        assert!(settings.is_ok());
    }
}
