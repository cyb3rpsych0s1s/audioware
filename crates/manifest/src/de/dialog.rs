use std::{collections::HashMap, path::PathBuf};

use either::Either;
use serde::Deserialize;

use crate::{Audio, Locale, paths_into_audios};

use super::{GenderBased, Settings, Usage};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
// variants declaration order matters: see https://github.com/cyb3rpsych0s1s/audioware/pull/39
pub enum SceneDialog {
    SingleInline {
        #[serde(flatten)]
        dialogs: HashMap<Locale, PathBuf>,
        usage: Option<Usage>,
        settings: Option<Settings>,
    },
    DualInline {
        #[serde(flatten)]
        dialogs: HashMap<Locale, GenderBased<PathBuf>>,
        usage: Option<Usage>,
        settings: Option<Settings>,
    },
}

pub type AnySceneDialog =
    Either<(HashMap<Locale, Audio>, Usage), (HashMap<Locale, GenderBased<Audio>>, Usage)>;

impl From<SceneDialog> for AnySceneDialog {
    fn from(value: SceneDialog) -> Self {
        let default_usage = Usage::OnDemand;
        match value {
            SceneDialog::SingleInline {
                dialogs,
                usage,
                settings,
            } => {
                let dialogs = paths_into_audios(dialogs, settings);
                Either::Left((dialogs, usage.unwrap_or(default_usage)))
            }
            SceneDialog::DualInline {
                dialogs,
                usage,
                settings,
            } => {
                let dialogs: HashMap<Locale, GenderBased<Audio>> = dialogs
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            GenderBased::<Audio> {
                                female: Audio {
                                    file: v.female,
                                    settings: settings.clone(),
                                },
                                male: Audio {
                                    file: v.male,
                                    settings: settings.clone(),
                                },
                            },
                        )
                    })
                    .collect();
                Either::Right((dialogs, usage.unwrap_or(default_usage)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod inline_dialog {
        use super::super::SceneDialog;
        use std::collections::HashMap;
        use test_case::test_case;

        #[test_case(r##"1300277703738512077:
    en-us: ./somewhere/dialog.wav"## ; "implicit on-demand inline scene dialog")]
        #[test_case(r##"1300277703738512077:
    en-us: ./somewhere/dialog.wav
    fr-fr: ./somewhere/else/dialog.wav"## ; "format must be consistent across locales")]
        fn basic_format(yaml: &str) {
            let unique_dialog = serde_yaml::from_str::<HashMap<i64, SceneDialog>>(yaml);
            dbg!("{}", &unique_dialog);
            assert!(unique_dialog.is_ok());
        }

        #[test_case(r##"1300277703738512077:
    en-us:
        file: ./somewhere/dialog.wav
    fr-fr: ./somewhere/else/dialog.wav"## ; "format must be consistent")]
        fn incompatibility(yaml: &str) {
            let unique_dialog = serde_yaml::from_str::<HashMap<i64, SceneDialog>>(yaml);
            dbg!("{}", &unique_dialog);
            assert!(unique_dialog.is_err());
        }
    }
    mod dual_dialog {
        use std::collections::HashMap;

        use super::super::SceneDialog;
        use test_case::test_case;

        #[test_case(r##"1300277703738512077:
    en-us:
        fem: ./somewhere/dialog.wav
        male: ./somewhere/else/dialog.wav
    fr-fr:
        fem: ./elsewhere/dialog.wav
        male: ./elsewhere/else/dialog.wav"## ; "dual scene dialog")]
        #[test_case(r##"1300277703738512077:
    en-us:
        fem: ./somewhere/dialog.wav
        male: ./somewhere/else/dialog.wav
    fr-fr:
        fem: ./elsewhere/dialog.wav
        male: ./elsewhere/else/dialog.wav
    settings:
        region:
            starts: 500ms"## ; "dual scene dialog with specific settings")]
        fn basic_format_with_settings(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<i64, SceneDialog>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_ok());
        }

        #[test_case(r##"1300277703738512077:
    en-us:
        fem: ./somewhere/dialog.wav"## ; "format must be consistent, partially defined gender is not allowed")]
        #[test_case(r##"1300277703738512077:
    en-us:
        fem: ./fem/dialog.wav
        male: ./male/dialog.wav
    fr-fr: ./unique/dialog.wav"## ; "format must be consistent, mixing unique and gender-based is not allowed")]
        fn incompatibility(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<i64, SceneDialog>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_err());
        }
    }
}
