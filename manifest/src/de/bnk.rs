use std::path::PathBuf;

use serde::Deserialize;

/// File descriptor entry for `.bnk`.
///
/// Vanilla example:
///
/// ```json
/// {
///     "Name": "arch_nazare",
///     "GUID": "{1102AD21-7710-45F9-94B0-240B26C392FE}",
///     "IsResident": false,
///     "ResourcePath": "base\\sound\\soundbanks\\arch_nazare.bnk",
///     "PC size:": 0,
///     "Xone size:": 0,
///     "PS4 size:": 0
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct SoundBankInfo {
    pub is_resident: bool,
    pub path: PathBuf,
    pub metadata: Option<AudioEventArray>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioEventArray {
    pub bus: Option<Vec<AudioEventMetadataArrayElement>>,
    pub events: Option<Vec<AudioEventMetadataArrayElement>>,
    pub game_parameter: Option<Vec<AudioEventMetadataArrayElement>>,
    pub state: Option<Vec<AudioEventMetadataArrayElement>>,
    pub state_group: Option<Vec<AudioEventMetadataArrayElement>>,
    pub switch: Option<Vec<AudioEventMetadataArrayElement>>,
    pub switch_group: Option<Vec<AudioEventMetadataArrayElement>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioEventMetadataArrayElement {
    pub is_looping: Option<bool>,
    pub max_attenuation: Option<f32>,
    pub min_duration: Option<f32>,
    pub max_duration: Option<f32>,
    pub red_id: String,
    pub stop_action_events: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub wwise_id: u32,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::SoundBankInfo;
    use red4ext_rs::types::ResRef;
    use test_case::test_case;

    #[test_case(r##"id:
    path: my_mod\\sound\\soundbanks\\custom_bank.bnk
    is_resident: true"## ; "simple .bnk double slashed path")]
    #[test_case(r##"id:
    path: my_mod\sound\soundbanks\custom_bank.bnk
    is_resident: true"## ; "simple .bnk single slashed path")]
    #[test_case(r##"id:
    path: my_mod\sound\soundbanks\custom_bank.bnk
    is_resident: true
    metadata:
        events:
            - redId: mus_lizzies_bds_music_01_play
              wwiseId: 2519536634
            - redId: mus_lizzies_bds_music_01_stop
              wwiseId: 3960092164"## ; "complex .bnk with simple events metadata")]
    fn bnk(yaml: &str) {
        let bnk = serde_yaml::from_str::<HashMap<String, SoundBankInfo>>(yaml);
        dbg!("{}", &bnk);
        assert!(bnk.is_ok());
        let path = ResRef::new(bnk.unwrap().iter().next().unwrap().1.path.clone());
        assert!(path.is_ok());
    }
}
