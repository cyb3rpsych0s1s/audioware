//! Bank storage for data and settings.

use std::{
    collections::{HashMap, hash_map::Keys},
    hash::Hash,
    sync::OnceLock,
};

use either::Either;
use kira::sound::{FromFileError, static_sound::StaticSoundData, streaming::StreamingSoundData};

use audioware_manifest::{
    DialogLine, Locale, PlayerGender, Settings as ManifestSettings, WrittenLocale,
};
use red4ext_rs::types::CName;

use crate::{Banks, BothKey, Id, Key, LocaleKey, SceneId, SceneKey, Usage};

pub trait BankData<K, V> {
    fn data(&self, key: &K) -> V;
}

pub trait BankSettings<K, V> {
    fn settings(&self, key: &K) -> Option<V>;
}

pub trait BankSubtitles {
    type Key;

    /// All subtitles stored in banks for a given [written locale](WrittenLocale),
    /// returned as raw values.
    fn subtitles(&self, locale: WrittenLocale) -> Vec<(CName, (String, String))>;
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
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.0.get().expect("should be initialized").keys()
    }
    pub fn safe_keys(&self) -> Option<Keys<'_, K, V>> {
        self.0.get().map(|x| x.keys())
    }
}
impl BankData<Key, StaticSoundData> for OnceStorage<Key, StaticSoundData> {
    fn data(&self, key: &Key) -> StaticSoundData {
        self.0
            .get()
            .expect("insertion guarantees")
            .get(key)
            .expect("insertion guarantees")
            .clone()
    }
}
impl BankData<SceneKey, StaticSoundData> for OnceStorage<SceneKey, StaticSoundData> {
    fn data(&self, key: &SceneKey) -> StaticSoundData {
        self.0
            .get()
            .expect("insertion guarantees")
            .get(key)
            .expect("insertion guarantees")
            .clone()
    }
}
impl<K: Eq + Hash> BankSettings<K, ManifestSettings> for OnceStorage<K, ManifestSettings> {
    fn settings(&self, key: &K) -> Option<ManifestSettings> {
        self.0.get().and_then(|x| x.get(key)).cloned()
    }
}
impl BankSubtitles for OnceStorage<LocaleKey, DialogLine> {
    type Key = LocaleKey;

    fn subtitles(&self, locale: WrittenLocale) -> Vec<(CName, (String, String))> {
        self.0
            .get()
            .map(|x| x.package(locale.into_inner()))
            .unwrap_or_default()
    }
}
impl BankSubtitles for OnceStorage<BothKey, DialogLine> {
    type Key = BothKey;

    fn subtitles(&self, locale: WrittenLocale) -> Vec<(CName, (String, String))> {
        self.0
            .get()
            .map(|x| x.package(locale.into_inner()))
            .unwrap_or_default()
    }
}

// pub(super) static UNIQUES: OnceStorage<UniqueKey, StaticSoundData> = OnceStorage::new();
// pub(super) static GENDERS: OnceStorage<GenderKey, StaticSoundData> = OnceStorage::new();
// pub(super) static LOCALES: OnceStorage<LocaleKey, StaticSoundData> = OnceStorage::new();
// pub(super) static MULTIS: OnceStorage<BothKey, StaticSoundData> = OnceStorage::new();

// pub(super) static UNI_SET: OnceStorage<UniqueKey, ManifestSettings> = OnceStorage::new();
// pub(super) static GEN_SET: OnceStorage<GenderKey, ManifestSettings> = OnceStorage::new();
// pub(super) static LOC_SET: OnceStorage<LocaleKey, ManifestSettings> = OnceStorage::new();
// pub(super) static MUL_SET: OnceStorage<BothKey, ManifestSettings> = OnceStorage::new();

// pub(super) static LOC_SUB: OnceStorage<LocaleKey, DialogLine> = OnceStorage::new();
// pub(super) static MUL_SUB: OnceStorage<BothKey, DialogLine> = OnceStorage::new();

impl BankSettings<Id, audioware_manifest::Settings> for Banks {
    /// Retrieves sound settings for a given [Id] if any.
    fn settings(&self, key: &Id) -> Option<audioware_manifest::Settings> {
        match key {
            Id::OnDemand(usage, ..) => {
                let settings = match usage {
                    Usage::Static(key, _) | Usage::Streaming(key, _) => match key {
                        Key::Unique(key) => self.unique_settings.get(key),
                        Key::Gender(key) => self.gender_settings.get(key),
                        Key::Locale(key) => self.single_settings.get(key),
                        Key::Both(key) => self.dual_settings.get(key),
                    },
                };
                settings.cloned()
            }
            // in-memory sound data already embed settings
            Id::InMemory(_, _) => None,
        }
    }
}

impl BankSettings<SceneId, audioware_manifest::Settings> for Banks {
    /// Retrieves sound settings for a given [SceneId] if any.
    fn settings(&self, key: &SceneId) -> Option<audioware_manifest::Settings> {
        match key {
            SceneId::OnDemand(usage, ..) => {
                let settings = match usage {
                    Usage::Static(key, _) | Usage::Streaming(key, _) => match key {
                        SceneKey::Locale(key) => self.single_scene_dialogs_settings.get(key),
                        SceneKey::Both(key) => self.dual_scene_dialogs_settings.get(key),
                    },
                };
                settings.cloned()
            }
            // in-memory sound data already embed settings
            SceneId::InMemory(_) => None,
        }
    }
}

impl BankData<Id, Either<StaticSoundData, StreamingSoundData<FromFileError>>> for Banks {
    /// Retrieves sound data for a given [Id], including settings if any.
    fn data(&self, key: &Id) -> Either<StaticSoundData, StreamingSoundData<FromFileError>> {
        match key {
            Id::OnDemand(Usage::Static(_, path), ..) => {
                let settings = self.settings(key);
                let data = StaticSoundData::from_file(path)
                    .expect("static sound data has already been validated");
                if let Some(settings) = settings {
                    return Either::Left(data.with_settings(settings.into()));
                }
                Either::Left(data)
            }
            Id::OnDemand(Usage::Streaming(_, path), ..) => {
                let settings = self.settings(key);
                let data = StreamingSoundData::from_file(path)
                    .expect("streaming sound data has already been validated");
                if let Some(settings) = settings {
                    return Either::Right(data.with_settings(settings.into()));
                }
                Either::Right(data)
            }
            // in-memory sound data already embed settings
            Id::InMemory(Key::Unique(key), ..) => {
                Either::Left(self.uniques.get(key).cloned().expect("key guarantees"))
            }
            Id::InMemory(Key::Gender(key), ..) => {
                Either::Left(self.genders.get(key).cloned().expect("key guarantees"))
            }
            Id::InMemory(Key::Locale(key), ..) => Either::Left(
                self.single_voices
                    .get(key)
                    .cloned()
                    .expect("key guarantees"),
            ),
            Id::InMemory(Key::Both(key), ..) => {
                Either::Left(self.dual_voices.get(key).cloned().expect("key guarantees"))
            }
        }
    }
}

impl BankData<SceneId, Either<StaticSoundData, StreamingSoundData<FromFileError>>> for Banks {
    /// Retrieves sound data for a given [SceneId], including settings if any.
    fn data(&self, key: &SceneId) -> Either<StaticSoundData, StreamingSoundData<FromFileError>> {
        match key {
            SceneId::OnDemand(Usage::Static(_, path), ..) => {
                let settings = self.settings(key);
                let data = StaticSoundData::from_file(path)
                    .expect("static sound data has already been validated");
                if let Some(settings) = settings {
                    return Either::Left(data.with_settings(settings.into()));
                }
                Either::Left(data)
            }
            SceneId::OnDemand(Usage::Streaming(_, path), ..) => {
                let settings = self.settings(key);
                let data = StreamingSoundData::from_file(path)
                    .expect("streaming sound data has already been validated");
                if let Some(settings) = settings {
                    return Either::Right(data.with_settings(settings.into()));
                }
                Either::Right(data)
            }
            // in-memory sound data already embed settings
            SceneId::InMemory(SceneKey::Locale(key), ..) => Either::Left(
                self.single_scene_dialogs
                    .get(key)
                    .cloned()
                    .expect("key guarantees"),
            ),
            SceneId::InMemory(SceneKey::Both(key), ..) => Either::Left(
                self.dual_scene_dialogs
                    .get(key)
                    .cloned()
                    .expect("key guarantees"),
            ),
        }
    }
}

impl BankSubtitles for Banks {
    type Key = Id;
    fn subtitles(&self, locale: WrittenLocale) -> Vec<(CName, (String, String))> {
        [
            self.single_subs
                .iter()
                .filter(|x| x.0.1 == locale)
                .map(|x| (x.0.0, (x.1.msg.clone(), x.1.msg.clone())))
                .collect::<Vec<_>>(),
            self.dual_subs
                .iter()
                .filter(|x| x.0.1 == locale)
                .map(|x| (x.0.0, (x.1.msg.clone(), x.1.msg.clone())))
                .collect::<Vec<_>>(),
        ]
        .concat()
    }
}

/// Returns raw entries to create a [localization package](https://github.com/psiberx/cp2077-codeware/wiki#localization-packages).
pub trait Package {
    fn package(&self, locale: Locale) -> Vec<(CName, (String, String))>;
}

impl Package for HashMap<LocaleKey, DialogLine> {
    fn package(&self, locale: Locale) -> Vec<(CName, (String, String))> {
        let mut out = Vec::new();
        for (k, v) in self {
            if k.1 == locale {
                out.push((k.0, (v.msg.clone(), v.msg.clone())));
            }
        }
        out
    }
}

impl Package for HashMap<BothKey, DialogLine> {
    fn package(&self, locale: Locale) -> Vec<(CName, (String, String))> {
        let mut out = Vec::new();
        let mut female: String;
        let mut male: String;
        for (k, v) in self {
            if k.1 == locale {
                if k.2 == PlayerGender::Female {
                    female = v.msg.clone();
                    male = self
                        .get(&BothKey(k.0, k.1, PlayerGender::Male))
                        .expect("genders cannot be partially defined")
                        .msg
                        .clone();
                } else {
                    male = v.msg.clone();
                    female = self
                        .get(&BothKey(k.0, k.1, PlayerGender::Female))
                        .expect("genders cannot be partially defined")
                        .msg
                        .clone();
                }
                out.push((k.0, (female, male)));
            }
        }
        out
    }
}
