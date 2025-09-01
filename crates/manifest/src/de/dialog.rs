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

impl From<SceneDialog> for Audio {
    fn from(value: SceneDialog) -> Self {
        match value {
            SceneDialog::SingleInline {
                dialogs,
                usage,
                settings,
            } => todo!(),
            SceneDialog::DualInline {
                dialogs,
                usage,
                settings,
            } => todo!(),
        }
    }
}
