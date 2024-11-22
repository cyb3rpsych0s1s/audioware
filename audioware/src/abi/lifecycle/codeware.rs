use audioware_manifest::PlayerGender;
use red4ext_rs::types::CName;

#[derive(Debug)]
pub enum Codeware {
    SetPlayerGender { gender: PlayerGender },
    UnsetPlayerGender,
    SetGameLocales { spoken: CName, written: CName },
}

impl std::fmt::Display for Codeware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Codeware::SetPlayerGender { gender } => write!(f, "set player gender: {gender}"),
            Codeware::UnsetPlayerGender => write!(f, "unset player gender"),
            Codeware::SetGameLocales { spoken, written } => {
                write!(f, "set game locales: spoken {spoken}, written {written}")
            }
        }
    }
}
