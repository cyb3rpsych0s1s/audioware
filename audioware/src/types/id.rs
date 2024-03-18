use red4ext_rs::types::{CName, EntityId};
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundEntityId(pub EntityId);

impl std::hash::Hash for SoundEntityId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct VoiceId(CName);

impl std::hash::Hash for VoiceId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
    }
}

impl AsRef<CName> for VoiceId {
    fn as_ref(&self) -> &CName {
        &self.0
    }
}

struct SoundIdVisitor;

impl<'de> Visitor<'de> for SoundIdVisitor {
    type Value = VoiceId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a unique CName pool valid string representation")
    }

    #[cfg(not(test))] // in-game, sound ids have to be unique
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if CName::new(v).to_string().as_str() == v {
            return Err(E::custom(format!(
                "string already exists in CName pool: {}",
                v
            )));
        }
        let created = CName::new_pooled(v);
        Ok(VoiceId(created))
    }

    #[cfg(test)] // allow to test deserialization
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VoiceId(CName::new(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for VoiceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SoundIdVisitor)
    }
}

impl std::fmt::Display for VoiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl PartialEq<CName> for VoiceId {
    fn eq(&self, other: &CName) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<VoiceId> for CName {
    fn eq(&self, other: &VoiceId) -> bool {
        self.eq(&other.0)
    }
}

impl From<CName> for VoiceId {
    fn from(value: CName) -> Self {
        Self(value)
    }
}
