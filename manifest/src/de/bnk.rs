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
    pub name: String,
    pub is_resident: bool,
    pub path: PathBuf,
}
