use std::path::{Path, PathBuf};

use audioware_core::{Amplitude, With};
use kira::{
    PlaybackRate, Tween,
    sound::{PlaybackPosition, streaming::StreamingSoundSettings},
};
use serde::Deserialize;
use std::time::Duration;

use super::setting::factor_or_semitones;
use crate::Interpolation;

#[derive(Debug, Deserialize, Clone)]
pub struct MainMenu {
    pub music: MainMenuMusic,
}

impl AsRef<Path> for MainMenu {
    fn as_ref(&self) -> &Path {
        self.music.as_ref()
    }
}

/// Main menu music is played continuously,
/// either as a sequential loop
/// or with a crossfade.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MainMenuMusic {
    SimpleLoop(PathBuf),
    CustomLoop {
        file: PathBuf,
        settings: LoopMainMenuSettings,
    },
    Crossfade {
        /// Audio file path.
        file: PathBuf,
        /// Volume for both tracks.
        volume: Option<Amplitude>,
        /// How the next track fades-in.
        fade_in: Fade,
        /// How the current track fades-out.
        fade_out: Fade,
        /// Slice for both tracks.
        region: Option<super::setting::Region>,
    },
}

impl AsRef<Path> for MainMenuMusic {
    fn as_ref(&self) -> &Path {
        match self {
            Self::SimpleLoop(file)
            | Self::CustomLoop { file, .. }
            | Self::Crossfade { file, .. } => file.as_path(),
        }
    }
}

/// Loop the main menu music sequentially.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum LoopMainMenu {
    Simple(PathBuf),
    Custom {
        file: PathBuf,
        settings: LoopMainMenuSettings,
    },
}

/// Deserialization type
/// for Main Menu music settings.
#[derive(Debug, Default, Deserialize, Clone)]
pub struct LoopMainMenuSettings {
    #[serde(with = "humantime_serde", default)]
    pub start_position: Option<Duration>,
    pub volume: Option<Amplitude>,
    pub region: Option<super::setting::Region>,
    #[serde(deserialize_with = "factor_or_semitones", default)]
    pub playback_rate: Option<PlaybackRate>,
    pub fade_in_tween: Option<Interpolation>,
}

impl With<LoopMainMenuSettings> for StreamingSoundSettings {
    fn with(mut self, settings: LoopMainMenuSettings) -> Self
    where
        Self: Sized,
    {
        if let Some(start_position) = settings.start_position {
            self = self.start_position(PlaybackPosition::Seconds(start_position.as_secs_f64()));
        }
        if let Some(volume) = settings.volume {
            self = self.volume(volume.as_decibels());
        }
        if let Some(region) = settings.region {
            self = self.loop_region(region);
        } else {
            self = self.loop_region(..); // we necessarily want to loop
        }
        if let Some(playback_rate) = settings.playback_rate {
            self = self.playback_rate(playback_rate);
        }
        if let Some(fade_in_tween) = settings.fade_in_tween {
            self = self.fade_in_tween(Some(fade_in_tween.into()));
        }
        self
    }
}

/// Deserialization subset type for [kira::Tween].
#[derive(Debug, Deserialize, Clone)]
pub struct Fade {
    /// How long the audio should fade.
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    /// How the audio should fade.
    #[serde(flatten)]
    pub easing: kira::Easing,
}

impl From<Fade> for Tween {
    fn from(value: Fade) -> Self {
        Self {
            start_time: kira::StartTime::Immediate,
            duration: value.duration,
            easing: value.easing,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::MainMenu;

    #[test_case(r##"music: ./somewhere/music.wav"## ; "simple main menu music loop")]
    #[test_case(r##"music:
    file: ./somewhere/music.wav
    settings:
        volume: 0.5
        region:
            starts: 2s 12ms"## ; "custom main menu music loop")]
    #[test_case(r##"music:
    file: ./somewhere/music.wav
    fade_in:
        duration: 5s
        Linear:
    fade_out:
        duration: 5s
        InPowf: 3"## ; "main menu music crossfade")]
    fn music(yaml: &str) {
        let main_menu = serde_yaml::from_str::<MainMenu>(yaml);
        dbg!(&main_menu);
        assert!(main_menu.is_ok());
    }
}
