use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
    time::{Duration, Instant},
};

use audioware_manifest::{
    CannotParseManifestSnafu, CannotReadManifestSnafu, Depot, DialogLine, Locale, Manifest,
    PlayerGender, R6Audioware, REDmod, Settings,
};
use either::Either;
use ensure::*;
use kira::sound::{
    static_sound::{StaticSoundData, StaticSoundSettings},
    streaming::{StreamingSoundData, StreamingSoundSettings},
    FromFileError,
};
use red4ext_rs::types::CName;
use snafu::ResultExt;

pub mod conflict;
mod ensure;
pub mod error;
pub use error::Error;
mod id;
mod key;
pub use id::*;
pub use key::*;

use crate::error::registry::Error as RegistryError;

static UNIQUES: OnceLock<HashMap<UniqueKey, StaticSoundData>> = OnceLock::new();
static GENDERS: OnceLock<HashMap<GenderKey, StaticSoundData>> = OnceLock::new();
static LOCALES: OnceLock<HashMap<LocaleKey, StaticSoundData>> = OnceLock::new();
static MULTIS: OnceLock<HashMap<BothKey, StaticSoundData>> = OnceLock::new();

static LOC_SUB: OnceLock<HashMap<LocaleKey, DialogLine>> = OnceLock::new();
static MUL_SUB: OnceLock<HashMap<BothKey, DialogLine>> = OnceLock::new();

static UNI_SET: OnceLock<HashMap<UniqueKey, Settings>> = OnceLock::new();
static GEN_SET: OnceLock<HashMap<GenderKey, Settings>> = OnceLock::new();
static LOC_SET: OnceLock<HashMap<LocaleKey, Settings>> = OnceLock::new();
static MUL_SET: OnceLock<HashMap<BothKey, Settings>> = OnceLock::new();

static KEYS: OnceLock<HashSet<Id>> = OnceLock::new();

pub struct Banks;
impl Banks {
    /// # Safety
    ///
    /// Will panic if [`Banks`] are not initialized yet.
    pub unsafe fn ids<'a>() -> &'a HashSet<Id> {
        KEYS.get().unwrap_unchecked()
    }
    pub fn exists(cname: &CName) -> bool {
        if cname == &CName::undefined() {
            return false;
        }
        KEYS.get()
            .and_then(|x| x.iter().find(|x| AsRef::<CName>::as_ref(x) == cname))
            .is_some()
    }
    pub fn exist<'a>(
        name: &CName,
        spoken: &Locale,
        gender: Option<&PlayerGender>,
    ) -> Result<&'a Id, RegistryError> {
        let mut maybe_missing_locale = false;
        if let Some(ids) = KEYS.get() {
            let mut key: &Key;
            for id in ids {
                key = id.as_ref();
                if let Some(key) = key.as_unique() {
                    if key.as_ref() == name {
                        return Ok(id);
                    }
                }
                if let Some(GenderKey(k, g)) = key.as_gender() {
                    if k == name {
                        if gender.is_none() {
                            return Err(RegistryError::RequireGender { cname: *name });
                        }
                        if Some(g) == gender {
                            return Ok(id);
                        }
                    }
                }
                if let Some(LocaleKey(k, l)) = key.as_locale() {
                    if k == name {
                        maybe_missing_locale = true;
                        if l == spoken {
                            return Ok(id);
                        }
                    }
                }
                if let Some(BothKey(k, l, g)) = key.as_both() {
                    if k == name {
                        maybe_missing_locale = true;
                        if l == spoken {
                            if gender.is_none() {
                                return Err(RegistryError::RequireGender { cname: *name });
                            }
                            if gender == Some(g) {
                                return Ok(id);
                            }
                        }
                    }
                }
            }
        }
        if maybe_missing_locale {
            return Err(RegistryError::MissingLocale {
                cname: *name,
                locale: *spoken,
            });
        }
        Err(RegistryError::NotFound { cname: *name })
    }
    pub fn data(id: &Id) -> Either<StaticSoundData, StreamingSoundData<FromFileError>> {
        let settings = Self::settings(id);
        match id {
            Id::OnDemand(Usage::Static(_, path)) => {
                let data = StaticSoundData::from_file(path)
                    .expect("static sound data has already been validated");
                if let Some(settings) = settings {
                    let settings = settings.left().expect("static sound settings should match");
                    return Either::Left(data.with_settings(settings));
                }
                Either::Left(data)
            }
            Id::OnDemand(Usage::Streaming(_, path)) => {
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
            Id::InMemory(Key::Unique(key)) => Either::Left(
                UNIQUES
                    .get()
                    .expect("insertion guarantees")
                    .get(key)
                    .expect("insertion guarantees")
                    .clone(),
            ),
            Id::InMemory(Key::Gender(key)) => Either::Left(
                GENDERS
                    .get()
                    .expect("insertion guarantees")
                    .get(key)
                    .expect("insertion guarantees")
                    .clone(),
            ),
            Id::InMemory(Key::Locale(key)) => Either::Left(
                LOCALES
                    .get()
                    .expect("insertion guarantees")
                    .get(key)
                    .expect("insertion guarantees")
                    .clone(),
            ),
            Id::InMemory(Key::Both(key)) => Either::Left(
                MULTIS
                    .get()
                    .expect("insertion guarantees")
                    .get(key)
                    .expect("insertion guarantees")
                    .clone(),
            ),
        }
    }
    fn settings(id: &Id) -> Option<Either<StaticSoundSettings, StreamingSoundSettings>> {
        match id {
            Id::OnDemand(Usage::Static(key, _)) => match key {
                Key::Unique(key) => UNI_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StaticSoundSettings::from)
                        .map(Either::Left)
                }),
                Key::Gender(key) => GEN_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StaticSoundSettings::from)
                        .map(Either::Left)
                }),
                Key::Locale(key) => LOC_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StaticSoundSettings::from)
                        .map(Either::Left)
                }),
                Key::Both(key) => MUL_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StaticSoundSettings::from)
                        .map(Either::Left)
                }),
            },
            Id::OnDemand(Usage::Streaming(key, _)) => match key {
                Key::Unique(key) => UNI_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StreamingSoundSettings::from)
                        .map(Either::Right)
                }),
                Key::Gender(key) => GEN_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StreamingSoundSettings::from)
                        .map(Either::Right)
                }),
                Key::Locale(key) => LOC_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StreamingSoundSettings::from)
                        .map(Either::Right)
                }),
                Key::Both(key) => MUL_SET.get().and_then(|x| {
                    x.get(key)
                        .cloned()
                        .map(StreamingSoundSettings::from)
                        .map(Either::Right)
                }),
            },
            // settings are already stored in-memory
            Id::InMemory(_) => None,
        }
    }
    pub fn setup() -> Initialization {
        let since = Instant::now();

        let mut errors: Vec<Error> = vec![];

        let mut mods = Vec::with_capacity(30);
        let mut redmod_exists = false;
        if let Ok(redmod) = REDmod::try_new() {
            mods = redmod.mods();
            redmod_exists = true;
        }
        if let Ok(r6audioware) = R6Audioware::try_new() {
            for m in r6audioware.mods() {
                if let Err(e) =
                    ensure_no_duplicate_accross_depots(redmod_exists, &m, mods.as_slice())
                {
                    errors.push(e);
                    continue;
                }
                mods.push(m);
            }
        }

        let mut file: Vec<u8>;
        let mut manifest: Manifest;
        let mut ids: HashSet<Id> = HashSet::new();
        let mut uniques: HashMap<UniqueKey, StaticSoundData> = HashMap::new();
        let mut genders: HashMap<GenderKey, StaticSoundData> = HashMap::new();
        let mut single_voices: HashMap<LocaleKey, StaticSoundData> = HashMap::new();
        let mut dual_voices: HashMap<BothKey, StaticSoundData> = HashMap::new();
        let mut single_subs: HashMap<LocaleKey, DialogLine> = HashMap::new();
        let mut dual_subs: HashMap<BothKey, DialogLine> = HashMap::new();
        let mut unique_settings: HashMap<UniqueKey, Settings> = HashMap::new();
        let mut gender_settings: HashMap<GenderKey, Settings> = HashMap::new();
        let mut single_settings: HashMap<LocaleKey, Settings> = HashMap::new();
        let mut dual_settings: HashMap<BothKey, Settings> = HashMap::new();

        for m in mods {
            let paths = m.manifests_paths();
            for ref path in paths {
                file = match std::fs::read(path).context(CannotReadManifestSnafu {
                    manifest: path.display().to_string(),
                }) {
                    Ok(x) => x,
                    Err(e) => {
                        errors.push(e.into());
                        continue;
                    }
                };
                manifest = match serde_yaml::from_slice::<Manifest>(file.as_slice()).context(
                    CannotParseManifestSnafu {
                        manifest: path.display().to_string(),
                    },
                ) {
                    Ok(x) => x,
                    Err(e) => {
                        errors.push(e.into());
                        continue;
                    }
                };
                if let Err(e) = ensure_manifest_no_duplicates(&manifest) {
                    errors.push(e);
                    continue;
                }
                if let Some(sfx) = manifest.sfx {
                    for (key, value) in sfx {
                        match ensure_sfx(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut uniques,
                            &mut unique_settings,
                        ) {
                            Ok(x) => x,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        }
                    }
                }
                if let Some(onos) = manifest.onos {
                    for (key, value) in onos {
                        match ensure_ono(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut genders,
                            &mut gender_settings,
                        ) {
                            Ok(x) => x,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        };
                    }
                }
                if let Some(voices) = manifest.voices {
                    for (key, value) in voices {
                        match ensure_voice(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut single_voices,
                            &mut dual_voices,
                            &mut single_subs,
                            &mut dual_subs,
                            &mut single_settings,
                            &mut dual_settings,
                        ) {
                            Ok(x) => x,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        };
                    }
                }
                if let Some(music) = manifest.music {
                    for (key, value) in music {
                        match ensure_music(key.as_str(), value, &m, &mut ids, &mut unique_settings)
                        {
                            Ok(x) => x,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        };
                    }
                }
            }
        }

        let lengths = ids.iter().fold((0, 0, 0), |acc, x| {
            let (mut odsta, mut odstr, mut imsta) = acc;
            match x {
                Id::OnDemand(Usage::Static(..)) => odsta += 1,
                Id::OnDemand(Usage::Streaming(..)) => odstr += 1,
                Id::InMemory(..) => imsta += 1,
            }
            (odsta, odstr, imsta)
        });

        let report = Initialization {
            duration: Instant::now() - since,
            lengths: format!(
                r##"ids:
- on-demand static audio    -> {}
- on-demand streaming audio -> {}
- in-memory static audio    -> {}"##,
                lengths.0, lengths.1, lengths.2
            ),
            len_ids: ids.len(),
            errors,
        };

        let _ = KEYS.set(ids);
        let _ = UNIQUES.set(uniques);
        let _ = GENDERS.set(genders);
        let _ = LOCALES.set(single_voices);
        let _ = MULTIS.set(dual_voices);
        let _ = LOC_SUB.set(single_subs);
        let _ = MUL_SUB.set(dual_subs);
        let _ = UNI_SET.set(unique_settings);
        let _ = GEN_SET.set(gender_settings);
        let _ = LOC_SET.set(single_settings);
        let _ = MUL_SET.set(dual_settings);

        report
    }
}

pub struct Initialization {
    duration: Duration,
    lengths: String,
    len_ids: usize,
    pub errors: Vec<Error>,
}

impl std::fmt::Display for Initialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Initialization {
            duration,
            lengths,
            len_ids,
            ..
        } = self;
        write!(
            f,
            r##"{lengths}
for a total of: {len_ids} id(s)
in {duration:?}
"##
        )
    }
}
