use red4ext_rs::types::CName;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SoundId(String);

impl std::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl SoundId {
    pub fn cname(&self) -> CName {
        CName::new(&self.0)
    }
}

impl PartialEq<CName> for SoundId {
    fn eq(&self, other: &CName) -> bool {
        self.cname().eq(other)
    }
}

impl PartialEq<SoundId> for CName {
    fn eq(&self, other: &SoundId) -> bool {
        self.eq(&other.cname())
    }
}
