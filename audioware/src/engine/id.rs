use red4ext_rs::types::CName;
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundId(CName);

impl std::hash::Hash for SoundId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
    }
}

struct SoundIdVisitor;

impl<'de> Visitor<'de> for SoundIdVisitor {
    type Value = SoundId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a unique CName pool valid string representation")
    }

    #[cfg(not(test))] // in-game, sound ids have to be unique
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if CName::new(v).is_valid() {
            return Err(E::custom(format!(
                "string already exists in CName pool: {}",
                v
            )));
        }
        Ok(SoundId(CName::new_pooled(v)))
    }

    #[cfg(test)] // allow to test deserialization
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SoundId(CName::new(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for SoundId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SoundIdVisitor)
    }
}

impl std::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl PartialEq<CName> for SoundId {
    fn eq(&self, other: &CName) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<SoundId> for CName {
    fn eq(&self, other: &SoundId) -> bool {
        self.eq(&other.0)
    }
}

impl From<CName> for SoundId {
    fn from(value: CName) -> Self {
        Self(value)
    }
}
