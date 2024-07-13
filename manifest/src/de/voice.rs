use std::{collections::HashMap, path::PathBuf};

use either::Either;
use red4ext_rs_bindings::ScnDialogLineType;
use serde::Deserialize;

use crate::{deserialize_optional_scn_dialog_line_type, Locale, PlayerGender};

use super::{paths_into_audios, Audio, DialogLine, Settings, Usage};

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
        #[serde(
            default,
            deserialize_with = "deserialize_optional_scn_dialog_line_type"
        )]
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
        #[serde(
            default,
            deserialize_with = "deserialize_optional_scn_dialog_line_type"
        )]
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

impl From<&Dialog> for Audio {
    fn from(value: &Dialog) -> Self {
        value.basic.clone()
    }
}

impl From<(&Dialog, Option<&Settings>)> for Audio {
    fn from(value: (&Dialog, Option<&Settings>)) -> Self {
        let mut audio: Audio = value.into();
        if let Some(settings) = value.1 {
            audio.merge_settings(settings.clone());
        }
        audio
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dialogs {
    Different {
        #[serde(flatten)]
        dialogs: HashMap<PlayerGender, Dialog>,
    },
    Shared {
        #[serde(flatten)]
        paths: HashMap<PlayerGender, PathBuf>,
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
        HashMap<Locale, HashMap<PlayerGender, Audio>>,
        Usage,
        Option<HashMap<Locale, HashMap<PlayerGender, DialogLine>>>,
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
                let dialogs: HashMap<Locale, HashMap<PlayerGender, Audio>> = dialogs
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            v.into_iter()
                                .map(|(k, v)| (k, (v, settings.as_ref()).into()))
                                .collect(),
                        )
                    })
                    .collect();
                Either::Right((dialogs, usage.unwrap_or(default_usage), None))
            }
            Voice::DualMulti {
                dialogs,
                usage,
                line,
                settings,
            } => {
                let mut aud: HashMap<Locale, HashMap<PlayerGender, Audio>> =
                    HashMap::with_capacity(dialogs.len());
                let mut sub: HashMap<Locale, HashMap<PlayerGender, DialogLine>> =
                    HashMap::with_capacity(dialogs.len());
                for (k, v) in dialogs.into_iter() {
                    match v {
                        super::Dialogs::Different { dialogs } => {
                            let aud_dialogs = dialogs
                                .iter()
                                .map(|(k, v)| {
                                    let mut basic = v.basic.clone();
                                    if let Some(settings) = settings.clone() {
                                        basic.merge_settings(settings);
                                    }
                                    (*k, basic)
                                })
                                .collect();
                            let aud_subs = dialogs
                                .iter()
                                .map(|(k, v)| {
                                    (
                                        *k,
                                        DialogLine {
                                            msg: v.subtitle.clone(),
                                            line: line.unwrap_or(ScnDialogLineType::Regular),
                                        },
                                    )
                                })
                                .collect();
                            aud.insert(k, aud_dialogs);
                            sub.insert(k, aud_subs);
                        }
                        super::Dialogs::Shared { paths, subtitle } => {
                            let (fem, male) = (
                                paths.get(&PlayerGender::Female).unwrap(),
                                paths.get(&PlayerGender::Male).unwrap(),
                            );
                            aud.insert(
                                k,
                                HashMap::from([
                                    (
                                        PlayerGender::Female,
                                        Audio {
                                            file: fem.clone(),
                                            settings: settings.clone(),
                                        },
                                    ),
                                    (
                                        PlayerGender::Male,
                                        Audio {
                                            file: male.clone(),
                                            settings: settings.clone(),
                                        },
                                    ),
                                ]),
                            );
                            let same = DialogLine {
                                msg: subtitle.clone(),
                                line: line.unwrap_or(default_line),
                            };
                            sub.insert(
                                k,
                                HashMap::from([
                                    (PlayerGender::Female, same.clone()),
                                    (PlayerGender::Male, same),
                                ]),
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
        fn basic_format_with_subtitles(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_ok());
        }

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
