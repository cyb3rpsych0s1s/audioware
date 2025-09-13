//! Banks storage.

use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use audioware_core::{AudioDuration, With};
use audioware_manifest::{
    Depot, DialogLine, Locale, Manifest, Mod, PlayerGender, R6Audioware, REDmod, Settings,
    SpokenLocale,
    error::{CannotParseManifest, CannotReadManifest},
};
use either::Either;
use ensure::*;
use kira::sound::static_sound::StaticSoundData;
use red4ext_rs::types::{CName, Cruid};
use snafu::ResultExt;

pub mod conflict;
mod ensure;
pub mod error;
pub use error::Error;
mod id;
mod key;
mod scene_id;
mod scene_key;
pub use id::*;
pub use key::*;
pub use scene_id::*;
pub use scene_key::*;
mod storage;
pub use storage::*;
mod usage;
pub use usage::*;

use crate::error::registry::{Error as RegistryError, ErrorDisplay};

#[cfg(feature = "hot-reload")]
static PREVIOUS_IDS: std::sync::LazyLock<std::sync::Mutex<HashSet<Id>>> =
    std::sync::LazyLock::new(Default::default);

#[cfg(feature = "hot-reload")]
static PREVIOUS_SCENE_IDS: std::sync::LazyLock<std::sync::Mutex<HashSet<SceneId>>> =
    std::sync::LazyLock::new(Default::default);

#[derive(Clone)]
pub struct Banks {
    pub ids: HashSet<Id>,
    pub scene_ids: HashSet<SceneId>,
    pub uniques: HashMap<UniqueKey, StaticSoundData>,
    pub genders: HashMap<GenderKey, StaticSoundData>,
    pub single_voices: HashMap<LocaleKey, StaticSoundData>,
    pub dual_voices: HashMap<BothKey, StaticSoundData>,
    pub single_subs: HashMap<LocaleKey, DialogLine>,
    pub dual_subs: HashMap<BothKey, DialogLine>,
    pub unique_settings: HashMap<UniqueKey, Settings>,
    pub gender_settings: HashMap<GenderKey, Settings>,
    pub single_settings: HashMap<LocaleKey, Settings>,
    pub dual_settings: HashMap<BothKey, Settings>,
    pub single_scene_dialogs: HashMap<SceneLocaleKey, StaticSoundData>,
    pub dual_scene_dialogs: HashMap<SceneBothKey, StaticSoundData>,
    pub single_scene_dialogs_settings: HashMap<SceneLocaleKey, Settings>,
    pub dual_scene_dialogs_settings: HashMap<SceneBothKey, Settings>,
}

impl Banks {
    /// # Safety
    ///
    /// Will panic if [Banks] are not initialized yet.
    pub unsafe fn ids(&self) -> &HashSet<Id> {
        &self.ids
    }
    /// Whether audio ID exists in banks or not.
    pub fn exists(&self, cname: &CName) -> bool {
        if cname == &CName::undefined() {
            return false;
        }
        self.ids.iter().any(|x| AsRef::<CName>::as_ref(&x) == cname)
    }
    /// Whether RUID exists in banks or not.
    pub fn exists_for_scene(&self, cruid: &Cruid) -> bool {
        if !cruid.is_defined() {
            return false;
        }
        self.scene_ids
            .iter()
            .any(|x| AsRef::<Cruid>::as_ref(&x) == cruid)
    }
    /// Return audio duration (as seconds) if any, otherwise `-1.0`.
    pub fn duration(
        &self,
        cname: &CName,
        locale: Locale,
        gender: PlayerGender,
        total: bool,
    ) -> f32 {
        let locale = SpokenLocale::from(locale);
        if let Ok(id) = self.ids.try_get(cname, &locale, Some(&gender)) {
            match (total, id, self.data(id)) {
                // if no need for total and in-memory, sound data already embed settings
                (false, Id::InMemory(..), data) => data
                    .left()
                    .expect("streaming cannot be stored in-memory")
                    .slice_duration(),
                // if no need for total and on-demand, check settings
                (false, Id::OnDemand(..), data) => match (data, self.settings(id)) {
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
    pub fn languages(&self) -> HashSet<Locale> {
        let mut out = HashSet::new();
        for key in self.single_subs.keys() {
            out.insert(key.1);
        }
        for key in self.dual_subs.keys() {
            out.insert(key.1);
        }
        out
    }
    fn mods() -> (Vec<Mod>, Vec<Error>) {
        let mut errors = Vec::with_capacity(10);
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
        (mods, errors)
    }
    /// Initialize banks.
    pub fn new() -> (Self, Initialization) {
        let since = Instant::now();

        let mut file: Vec<u8>;
        let mut manifest: Manifest;
        let (mods, mut errors) = Self::mods();
        let mut scene_errors = Vec::with_capacity(30);

        let mut ids: HashSet<Id> = HashSet::new();
        let mut scene_ids: HashSet<SceneId> = HashSet::new();
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
        let mut single_scene_dialogs: HashMap<SceneLocaleKey, StaticSoundData> = HashMap::new();
        let mut dual_scene_dialogs: HashMap<SceneBothKey, StaticSoundData> = HashMap::new();
        let mut single_scene_dialogs_settings: HashMap<SceneLocaleKey, Settings> = HashMap::new();
        let mut dual_scene_dialogs_settings: HashMap<SceneBothKey, Settings> = HashMap::new();

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
                if let Some(scene_dialogs) = manifest.scene_dialogs {
                    for (key, value) in scene_dialogs {
                        match ensure_scene_dialogs(
                            key as i64,
                            value,
                            &m,
                            &mut scene_ids,
                            &mut single_scene_dialogs,
                            &mut dual_scene_dialogs,
                            &mut single_scene_dialogs_settings,
                            &mut dual_scene_dialogs_settings,
                        ) {
                            Ok(x) => x,
                            Err(e) => {
                                scene_errors.push(e);
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
                Id::OnDemand(Usage::Static(..), ..) => odsta += 1,
                Id::OnDemand(Usage::Streaming(..), ..) => odstr += 1,
                Id::InMemory(..) => imsta += 1,
            }
            (odsta, odstr, imsta)
        });

        let scene_lengths = scene_ids.iter().fold((0, 0, 0), |acc, x| {
            let (mut odsta, mut odstr, mut imsta) = acc;
            match x {
                SceneId::OnDemand(Usage::Static(..), ..) => odsta += 1,
                SceneId::OnDemand(Usage::Streaming(..), ..) => odstr += 1,
                SceneId::InMemory(..) => imsta += 1,
            }
            (odsta, odstr, imsta)
        });

        #[cfg(feature = "hot-reload")]
        let errors = errors
            .into_iter()
            .map(std::sync::Arc::new)
            .collect::<Vec<_>>();

        #[cfg(feature = "hot-reload")]
        let scene_errors = scene_errors
            .into_iter()
            .map(std::sync::Arc::new)
            .collect::<Vec<_>>();

        let report = Initialization {
            duration: Instant::now() - since,
            lengths: format!(
                r##"ids:
- on-demand static audio    -> {}
- on-demand streaming audio -> {}
- in-memory static audio    -> {}"##,
                lengths.0, lengths.1, lengths.2
            ),
            scene_lengths: format!(
                r##"scene ids:
- on-demand static audio    -> {}
- on-demand streaming audio -> {}
- in-memory static audio    -> {}"##,
                scene_lengths.0, scene_lengths.1, scene_lengths.2
            ),
            len_ids: ids.len(),
            len_scene_ids: scene_ids.len(),
            errors,
            scene_errors,
        };

        (
            Self {
                ids,
                scene_ids,
                uniques,
                genders,
                single_voices,
                dual_voices,
                single_subs,
                dual_subs,
                unique_settings,
                gender_settings,
                single_settings,
                dual_settings,
                single_scene_dialogs,
                dual_scene_dialogs,
                single_scene_dialogs_settings,
                dual_scene_dialogs_settings,
            },
            report,
        )
    }
    #[cfg(feature = "hot-reload")]
    pub fn hot_reload(&mut self) -> Initialization {
        PREVIOUS_IDS
            .lock()
            .expect("already loaded before")
            .clone_from(&self.ids.drain().collect());
        PREVIOUS_SCENE_IDS
            .lock()
            .expect("already loaded before")
            .clone_from(&self.scene_ids.drain().collect());
        let (banks, initialization) = Self::new();
        self.ids = banks.ids;
        self.scene_ids = banks.scene_ids;
        self.uniques = banks.uniques;
        self.genders = banks.genders;
        self.single_voices = banks.single_voices;
        self.dual_voices = banks.dual_voices;
        self.single_subs = banks.single_subs;
        self.dual_subs = banks.dual_subs;
        self.unique_settings = banks.unique_settings;
        self.gender_settings = banks.gender_settings;
        self.single_settings = banks.single_settings;
        self.dual_settings = banks.dual_settings;
        self.single_scene_dialogs = banks.single_scene_dialogs;
        self.dual_scene_dialogs = banks.dual_scene_dialogs;
        self.single_scene_dialogs_settings = banks.single_scene_dialogs_settings;
        self.dual_scene_dialogs_settings = banks.dual_scene_dialogs_settings;
        initialization
    }
}

/// Outcome of [Banks] initialization.
#[cfg_attr(feature = "hot-reload", derive(Clone))]
pub struct Initialization {
    duration: Duration,
    lengths: String,
    scene_lengths: String,
    len_ids: usize,
    len_scene_ids: usize,
    #[cfg(not(feature = "hot-reload"))]
    pub errors: Vec<Error>,
    #[cfg(feature = "hot-reload")]
    pub errors: Vec<std::sync::Arc<Error>>,
    #[cfg(not(feature = "hot-reload"))]
    pub scene_errors: Vec<Error>,
    #[cfg(feature = "hot-reload")]
    pub scene_errors: Vec<std::sync::Arc<Error>>,
}

pub enum InitializationOutcome {
    CompleteFailure = 0,
    PartialSuccess = 1,
    Success = 2,
}

impl Initialization {
    pub fn outcome_sections(&self) -> InitializationOutcome {
        if self.errors.is_empty() {
            InitializationOutcome::Success
        } else if self.len_ids == 0 {
            InitializationOutcome::CompleteFailure
        } else {
            InitializationOutcome::PartialSuccess
        }
    }
    pub fn outcome_scene(&self) -> InitializationOutcome {
        if self.scene_errors.is_empty() {
            InitializationOutcome::Success
        } else if self.len_scene_ids == 0 {
            InitializationOutcome::CompleteFailure
        } else {
            InitializationOutcome::PartialSuccess
        }
    }
    pub fn outcome(&self) -> InitializationOutcome {
        match (self.outcome_sections(), self.outcome_scene()) {
            (InitializationOutcome::Success, InitializationOutcome::Success) => {
                InitializationOutcome::Success
            }
            (InitializationOutcome::CompleteFailure, InitializationOutcome::CompleteFailure) => {
                InitializationOutcome::CompleteFailure
            }
            _ => InitializationOutcome::PartialSuccess,
        }
    }
}

impl std::fmt::Display for Initialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Initialization {
            duration,
            lengths,
            len_ids,
            errors,
            scene_errors,
            scene_lengths,
            len_scene_ids,
        } = self;
        write!(
            f,
            r##"initialization took {duration:?}
{lengths}
for a total of: {len_ids} id(s)
{}
-------------------------------
{scene_lengths}
for a total of: {len_scene_ids} scene id(s)
{}
"##,
            if errors.is_empty() {
                "no error reported!".to_string()
            } else {
                format!(
                    "error(s):\n{}",
                    errors
                        .iter()
                        .map(|e| format!("- {e}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            },
            if scene_errors.is_empty() {
                "no scene error reported!".to_string()
            } else {
                format!(
                    "scene error(s):\n{}",
                    scene_errors
                        .iter()
                        .map(|e| format!("- {e}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        )
    }
}

pub trait TryGet {
    type Id;
    type Raw;
    fn try_get<K: ErrorDisplay + Clone>(
        &self,
        name: &K,
        spoken: &SpokenLocale,
        gender: Option<&PlayerGender>,
    ) -> Result<&Self::Id, Error>
    where
        Self::Raw: PartialEq<K>,
        error::Error: From<crate::error::registry::Error<K>>;
}

impl TryGet for HashSet<Id> {
    type Id = Id;
    type Raw = CName;

    fn try_get<K: ErrorDisplay + Clone>(
        &self,
        name: &K,
        spoken: &SpokenLocale,
        gender: Option<&PlayerGender>,
    ) -> Result<&Self::Id, Error>
    where
        Self::Raw: PartialEq<K>,
        error::Error: From<crate::error::registry::Error<K>>,
    {
        let mut maybe_missing_locale = false;
        let mut key: &Key;
        for id in self.iter() {
            key = id.as_ref();
            if let Some(key) = key.as_unique()
                && key.as_ref() == name
            {
                return Ok(id);
            }
            if let Some(GenderKey(k, g)) = key.as_gender()
                && k == name
            {
                if gender.is_none() {
                    return Err(RegistryError::RequireGender { key: name.clone() }.into());
                }
                if Some(g) == gender {
                    return Ok(id);
                }
            }
            if let Some(LocaleKey(k, l)) = key.as_locale()
                && k == name
            {
                maybe_missing_locale = true;
                if l == spoken {
                    return Ok(id);
                }
            }
            if let Some(BothKey(k, l, g)) = key.as_both()
                && k == name
            {
                maybe_missing_locale = true;
                if l == spoken {
                    if gender.is_none() {
                        return Err(RegistryError::RequireGender { key: name.clone() }.into());
                    }
                    if gender == Some(g) {
                        return Ok(id);
                    }
                }
            }
        }
        if maybe_missing_locale {
            return Err(RegistryError::MissingSpokenLocale {
                key: name.clone(),
                locale: *spoken,
            }
            .into());
        }
        Err(RegistryError::NotFound { key: name.clone() }.into())
    }
}

impl TryGet for HashSet<SceneId> {
    type Id = SceneId;
    type Raw = Cruid;

    fn try_get<K: ErrorDisplay + Clone>(
        &self,
        name: &K,
        spoken: &SpokenLocale,
        gender: Option<&PlayerGender>,
    ) -> Result<&Self::Id, Error>
    where
        Self::Raw: PartialEq<K>,
        error::Error: From<crate::error::registry::Error<K>>,
    {
        let mut maybe_missing_locale = false;
        let mut key: &SceneKey;
        for id in self.iter() {
            key = id.as_ref();
            match key {
                SceneKey::Locale(SceneLocaleKey(key, locale)) if key == name => {
                    maybe_missing_locale = true;
                    if locale == spoken {
                        return Ok(id);
                    }
                }
                SceneKey::Both(SceneBothKey(key, locale, g)) if key == name => {
                    maybe_missing_locale = true;
                    if locale == spoken {
                        if gender.is_none() {
                            return Err(RegistryError::RequireGender { key: name.clone() }.into());
                        }
                        if gender == Some(g) {
                            return Ok(id);
                        }
                    }
                }
                _ => continue,
            }
        }
        if maybe_missing_locale {
            return Err(RegistryError::MissingSpokenLocale {
                key: name.clone(),
                locale: *spoken,
            }
            .into());
        }
        Err(RegistryError::NotFound { key: name.clone() }.into())
    }
}
