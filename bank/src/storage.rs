//! Bank storage for data and settings.

use std::{collections::HashMap, hash::Hash, sync::OnceLock};

use either::Either;
use kira::sound::{
    static_sound::{StaticSoundData, StaticSoundSettings},
    streaming::{StreamingSoundData, StreamingSoundSettings},
    FromFileError,
};

use audioware_manifest::Settings as ManifestSettings;

use crate::{Banks, BothKey, GenderKey, Id, Key, LocaleKey, UniqueKey, Usage};

pub trait BankData {
    type Key;
    type Data;
    fn data(&self, key: &Self::Key) -> Self::Data;
}

pub trait BankSettings {
    type Key;
    type Settings;
    fn settings(&self, key: &Self::Key) -> Option<Self::Settings>;
}

pub struct OnceStorage<K, V>(OnceLock<HashMap<K, V>>);
impl<K, V> Default for OnceStorage<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> OnceStorage<K, V> {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }
    pub fn set(&self, value: HashMap<K, V>) -> Result<(), HashMap<K, V>> {
        self.0.set(value)
    }
}
impl<K: Eq + Hash> BankData for OnceStorage<K, StaticSoundData> {
    type Key = K;
    type Data = StaticSoundData;

    fn data(&self, key: &Self::Key) -> Self::Data {
        self.0
            .get()
            .expect("insertion guarantees")
            .get(key)
            .expect("insertion guarantees")
            .clone()
    }
}
impl<K: Eq + Hash> BankSettings for OnceStorage<K, ManifestSettings> {
    type Key = K;
    type Settings = ManifestSettings;

    fn settings(&self, key: &Self::Key) -> Option<Self::Settings> {
        self.0.get().and_then(|x| x.get(key)).cloned()
    }
}

pub(super) static UNIQUES: OnceStorage<UniqueKey, StaticSoundData> = OnceStorage::new();
pub(super) static GENDERS: OnceStorage<GenderKey, StaticSoundData> = OnceStorage::new();
pub(super) static LOCALES: OnceStorage<LocaleKey, StaticSoundData> = OnceStorage::new();
pub(super) static MULTIS: OnceStorage<BothKey, StaticSoundData> = OnceStorage::new();

pub(super) static UNI_SET: OnceStorage<UniqueKey, ManifestSettings> = OnceStorage::new();
pub(super) static GEN_SET: OnceStorage<GenderKey, ManifestSettings> = OnceStorage::new();
pub(super) static LOC_SET: OnceStorage<LocaleKey, ManifestSettings> = OnceStorage::new();
pub(super) static MUL_SET: OnceStorage<BothKey, ManifestSettings> = OnceStorage::new();

impl BankSettings for Banks {
    type Key = Id;
    type Settings = Either<StaticSoundSettings, StreamingSoundSettings>;

    /// Retrieves sound settings for a given [Id] if any.
    fn settings(&self, key: &Self::Key) -> Option<Self::Settings> {
        match key {
            Id::OnDemand(usage, ..) => {
                let settings = match usage {
                    Usage::Static(key, _) | Usage::Streaming(key, _) => match key {
                        Key::Unique(key) => UNI_SET.settings(key),
                        Key::Gender(key) => GEN_SET.settings(key),
                        Key::Locale(key) => LOC_SET.settings(key),
                        Key::Both(key) => MUL_SET.settings(key),
                    },
                };
                match usage {
                    Usage::Static(_, _) => {
                        settings.map(StaticSoundSettings::from).map(Either::Left)
                    }
                    Usage::Streaming(_, _) => settings
                        .map(StreamingSoundSettings::from)
                        .map(Either::Right),
                }
            }
            // in-memory sound data already embed settings
            Id::InMemory(_, _) => None,
        }
    }
}

impl BankData for Banks {
    type Key = Id;
    type Data = Either<StaticSoundData, StreamingSoundData<FromFileError>>;

    /// Retrieves sound data for a given [Id], including settings if any.
    fn data(&self, key: &Self::Key) -> Self::Data {
        match key {
            Id::OnDemand(Usage::Static(_, path), ..) => {
                let settings = self.settings(key);
                let data = StaticSoundData::from_file(path)
                    .expect("static sound data has already been validated");
                if let Some(settings) = settings {
                    let settings = settings.left().expect("static sound settings should match");
                    return Either::Left(data.with_settings(settings));
                }
                Either::Left(data)
            }
            Id::OnDemand(Usage::Streaming(_, path), ..) => {
                let settings = self.settings(key);
                let data = StreamingSoundData::from_file(path)
                    .expect("streaming sound data has already been validated");
                if let Some(settings) = settings {
                    let settings = settings
                        .right()
                        .expect("streaming sound settings should match");
                    return Either::Right(data.with_settings(settings));
                }
                Either::Right(data)
            }
            // in-memory sound data already embed settings
            Id::InMemory(Key::Unique(key), ..) => Either::Left(UNIQUES.data(key)),
            Id::InMemory(Key::Gender(key), ..) => Either::Left(GENDERS.data(key)),
            Id::InMemory(Key::Locale(key), ..) => Either::Left(LOCALES.data(key)),
            Id::InMemory(Key::Both(key), ..) => Either::Left(MULTIS.data(key)),
        }
    }
}
