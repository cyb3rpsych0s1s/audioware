use std::path::PathBuf;

use audioware_core::Amplitude;

use crate::Interpolation;

#[derive(Debug, Deserialize, Clone)]
pub struct MainMenu {
    music: MainMenuMusic,
}

/// Main menu music is played continuously,
/// either as a sequential loop
/// or with a crossfade.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MainMenuMusic {
    Crossfade {
        /// Audio file path.
        file: PathBuf,
        /// Volume for both tracks.
        volume: Option<Amplitude>,
        /// How the next track fades-in.
        fade_in: Option<Fade>,
        /// How the current track fades-out.
        fade_out: Option<Fade>,
        /// Slice for both tracks.
        region: Option<super::setting::Region>,
    },
    Loop(LoopMainMenu),
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
            starts: 2"## ; "custom main menu music loop")]
    fn music(yaml: &str) {
        let main_menu = serde_yaml::from_str::<MainMenu>(yaml);
        dbg!("{}", &main_menu);
        assert!(main_menu.is_ok());
    }
}
