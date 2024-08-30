//! Voices definitions, either gender-based, locale-based or both.

use std::{collections::HashMap, path::PathBuf};

use audioware_core::With;
use either::Either;
use serde::Deserialize;

use crate::{Locale, PlayerGender, ScnDialogLineType};

use super::{paths_into_audios, Audio, DialogLine, GenderBased, Settings, Usage};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    SingleInline {
        #[serde(flatten)]
        dialogs: HashMap<Locale, PathBuf>,
        usage: Option<Usage>,
        settings: Option<Settings>,
    },
    SingleMulti {
        #[serde(flatten)]
        dialogs: HashMap<Locale, Dialog>,
        usage: Option<Usage>,
        line: Option<ScnDialogLineType>,
        settings: Option<Settings>,
    },
    DualInline {
        #[serde(flatten)]
        dialogs: HashMap<Locale, HashMap<PlayerGender, PathBuf>>,
        usage: Option<Usage>,
        settings: Option<Settings>,
    },
    DualMulti {
        #[serde(flatten)]
        dialogs: HashMap<Locale, Dialogs>,
        usage: Option<Usage>,
        line: Option<ScnDialogLineType>,
        settings: Option<Settings>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Dialog {
    #[serde(flatten)]
    pub basic: Audio,
    pub subtitle: String,
}

impl From<(&Dialog, Option<&Settings>)> for Audio {
    fn from(value: (&Dialog, Option<&Settings>)) -> Self {
        let mut audio: Audio = value.into();
        if let Some(settings) = value.1 {
            audio = audio.with(settings.clone());
        }
        audio
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dialogs {
    Different {
        #[serde(flatten)]
        dialogs: Box<GenderBased<Dialog>>,
    },
    Shared {
        #[serde(flatten)]
        paths: GenderBased<PathBuf>,
        subtitle: String,
    },
}

/// ultimately a voice is just either file path for each locale
/// with optional gender,
/// optional corresponding dialog lines,
/// audio usage
/// and optional settings
pub type AnyVoice = Either<
    (
        HashMap<Locale, Audio>,
        Usage,
        Option<HashMap<Locale, DialogLine>>,
    ),
    (
        HashMap<Locale, GenderBased<Audio>>,
        Usage,
        Option<HashMap<Locale, GenderBased<DialogLine>>>,
    ),
>;

impl From<Voice> for AnyVoice {
    fn from(value: Voice) -> Self {
        let default_usage = Usage::OnDemand;
        let default_line = ScnDialogLineType::Regular;
        match value {
            Voice::SingleInline {
                dialogs,
                usage,
                settings,
            } => {
                let dialogs = paths_into_audios(dialogs, settings);
                Either::Left((dialogs, usage.unwrap_or(default_usage), None))
            }
            Voice::SingleMulti {
                dialogs,
                usage,
                line,
                settings,
            } => {
                let mut aud: HashMap<Locale, Audio> = HashMap::with_capacity(dialogs.len());
                let mut sub: HashMap<Locale, DialogLine> = HashMap::with_capacity(dialogs.len());
                for (k, ref v) in dialogs.into_iter() {
                    let audio: Audio = (v, settings.as_ref()).into();
                    aud.insert(k, audio);
                    sub.insert(
                        k,
                        DialogLine {
                            msg: v.subtitle.clone(),
                            line: line.unwrap_or(default_line),
                        },
                    );
                }
                Either::Left((aud, usage.unwrap_or(default_usage), Some(sub)))
            }
            Voice::DualInline {
                dialogs,
                usage,
                settings,
            } => {
                let dialogs: HashMap<Locale, GenderBased<Audio>> = dialogs
                    .into_iter()
                    .map(|(k, v)| (k, GenderBased::<Audio>::from((v, settings.clone()))))
                    .collect();
                Either::Right((dialogs, usage.unwrap_or(default_usage), None))
            }
            Voice::DualMulti {
                dialogs,
                usage,
                line,
                settings,
            } => {
                let mut aud: HashMap<Locale, GenderBased<Audio>> =
                    HashMap::with_capacity(dialogs.len());
                let mut sub: HashMap<Locale, GenderBased<DialogLine>> =
                    HashMap::with_capacity(dialogs.len());
                for (k, v) in dialogs.into_iter() {
                    match v {
                        super::Dialogs::Different { dialogs } => {
                            let dialogs = *dialogs;
                            aud.insert(
                                k,
                                GenderBased {
                                    fem: settings.clone().map_or(dialogs.fem.basic.clone(), |x| {
                                        dialogs.fem.basic.with(x)
                                    }),
                                    male: settings
                                        .clone()
                                        .map_or(dialogs.male.basic.clone(), |x| {
                                            dialogs.male.basic.with(x)
                                        }),
                                },
                            );
                            sub.insert(
                                k,
                                GenderBased {
                                    fem: DialogLine {
                                        msg: dialogs.fem.subtitle.clone(),
                                        line: line.unwrap_or(ScnDialogLineType::Regular),
                                    },
                                    male: DialogLine {
                                        msg: dialogs.male.subtitle.clone(),
                                        line: line.unwrap_or(ScnDialogLineType::Regular),
                                    },
                                },
                            );
                        }
                        super::Dialogs::Shared { paths, subtitle } => {
                            aud.insert(k, paths.into());
                            let same = DialogLine {
                                msg: subtitle.clone(),
                                line: line.unwrap_or(default_line),
                            };
                            sub.insert(
                                k,
                                GenderBased {
                                    fem: same.clone(),
                                    male: same,
                                },
                            );
                        }
                    }
                }
                Either::Right((aud, usage.unwrap_or(default_usage), Some(sub)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod unique_dialog {
        use super::super::Voice;
        use std::collections::HashMap;
        use test_case::test_case;

        #[test_case(r##"id:
    en-us: ./somewhere/sfx.wav"## ; "implicit on-demand unique dialog no subtitle")]
        #[test_case(r##"id:
    en-us:
        file: ./somewhere/sfx.wav
        subtitle: "hello world""## ; "implicit on-demand unique dialog with subtitle")]
        #[test_case(r##"id:
    en-us:
        file: ./somewhere/sfx.wav
        subtitle: "hello world"
    line: radio"## ; "implicit on-demand unique dialog with subtitle and line type")]
        #[test_case(r##"id:
    en-us: ./somewhere/sfx.wav
    fr-fr: ./somewhere/sfx.wav"## ; "format must be consistent across locales when there's no subtitle")]
        #[test_case(r##"id:
    en-us:
        file: ./somewhere/sfx.wav
        subtitle: "hello world"
    fr-fr:
        file: ./somewhere/else/sfx.wav
        subtitle: "bonjour tout le monde"
    line: radio"## ; "format must be consistent across locales when there are subtitles")]
        fn basic_format(yaml: &str) {
            let unique_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &unique_dialog);
            assert!(unique_dialog.is_ok());
        }

        #[test_case(r##"id:
    en-us:
        file: ./somewhere/sfx.wav
        subtitle: "hello world"
    fr-fr: ./somewhere/else/sfx.wav"## ; "format must be consistent")]
        fn incompatibility(yaml: &str) {
            let unique_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &unique_dialog);
            assert!(unique_dialog.is_err());
        }
    }

    mod dual_dialog {
        use std::collections::HashMap;

        use super::super::Voice;
        use test_case::test_case;

        #[test_case(r##"id:
    en-us:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
    fr-fr:
        fem: ./elsewhere/sfx.wav
        male: ./elsewhere/else/sfx.wav"## ; "dual dialog without subtitle")]
        #[test_case(r##"id:
    en-us:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
    fr-fr:
        fem: ./elsewhere/sfx.wav
        male: ./elsewhere/else/sfx.wav
    settings:
        region:
            starts: 500ms"## ; "dual dialog without subtitle with specific settings")]
        fn basic_format_without_subtitle(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_ok());
        }

        #[test_case(r##"id:
    en-us:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
        subtitle: "hello world"
    line: radio"## ; "dual dialog with shared subtitle")]
        #[test_case(r##"id:
    en-us:
        fem:
            file: ./somewhere/sfx.wav
            subtitle: "hello world"
        male:
            file: ./somewhere/else/sfx.wav
            subtitle: "hello world""## ; "dual dialog with different subtitles and default line")]
        #[test_case(r##"id:
    en-us:
        fem:
            file: ./somewhere/sfx.wav
            subtitle: "hello world"
            settings:
                region:
                    starts: 500ms
        male:
            file: ./somewhere/else/sfx.wav
            subtitle: "hello world"
            settings:
                region:
                    starts: 300ms"## ; "dual dialog with different subtitles, default line and custom settings")]
        fn basic_format_with_subtitles(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_ok());
        }

        #[test_case(r##"id:
    en-us:
        fem: ./somewhere/sfx.wav
        subtitle: "hello world""## ; "format must be consistent, partially defined gender is not allowed")]
        #[test_case(r##"id:
    en-us:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
        subtitle: "hello world"
    fr-fr:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
    line: radio"## ; "format must be consistent, mixing sub/no-sub is not allowed")]
        #[test_case(r##"id:
    en-us:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
    fr-fr:
        fem: ./somewhere/sfx.wav
        male: ./somewhere/else/sfx.wav
    line: radio"## ; "format must be consistent, if there's no subtitle there shouldn't be any line")]
        fn incompatibility(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_err());
        }
    }
}
