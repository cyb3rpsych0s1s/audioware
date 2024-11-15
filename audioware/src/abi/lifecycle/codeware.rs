use audioware_manifest::PlayerGender;

#[derive(Debug)]
pub enum Codeware {
    SetPlayerGender { gender: PlayerGender },
    UnsetPlayerGender,
}

impl std::fmt::Display for Codeware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Codeware::SetPlayerGender { gender } => write!(f, "set player gender: {gender}"),
            Codeware::UnsetPlayerGender => write!(f, "unset player gender"),
        }
    }
}
