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
    fn bnk(yaml: &str) {
        let bnk = serde_yaml::from_str::<HashMap<String, SoundBankInfo>>(yaml);
        dbg!("{}", &bnk);
        assert!(bnk.is_ok());
        let path = ResRef::new(bnk.unwrap().iter().next().unwrap().1.path.clone());
        assert!(path.is_ok());
    }
}
