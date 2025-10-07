//! Guarantees to uphold at all time,
//! as explained in the [book](https://cyb3rpsych0s1s.github.io/audioware/MANIFEST.html#guarantees).

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::{Path, PathBuf},
};

use audioware_core::With;
use audioware_manifest::*;
use either::Either;
use kira::sound::{FromFileError, static_sound::StaticSoundData, streaming::StreamingSoundData};
use red4ext_rs::types::{CName, CNamePool, Cruid};
use snafu::ensure;

use crate::SceneKey;

use super::{
    BothKey, Error, GenderKey, Id, Key, LocaleKey, SceneBothKey, SceneId, SceneLocaleKey,
    UniqueKey,
    conflict::{Conflict, Conflictual},
    error::validation::{self, *},
};

/// Ensure no duplicate mod folder name across depots: `r6\audioware` and `mods`.
#[inline]
pub fn ensure_no_duplicate_accross_depots(
    redmod_exists: bool,
    r#mod: &Mod,
    mods: &[Mod],
) -> Result<(), Error> {
    ensure!(
        !redmod_exists || !mods.iter().any(|x| x.same_folder_name(r#mod.as_ref())),
        DuplicateAcrossDepotsSnafu {
            folder: r#mod.as_ref().display().to_string()
        }
    );
    Ok(())
}

#[cfg(not(feature = "hot-reload"))]
#[inline]
pub fn ensure_key_unique_or_inserted(cname: &str) -> Result<bool, Error> {
    ensure_key_unique(cname)?;
    Ok(true)
}

#[cfg(feature = "hot-reload")]
#[inline]
pub fn ensure_key_unique_or_inserted(cname: &str) -> Result<bool, Error> {
    // if it already existed in a previous load, it's probably a recycled key.
    if crate::PREVIOUS_IDS
        .lock()
        .unwrap()
        .iter()
        .any(|x| AsRef::<CName>::as_ref(x).as_str() == cname)
    {
        return Ok(true);
    }
    ensure_key_unique(cname)?;
    Ok(false)
}

#[cfg(not(feature = "hot-reload"))]
#[inline]
pub fn ensure_localized_key_unique_or_inserted(
    ids: &HashSet<Id>,
    cname: &str,
    locale: Locale,
) -> Result<bool, Error> {
    let existed = ensure_localized_key_unique(ids, cname, locale)?;
    Ok(existed)
}

#[cfg(feature = "hot-reload")]
#[inline]
pub fn ensure_localized_key_unique_or_inserted(
    ids: &HashSet<Id>,
    cname: &str,
    locale: Locale,
) -> Result<bool, Error> {
    // if it already existed in a previous load, it's probably a recycled key.
    if crate::PREVIOUS_IDS
        .lock()
        .unwrap()
        .iter()
        .any(|x| AsRef::<CName>::as_ref(x).as_str() == cname && x.locale() == Some(locale))
    {
        return Ok(true);
    }
    let existed = ensure_localized_key_unique(ids, cname, locale)?;
    Ok(existed)
}

/// Ensure [CName] does not already exist in [game pool](CNamePool),
/// unless it already exists for some [locale](Locale).
#[inline]
pub fn ensure_localized_key_unique(
    ids: &HashSet<Id>,
    cname: &str,
    _: Locale,
) -> Result<bool, Error> {
    // Conflict trait will make sure there's no identical duplicate, so no need to check twice.
    if ids
        .iter()
        .any(|x| AsRef::<CName>::as_ref(x).as_str() == cname && x.locale().is_some())
    {
        return Ok(true);
    }
    ensure_key_unique(cname)?;
    Ok(false)
}

/// Ensure [CName] does not already exist in [game pool](CNamePool).
#[inline]
pub fn ensure_key_unique(cname: &str) -> Result<(), Error> {
    ensure!(
        CName::new(cname).to_string().as_str() != cname,
        NonUniqueKeySnafu {
            cname: cname.to_string()
        }
    );
    Ok(())
}

/// Ensure [Key] variants do not [Conflict] with each others.
#[inline]
pub fn ensure_key_no_conflict<T: Conflictual>(
    key: &T,
    raw: &str,
    pool: &impl Conflict<T>,
) -> Result<(), Error> {
    ensure!(
        !pool.conflict(key),
        ConflictingKeySnafu {
            cname: raw.to_string()
        }
    );
    Ok(())
}

/// Ensure [SceneKey] variants do not [Conflict] with each others.
#[inline]
pub fn ensure_scene_key_no_conflict<T: Conflictual>(
    key: &T,
    raw: &(i64, Locale),
    pool: &impl Conflict<T>,
) -> Result<(), Error> {
    ensure!(
        !pool.conflict(key),
        ConflictingSceneKeySnafu {
            cruid: raw.0,
            locale: raw.1
        }
    );
    Ok(())
}

/// Ensure [Manifest] does not contain duplicate keys among
/// [Sfx],
/// [Ono],
/// etc.
pub fn ensure_manifest_no_duplicates(manifest: &Manifest) -> Result<(), Error> {
    let mut hashset = HashSet::with_capacity(100);
    if let Some(sfx) = manifest.sfx.as_ref() {
        for key in sfx.keys() {
            ensure!(
                hashset.insert(key.as_str()),
                ConflictingKeySnafu { cname: key.clone() }
            );
        }
    }
    if let Some(onos) = manifest.onos.as_ref() {
        for key in onos.keys() {
            ensure!(
                hashset.insert(key.as_str()),
                ConflictingKeySnafu { cname: key.clone() }
            );
        }
    }
    if let Some(voices) = manifest.voices.as_ref() {
        for key in voices.keys() {
            ensure!(
                hashset.insert(key.as_str()),
                ConflictingKeySnafu { cname: key.clone() }
            );
        }
    }
    if let Some(music) = manifest.music.as_ref() {
        for key in music.keys() {
            ensure!(
                hashset.insert(key.as_str()),
                ConflictingKeySnafu { cname: key.clone() }
            );
        }
    }
    Ok(())
}

/// Ensure audio file [Path] is located inside [Mod] depot.
pub fn ensure_located_in_depot(
    file: &impl AsRef<std::path::Path>,
    folder: &Mod,
) -> Result<(), Error> {
    let arg = std::fs::canonicalize(folder)?;
    let path = arg.join(file);
    if !path.starts_with(arg) {
        return Err(Error::Validation {
            source: validation::Error::AudioOutsideDepot {
                path: path.display().to_string(),
            },
        });
    }
    Ok(())
}

/// Ensure path refers to valid audio (based on [usage](Usage)).
pub fn ensure_valid_audio_and_settings(
    path: &impl AsRef<std::path::Path>,
    m: &Mod,
    usage: Usage,
    settings: Option<&Settings>,
) -> Result<Either<StaticSoundData, StreamingSoundData<FromFileError>>, Error> {
    let data = ensure_valid_audio_data(path, m, usage)?;
    ensure_valid_contextual_audio_settings(&data, settings, path.as_ref())?;
    Ok(data)
}

pub fn ensure_valid_audio_with_settings_and_captions(
    path: &impl AsRef<std::path::Path>,
    m: &Mod,
    usage: Usage,
    settings: Option<&Settings>,
    captions: Option<&[Caption]>,
) -> Result<Either<StaticSoundData, StreamingSoundData<FromFileError>>, Error> {
    let data = ensure_valid_audio_and_settings(path, m, usage, settings)?;
    if let Some(captions) = captions {
        ensure_valid_jingle_captions(&data, captions)?;
    }
    Ok(data)
}

pub fn ensure_valid_audio_data(
    path: &impl AsRef<std::path::Path>,
    m: &Mod,
    usage: Usage,
) -> Result<Either<StaticSoundData, StreamingSoundData<FromFileError>>, Error> {
    use snafu::ResultExt;
    let filepath = m.as_ref().join(path.as_ref());
    ensure_located_in_depot(path, m)?;
    let data = match usage {
        Usage::OnDemand | Usage::InMemory => StaticSoundData::from_file(filepath)
            .context(InvalidAudioSnafu {
                path: path.as_ref().display().to_string(),
            })
            .map(Either::Left)?,
        Usage::Streaming => StreamingSoundData::from_file(filepath)
            .context(InvalidAudioSnafu {
                path: path.as_ref().display().to_string(),
            })
            .map(Either::Right)?,
    };
    Ok(data)
}

/// Ensure given settings are valid for audio.
pub fn ensure_valid_contextual_audio_settings(
    audio: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    settings: Option<&Settings>,
    path: &Path,
) -> Result<(), Error> {
    let mut errors = settings.validate().err().unwrap_or_default();
    errors.extend(settings.validate_for(audio).err().unwrap_or_default());
    ensure!(
        errors.is_empty(),
        InvalidAudioSettingsSnafu {
            which: path.display().to_string(),
            why: errors
        }
    );
    Ok(())
}

#[doc(hidden)]
pub fn ensure_valid_jingle_captions(
    audio: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    captions: &[Caption],
) -> Result<(), Error> {
    if !captions.is_empty() {
        let duration = match audio {
            Either::Left(x) => x.duration(),
            Either::Right(x) => x.duration(),
        }
        .as_secs_f32();
        for (idx, caption) in captions.iter().enumerate() {
            if caption.starts >= duration {
                return Err(Error::from(validation::Error::InvalidAudioCaption {
                    which: "starts".to_string(),
                    why: format!("greater than audio duration (captions[{idx}])"),
                }));
            }
        }
        let mut previous_starts = captions[0].starts;
        for (idx, caption) in captions.iter().enumerate().skip(1) {
            if previous_starts >= caption.starts {
                return Err(Error::from(validation::Error::InvalidAudioCaption {
                    which: "starts".to_string(),
                    why: format!(
                        "unordered sequence (captions[{idx}] and captions[{}])",
                        idx - 1
                    ),
                }));
            }
            previous_starts = caption.starts;
        }
    }
    Ok(())
}

/// Ensure data is properly stored.
pub fn ensure_store_data<T: PartialEq + Eq + Hash + Clone + Into<Key>>(
    key: T,
    value: StaticSoundData,
    settings: Option<Settings>,
    path: &impl AsRef<std::path::Path>,
    store: &mut HashMap<T, StaticSoundData>,
) -> Result<(), Error> {
    let value = match settings {
        Some(settings) => value.with(settings),
        None => value,
    };
    ensure!(
        store.insert(key.clone(), value).is_none(),
        CannotStoreDataSnafu {
            key,
            path: path.as_ref().display().to_string()
        }
    );
    Ok(())
}

/// Ensure data is properly stored.
pub fn ensure_store_scene_data<T: PartialEq + Eq + Hash + Clone + Into<SceneKey>>(
    key: T,
    value: StaticSoundData,
    settings: Option<Settings>,
    path: &impl AsRef<std::path::Path>,
    store: &mut HashMap<T, StaticSoundData>,
) -> Result<(), Error> {
    let value = match settings {
        Some(settings) => value.with(settings),
        None => value,
    };
    ensure!(
        store.insert(key.clone(), value).is_none(),
        CannotStoreSceneDataSnafu {
            key,
            path: path.as_ref().display().to_string()
        }
    );
    Ok(())
}

/// Ensure subtitle is properly stored.
pub fn ensure_store_subtitle<T: PartialEq + Eq + Hash + Clone + Into<Key>>(
    key: T,
    value: DialogLine,
    store: &mut HashMap<T, DialogLine>,
) -> Result<(), Error> {
    ensure!(
        store.insert(key.clone(), value.clone()).is_none(),
        CannotStoreSubtitleSnafu { key, value }
    );
    Ok(())
}

/// Ensure settings are properly stored.
pub fn ensure_store_settings<T: PartialEq + Eq + Hash + Clone>(
    key: &T,
    value: Settings,
    store: &mut HashMap<T, Settings>,
) -> Result<(), Error> {
    ensure!(
        store.insert(key.clone(), value.clone()).is_none(),
        CannotStoreSettingsSnafu
    );
    Ok(())
}

/// Ensure [Id] is properly indexed in appropriate bank.
#[inline]
pub fn ensure_store_id(id: Id, store: &mut HashSet<Id>) -> Result<(), Error> {
    ensure!(store.insert(id.clone()), CannotStoreAgnosticIdSnafu { id });
    Ok(())
}

/// Ensure [SceneId] is properly indexed in appropriate bank.
#[inline]
pub fn ensure_store_scene_id(id: SceneId, store: &mut HashSet<SceneId>) -> Result<(), Error> {
    ensure!(store.insert(id.clone()), CannotStoreSceneIdSnafu { id });
    Ok(())
}

/// Ensure guarantees are upheld.
#[allow(clippy::too_many_arguments)]
fn ensure<'a, K: PartialEq + Eq + Hash + Clone + Into<Key> + Conflictual>(
    k: &'a str,
    key: K,
    path: PathBuf,
    m: &Mod,
    usage: Usage,
    settings: Option<Settings>,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<K, StaticSoundData>,
    smap: &'a mut HashMap<K, Settings>,
    source: Source,
) -> Result<(), Error>
where
    HashSet<Id>: Conflict<K>,
{
    let data = ensure_valid_audio_and_settings(&path, m, usage, settings.as_ref())?
        .map_either_with(
            (usage, settings.as_ref().and_then(|x| x.region.clone())),
            |ctx, data| {
                if let (Usage::InMemory, Some(region)) = ctx {
                    data.slice(region)
                } else {
                    data
                }
            },
            |_, data| data,
        );
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = match usage {
        Usage::InMemory => Id::InMemory(key.clone().into(), source),
        Usage::OnDemand => Id::OnDemand(
            crate::Usage::Static(key.clone().into(), m.as_ref().join(path.clone())),
            source,
        ),
        Usage::Streaming => Id::OnDemand(
            crate::Usage::Streaming(key.clone().into(), m.as_ref().join(path.clone())),
            source,
        ),
    };
    if usage == Usage::InMemory {
        ensure_store_data(key, data.left().unwrap(), settings, &path, map)?;
    } else if let Some(settings) = settings {
        ensure_store_settings(&key, settings, smap)?;
    }
    ensure_store_id(id, set)?;
    Ok(())
}

/// Ensure guarantees are upheld.
#[allow(clippy::too_many_arguments)]
fn ensure_scene<'a, K: PartialEq + Eq + Hash + Clone + Into<SceneKey> + Conflictual>(
    key: K,
    path: PathBuf,
    m: &Mod,
    usage: Usage,
    settings: Option<Settings>,
    set: &'a mut HashSet<SceneId>,
    map: &'a mut HashMap<K, StaticSoundData>,
    smap: &'a mut HashMap<K, Settings>,
) -> Result<(), Error>
where
    HashSet<SceneId>: Conflict<K>,
    SceneKey: From<K>,
{
    let data = ensure_valid_audio_and_settings(&path, m, usage, settings.as_ref())?
        .map_either_with(
            (usage, settings.as_ref().and_then(|x| x.region.clone())),
            |ctx, data| {
                if let (Usage::InMemory, Some(region)) = ctx {
                    data.slice(region)
                } else {
                    data
                }
            },
            |_, data| data,
        );
    let id: SceneId = match usage {
        Usage::InMemory => SceneId::InMemory(key.clone().into()),
        Usage::OnDemand => SceneId::OnDemand(crate::Usage::Static(
            key.clone().into(),
            m.as_ref().join(path.clone()),
        )),
        Usage::Streaming => SceneId::OnDemand(crate::Usage::Streaming(
            key.clone().into(),
            m.as_ref().join(path.clone()),
        )),
    };
    if usage == Usage::InMemory {
        ensure_store_scene_data(key, data.left().unwrap(), settings, &path, map)?;
    } else if let Some(settings) = settings {
        ensure_store_settings(&key, settings, smap)?;
    }
    ensure_store_scene_id(id, set)?;
    Ok(())
}

/// Ensure [Sfx] guarantees are upheld.
pub fn ensure_sfx<'a>(
    k: &'a str,
    v: Sfx,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<UniqueKey, StaticSoundData>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
) -> Result<(), Error> {
    let existed = ensure_key_unique_or_inserted(k)?;
    let UsableAudio {
        audio: Audio { file, settings },
        usage,
    } = v.into();
    let c_string = std::ffi::CString::new(k)?;
    let cname = CName::new(k);
    let key = UniqueKey(cname);
    ensure(
        k,
        key,
        file,
        m,
        usage.unwrap_or(Usage::InMemory),
        settings,
        set,
        map,
        smap,
        Source::Sfx,
    )?;

    if !existed {
        CNamePool::add_cstr(&c_string);
    }
    Ok(())
}

/// Ensure [Ono] guarantees are upheld.
pub fn ensure_ono<'a>(
    k: &'a str,
    v: Ono,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<GenderKey, StaticSoundData>,
    smap: &'a mut HashMap<GenderKey, Settings>,
) -> Result<(), Error> {
    let existed = ensure_key_unique_or_inserted(k)?;
    let (usage, genders) = v.into();
    let c_string = std::ffi::CString::new(k)?;
    let cname = CName::new(k);
    let mut key: GenderKey;
    for (gender, Audio { file, settings }) in genders {
        key = GenderKey(cname, gender);
        ensure(
            k,
            key,
            file,
            m,
            usage,
            settings.clone(),
            set,
            map,
            smap,
            Source::Ono,
        )?;
    }

    if !existed {
        CNamePool::add_cstr(&c_string);
    }
    Ok(())
}

/// Ensure [Voice] guarantees are upheld.
#[allow(clippy::too_many_arguments)]
pub fn ensure_voice<'a>(
    k: &'a str,
    v: Voice,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    simple: &'a mut HashMap<LocaleKey, StaticSoundData>,
    complex: &'a mut HashMap<BothKey, StaticSoundData>,
    simple_subs: &'a mut HashMap<LocaleKey, DialogLine>,
    complex_subs: &'a mut HashMap<BothKey, DialogLine>,
    simple_settings: &'a mut HashMap<LocaleKey, Settings>,
    complex_settings: &'a mut HashMap<BothKey, Settings>,
) -> Result<(), Error> {
    let mut existed = false;
    let v: AnyVoice = v.into();
    let c_string = std::ffi::CString::new(k)?;
    let cname = CName::new(k);
    let mut simple_key: LocaleKey;
    let mut complex_key: BothKey;
    match v {
        Either::Left((aud, usage, subs)) => {
            for (locale, Audio { file, settings }) in aud {
                existed = existed || ensure_localized_key_unique_or_inserted(set, k, locale)?;
                simple_key = LocaleKey(cname, locale);
                if let Some(subs) = subs.as_ref().and_then(|x| x.get(&locale)) {
                    ensure_store_subtitle::<LocaleKey>(
                        simple_key.clone(),
                        subs.clone(),
                        simple_subs,
                    )?;
                }
                ensure(
                    k,
                    simple_key,
                    file,
                    m,
                    usage,
                    settings.clone(),
                    set,
                    simple,
                    simple_settings,
                    Source::Voices,
                )?;
            }
        }
        Either::Right((aud, usage, subs)) => {
            for (locale, genders) in aud {
                existed = existed || ensure_localized_key_unique_or_inserted(set, k, locale)?;
                for (gender, Audio { file, settings }) in genders {
                    complex_key = BothKey(cname, locale, gender);
                    if let Some(subs) = subs.as_ref().and_then(|x| x.get(&locale)) {
                        ensure_store_subtitle::<BothKey>(
                            complex_key.clone(),
                            if gender == PlayerGender::Female {
                                subs.female.clone()
                            } else {
                                subs.male.clone()
                            },
                            complex_subs,
                        )?;
                    }
                    ensure(
                        k,
                        complex_key,
                        file,
                        m,
                        usage,
                        settings.clone(),
                        set,
                        complex,
                        complex_settings,
                        Source::Voices,
                    )?;
                }
            }
        }
    }

    if !existed {
        CNamePool::add_cstr(&c_string);
    }
    Ok(())
}

/// Ensure [Music] guarantees are upheld.
pub fn ensure_music<'a>(
    k: &'a str,
    v: Music,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<UniqueKey, StaticSoundData>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
) -> Result<(), Error> {
    let existed = ensure_key_unique_or_inserted(k)?;
    let UsableAudio {
        audio: Audio { file, settings },
        usage,
    } = v.into();
    let c_string = std::ffi::CString::new(k)?;
    let cname = CName::new(k);
    let key = UniqueKey(cname);
    ensure(
        k,
        key,
        file,
        m,
        usage.unwrap_or(Usage::Streaming),
        settings,
        set,
        map,
        smap,
        Source::Music,
    )?;

    if !existed {
        CNamePool::add_cstr(&c_string);
    }
    Ok(())
}

#[doc(hidden)]
pub fn ensure_jingles<'a>(
    k: &'a str,
    v: Jingle,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
) -> Result<(), Error> {
    let existed = ensure_key_unique_or_inserted(k)?;
    let Audio { file, settings } = (&v).into();
    ensure_valid_audio_with_settings_and_captions(
        &file,
        m,
        Usage::Streaming,
        settings.as_ref(),
        v.captions(),
    )?;
    let c_string = std::ffi::CString::new(k)?;
    let cname = CName::new(k);
    let key = UniqueKey(cname);
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = Id::OnDemand(
        crate::Usage::Streaming(crate::Key::Unique(key.clone()), m.as_ref().join(file)),
        Source::Jingle,
    );
    if let Some(settings) = settings {
        ensure_store_settings::<UniqueKey>(&key, settings, smap)?;
    }
    ensure_store_id(id, set)?;

    if !existed {
        CNamePool::add_cstr(&c_string);
    }
    Ok(())
}

#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub fn ensure_scene_dialogs<'a>(
    k: i64,
    v: SceneDialogs,
    m: &Mod,
    set: &'a mut HashSet<SceneId>,
    single: &'a mut HashMap<SceneLocaleKey, StaticSoundData>,
    dual: &'a mut HashMap<SceneBothKey, StaticSoundData>,
    single_settings: &'a mut HashMap<SceneLocaleKey, Settings>,
    dual_settings: &'a mut HashMap<SceneBothKey, Settings>,
) -> Result<(), Error> {
    let mut errors = Vec::with_capacity(10);
    let mut locale_key: SceneLocaleKey;
    let mut both_key: SceneBothKey;
    let v: AnySceneDialog = v.into();
    let cruid = Cruid::from(k);
    match v {
        Either::Left((aud, usage)) => {
            for (locale, Audio { file, settings }) in aud {
                locale_key = SceneLocaleKey(cruid, locale);
                ensure_scene_key_no_conflict(&locale_key, &(k, locale), set)?;
                if let Err(e) = ensure_scene(
                    locale_key,
                    file,
                    m,
                    usage,
                    settings,
                    set,
                    single,
                    single_settings,
                ) {
                    errors.push(e);
                    continue;
                }
            }
        }
        Either::Right((aud, usage)) => {
            let mut checked_conflict;
            'outer: for (locale, genders) in aud {
                checked_conflict = false;
                for (gender, Audio { file, settings }) in genders {
                    both_key = SceneBothKey(Cruid::from(k), locale, gender);
                    // only check once per pair, otherwise gender will conflict with each other
                    if !checked_conflict {
                        if let Err(e) = ensure_scene_key_no_conflict(&both_key, &(k, locale), set) {
                            errors.push(e);
                            continue 'outer;
                        }
                        checked_conflict = true;
                    }
                    if let Err(e) =
                        ensure_scene(both_key, file, m, usage, settings, set, dual, dual_settings)
                    {
                        errors.push(e);
                        // other gender might already have been added
                        dual.remove(&SceneBothKey(Cruid::from(k), locale, gender.opposite()));
                        dual_settings.remove(&SceneBothKey(
                            Cruid::from(k),
                            locale,
                            gender.opposite(),
                        ));
                        continue 'outer;
                    }
                }
            }
        }
    };
    Ok(())
}

#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub fn ensure_main_menu(v: &MainMenu, m: &Mod) -> Result<(), Error> {
    ensure_valid_audio_and_settings(v, m, Usage::Streaming, None)?;
    Ok(())
}
