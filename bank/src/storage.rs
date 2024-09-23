//! Bank storage for data and settings.

use std::{
    collections::{
        hash_map::{Iter, Keys, Values},
        HashMap,
    },
    hash::Hash,
    sync::OnceLock,
};

use either::Either;
use kira::sound::{static_sound::StaticSoundData, streaming::StreamingSoundData, FromFileError};

use audioware_manifest::{
    AudioEventMetadataArrayElement, DialogLine, Locale, PlayerGender, Settings as ManifestSettings,
    WrittenLocale,
};
use red4ext_rs::types::CName;

use crate::{Banks, BothKey, GenderKey, Id, Key, LocaleKey, SoundBank, UniqueKey, Usage};

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
    pub fn values(&self) -> Values<'_, K, V> {
        self.0.get().expect("should be initialized").values()
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.0.get().expect("should be initialized").iter()
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

pub(super) static UNIQUES: OnceStorage<UniqueKey, StaticSoundData> = OnceStorage::new();
pub(super) static GENDERS: OnceStorage<GenderKey, StaticSoundData> = OnceStorage::new();
pub(super) static LOCALES: OnceStorage<LocaleKey, StaticSoundData> = OnceStorage::new();
pub(super) static MULTIS: OnceStorage<BothKey, StaticSoundData> = OnceStorage::new();

pub(super) static UNI_SET: OnceStorage<UniqueKey, ManifestSettings> = OnceStorage::new();
pub(super) static GEN_SET: OnceStorage<GenderKey, ManifestSettings> = OnceStorage::new();
pub(super) static LOC_SET: OnceStorage<LocaleKey, ManifestSettings> = OnceStorage::new();
pub(super) static MUL_SET: OnceStorage<BothKey, ManifestSettings> = OnceStorage::new();

pub(super) static LOC_SUB: OnceStorage<LocaleKey, DialogLine> = OnceStorage::new();
pub(super) static MUL_SUB: OnceStorage<BothKey, DialogLine> = OnceStorage::new();

pub static BNKS: OnceStorage<CName, SoundBank> = OnceStorage::new();

impl BankSettings for Banks {
    type Key = Id;
    type Settings = audioware_manifest::Settings;

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
                    Usage::Static(_, _) => settings,
                    Usage::Streaming(_, _) => settings,
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
            Id::InMemory(Key::Unique(key), ..) => Either::Left(UNIQUES.data(key)),
            Id::InMemory(Key::Gender(key), ..) => Either::Left(GENDERS.data(key)),
            Id::InMemory(Key::Locale(key), ..) => Either::Left(LOCALES.data(key)),
            Id::InMemory(Key::Both(key), ..) => Either::Left(MULTIS.data(key)),
        }
    }
}

impl BankSubtitles for Banks {
    type Key = Id;
    fn subtitles(&self, locale: WrittenLocale) -> Vec<(CName, (String, String))> {
        [LOC_SUB.subtitles(locale), MUL_SUB.subtitles(locale)].concat()
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

macro_rules! count_metadata {
    ($metadata:ident) => {
        BNKS.iter().fold(0, |acc, (_, v)| {
            acc + v
                .metadata
                .as_ref()
                .and_then(|x| x.$metadata.as_ref().map(|x| x.len()))
                .unwrap_or(0)
        })
    };
}

impl Banks {
    pub fn bus_count() -> usize {
        count_metadata!(bus)
    }
    pub fn events_count() -> usize {
        count_metadata!(events)
    }
    pub fn game_parameter_count() -> usize {
        count_metadata!(game_parameter)
    }
    pub fn state_count() -> usize {
        count_metadata!(state)
    }
    pub fn state_group_count() -> usize {
        count_metadata!(state_group)
    }
    pub fn switch_count() -> usize {
        count_metadata!(switch)
    }
    pub fn switch_group_count() -> usize {
        count_metadata!(switch_group)
    }
}

macro_rules! metadata_ref {
    ($metadata:ident, $count:ident) => {{
        let mut references = Vec::with_capacity(Self::$count());
        for (_, v) in BNKS.iter() {
            if let Some(x) = v.metadata.as_ref() {
                if let Some(x) = x.$metadata.as_ref() {
                    for elem in x.iter() {
                        references.push(elem);
                    }
                }
            }
        }
        references
    }};
}

#[rustfmt::skip]
impl Banks {
    pub fn bus_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(bus, bus_count) }
    pub fn events_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(events, events_count) }
    pub fn game_parameter_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(game_parameter, game_parameter_count) }
    pub fn state_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(state, state_count) }
    pub fn state_group_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(state_group, state_group_count) }
    pub fn switch_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(switch, switch_count) }
    pub fn switch_group_metadata<'a>() -> Vec<&'a AudioEventMetadataArrayElement> { metadata_ref!(switch_group, switch_group_count) }
}
