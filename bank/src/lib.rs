//! Banks storage.

use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
    time::{Duration, Instant},
};

use audioware_core::{AudioDuration, With};
use audioware_manifest::{
    error::{CannotParseManifest, CannotReadManifest},
    Depot, DialogLine, Locale, Manifest, PlayerGender, R6Audioware, REDmod, Settings, SpokenLocale,
};
use either::Either;
use ensure::*;
use kira::sound::static_sound::StaticSoundData;
use red4ext_rs::types::CName;
use snafu::ResultExt;

mod bnk;
pub mod conflict;
mod ensure;
pub mod error;
pub use error::Error;
mod id;
mod key;
pub use bnk::*;
pub use id::*;
pub use key::*;
mod storage;
pub use storage::*;

use crate::error::registry::Error as RegistryError;

static KEYS: OnceLock<HashSet<Id>> = OnceLock::new();

pub struct Banks;
impl Banks {
    /// # Safety
    ///
    /// Will panic if [Banks] are not initialized yet.
    pub unsafe fn ids<'a>() -> &'a HashSet<Id> {
        KEYS.get().unwrap_unchecked()
    }
    /// Whether audio ID exists in banks or not.
    pub fn exists(cname: &CName) -> bool {
        if cname == &CName::undefined() {
            return false;
        }
        KEYS.get()
            .and_then(|x| x.iter().find(|x| AsRef::<CName>::as_ref(x) == cname))
            .is_some()
    }
    /// Return audio duration (as seconds) if any, otherwise `-1.0`.
    pub fn duration(cname: &CName, locale: Locale, gender: PlayerGender, total: bool) -> f32 {
        let locale = SpokenLocale::from(locale);
        if let Ok(id) = Self::try_get(cname, &locale, Some(&gender)) {
            match (total, id, Banks.data(id)) {
                // if no need for total and in-memory, sound data already embed settings
                (false, Id::InMemory(..), data) => data
                    .left()
                    .expect("streaming cannot be stored in-memory")
                    .slice_duration(),
                // if no need for total and on-demand, check settings
                (false, Id::OnDemand(..), data) => match (data, Banks.settings(id)) {
                    (Either::Left(x), settings) => x.with(settings).slice_duration(),
                    (Either::Right(x), settings) => x.with(settings).slice_duration(),
                },
                // if need total
                (true, _, data) => match data {
                    Either::Left(x) => x.total_duration(),
                    Either::Right(x) => x.total_duration(),
                },
            }
            .as_secs_f32()
        } else {
            -1.0
        }
    }
    /// All languages found in [Manifest]s.
    pub fn languages() -> HashSet<Locale> {
        let mut out = HashSet::new();
        for key in LOC_SUB.keys() {
            out.insert(key.1);
        }
        for key in MUL_SUB.keys() {
            out.insert(key.1);
        }
        out
    }
    pub fn try_get<'a>(
        name: &CName,
        spoken: &SpokenLocale,
        gender: Option<&PlayerGender>,
    ) -> Result<&'a Id, Error> {
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
                            return Err(RegistryError::RequireGender { cname: *name }.into());
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
                                return Err(RegistryError::RequireGender { cname: *name }.into());
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
            return Err(RegistryError::MissingSpokenLocale {
                cname: *name,
                locale: *spoken,
            }
            .into());
        }
        Err(RegistryError::NotFound { cname: *name }.into())
    }
    /// Initialize banks.
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
        let mut bnks: HashMap<CName, crate::bnk::SoundBank> = HashMap::new();
        let mut bnks_count: usize = 0;

        for m in mods {
            let paths = m.manifests_paths();
            for ref path in paths {
                file = match std::fs::read(path).context(CannotReadManifest {
                    manifest: path.display().to_string(),
                }) {
                    Ok(x) => x,
                    Err(e) => {
                        errors.push(e.into());
                        continue;
                    }
                };
                match serde_yaml::from_slice::<Manifest>(file.as_slice()).context(
                    CannotParseManifest {
                        manifest: path.display().to_string(),
                    },
                ) {
                    Ok(x) => {
                        manifest = x;
                    }
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
                        match ensure_music(
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
                        };
                    }
                }
                if let Some(jingles) = manifest.jingles {
                    for (key, value) in jingles {
                        match ensure_jingles(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut unique_settings,
                        ) {
                            Ok(x) => x,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        };
                    }
                }
                if let Some(banks) = manifest.banks {
                    for (key, value) in banks {
                        match ensure_bnk(key.as_str(), value, &mut bnks, &mut ids) {
                            Ok(_) => {
                                bnks_count += 1;
                            }
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        }
                    }
                }
            }
        }

        let lengths = ids.iter().fold((0, 0, 0), |acc, x| {
            let (mut odsta, mut odstr, mut imsta) = acc;
            match x {
                Id::OnDemand(Usage::Static(..), ..) => odsta += 1,
                Id::OnDemand(Usage::Streaming(..), ..) => odstr += 1,
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
- in-memory static audio    -> {}
- .bnk                      -> {}"##,
                lengths.0, lengths.1, lengths.2, bnks_count,
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
        let _ = BNKS.set(bnks);

        report
    }
}

/// Outcome of [Banks] initialization.
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
