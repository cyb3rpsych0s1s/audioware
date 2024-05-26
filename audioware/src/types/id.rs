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

/// special kind of id guaranteed to be unique and to exist in banks
#[derive(Debug, PartialEq, Eq)]
pub enum Id {
    /// voice related id
    Voice(VoiceId),
    /// sfx related id
    Sfx(SfxId),
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::Voice(x) => write!(f, "{} |voice id|", x),
            Id::Sfx(x) => write!(f, "{} |sfx id|", x),
        }
    }
}

/// only AnyId should be constructed from CName
impl From<CName> for AnyId {
    fn from(value: CName) -> Self {
        AnyId(value)
    }
}

impl std::fmt::Display for AnyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (untyped)", &self.0)
    }
}

impl std::hash::Hash for Id {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Self::Voice(VoiceId(id)) = self {
            u64::from(id.clone()).hash(state);
        }
    }
}

// used inside macro

impl PartialEq<VoiceId> for AnyId {
    fn eq(&self, other: &VoiceId) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialEq<SfxId> for AnyId {
    fn eq(&self, other: &SfxId) -> bool {
        self.0.eq(&other.0)
    }
}

macro_rules! id {
    ($target:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $target(CName);

        impl std::hash::Hash for $target {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                u64::from(self.0.clone()).hash(state);
            }
        }

        impl AsRef<CName> for $target {
            fn as_ref(&self) -> &CName {
                &self.0
            }
        }

        impl PartialEq<CName> for $target {
            fn eq(&self, other: &CName) -> bool {
                self.0.eq(other)
            }
        }

        impl PartialEq<$target> for CName {
            fn eq(&self, other: &$target) -> bool {
                self.eq(&other.0)
            }
        }
    };
    ($target:ident, display) => {
        impl std::fmt::Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    };
    ($target:ident, $variant:ident, equality) => {
        impl PartialEq<$target> for Id {
            fn eq(&self, other: &$target) -> bool {
                match self {
                    Id::$variant(id) => id == other,
                    _ => false,
                }
            }
        }

        impl PartialEq<Id> for $target {
            fn eq(&self, other: &Id) -> bool {
                match other {
                    Id::$variant(id) => id == self,
                    _ => false,
                }
            }
        }
    };
    ($target:ident, $visitor:ident, $variant:ident) => {
        id!($target);
        id!($target, display);
        id!($target, $variant, equality);

        struct $visitor;

        impl<'de> Visitor<'de> for $visitor {
            type Value = $target;

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
                Ok($target(created))
            }

            #[cfg(test)] // allow to test deserialization
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok($target(CName::new(v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        impl<'de> Deserialize<'de> for $target {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor)
            }
        }

        impl From<&$target> for Id {
            fn from(value: &$target) -> Self {
                Self::$variant(value.clone())
            }
        }
    };
}

id!(VoiceId, VoiceIdVisitor, Voice);
id!(SfxId, SfxIdVisitor, Sfx);
id!(AnyId);
