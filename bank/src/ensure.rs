use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::PathBuf,
};

use audioware_manifest::*;
use either::Either;
use kira::sound::{
    static_sound::{StaticSoundData, StaticSoundSettings},
    streaming::{StreamingSoundData, StreamingSoundSettings},
    FromFileError,
};
use red4ext_rs::types::{CName, CNamePool};
use snafu::ensure;

use crate::{error::validation::*, Id};

use super::{
    conflict::{Conflict, Conflictual},
    BothKey, Error, GenderKey, Key, LocaleKey, UniqueKey,
};

/// ensure no duplicate mod folder name across depots: `r6\audioware` and `mods`.
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

/// ensure [`CName`] does not already exist in game pool.
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

/// ensure [`Key`](crate::Key) variants do not [`Conflict`].
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

/// ensure [`Manifest`] does not contain duplicate keys among
/// [`Sfx`](crate::manifest::de::Sfx),
/// [`Ono`](crate::manifest::de::Ono),
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

/// ensure [`Path`](std::path::Path) contains valid audio (based on usage)
#[inline]
pub fn ensure_valid_audio(
    path: &impl AsRef<std::path::Path>,
    m: &Mod,
    usage: Usage,
    settings: Option<&Settings>,
) -> Result<Either<StaticSoundData, StreamingSoundData<FromFileError>>, Error> {
    use snafu::ResultExt;
    let filepath = m.as_ref().join(path.as_ref());
    let data = match usage {
        Usage::OnDemand | Usage::InMemory => {
            let data = StaticSoundData::from_file(filepath).context(InvalidAudioSnafu {
                path: path.as_ref().display().to_string(),
            })?;
            match settings.map(|x| StaticSoundSettings::from(x.clone())) {
                Some(settings) => Either::Left(data.with_settings(settings)),
                None => Either::Left(data),
            }
        }
        Usage::Streaming => {
            let data = StreamingSoundData::from_file(filepath).context(InvalidAudioSnafu {
                path: path.as_ref().display().to_string(),
            })?;
            match settings.map(|x| StreamingSoundSettings::from(x.clone())) {
                Some(settings) => Either::Right(data.with_settings(settings)),
                None => Either::Right(data),
            }
        }
    };
    ensure_valid_audio_settings(&data, settings)?;
    Ok(data)
}

pub fn ensure_valid_audio_settings(
    _audio: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    _settings: Option<&Settings>,
) -> Result<(), Error> {
    Ok(())
}

pub fn ensure_store_data<T: PartialEq + Eq + Hash + Clone + Into<Key>>(
    key: T,
    value: StaticSoundData,
    settings: Option<Settings>,
    path: &impl AsRef<std::path::Path>,
    store: &mut HashMap<T, StaticSoundData>,
) -> Result<(), Error> {
    let value = match settings {
        Some(settings) => value.with_settings(settings.into()),
        None => value,
    };
    ensure!(
        store.insert(key.clone(), value).is_none(),
        CannotStoreDataSnafu {
            id: Id::InMemory(key.into()),
            path: path.as_ref().display().to_string()
        }
    );
    Ok(())
}

pub fn ensure_store_subtitle<T: PartialEq + Eq + Hash + Clone + Into<Key>>(
    key: T,
    value: DialogLine,
    store: &mut HashMap<T, DialogLine>,
) -> Result<(), Error> {
    ensure!(store.insert(key, value).is_none(), CannotStoreSubtitleSnafu);
    Ok(())
}

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

/// ensure [`Id`] is properly indexed in appropriate bank
#[inline]
pub fn ensure_store_id(id: Id, store: &mut HashSet<Id>) -> Result<(), Error> {
    ensure!(store.insert(id.clone()), CannotStoreAgnosticIdSnafu { id });
    Ok(())
}

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
) -> Result<(), Error>
where
    HashSet<Id>: Conflict<K>,
{
    let data = ensure_valid_audio(&path, m, usage, settings.as_ref())?;
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = match usage {
        Usage::InMemory => Id::InMemory(key.clone().into()),
        Usage::OnDemand => Id::OnDemand(crate::Usage::Static(
            key.clone().into(),
            m.as_ref().join(path.clone()),
        )),
        Usage::Streaming => Id::OnDemand(crate::Usage::Streaming(
            key.clone().into(),
            m.as_ref().join(path.clone()),
        )),
    };
    if usage == Usage::InMemory {
        ensure_store_data(key, data.left().unwrap(), settings, &path, map)?;
    } else if let Some(settings) = settings {
        ensure_store_settings(&key, settings, smap)?;
    }
    ensure_store_id(id, set)?;
    Ok(())
}

pub fn ensure_sfx<'a>(
    k: &'a str,
    v: Sfx,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<UniqueKey, StaticSoundData>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let UsableAudio {
        audio: Audio { file, settings },
        usage,
    } = v.into();
    let c_string = std::ffi::CString::new(k).expect("CString::new failed");
    let cname = CNamePool::add_cstr(&c_string);
    let key = UniqueKey(cname);
    ensure(k, key, file, m, usage, settings, set, map, smap)?;
    Ok(())
}

pub fn ensure_ono<'a>(
    k: &'a str,
    v: Ono,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<GenderKey, StaticSoundData>,
    smap: &'a mut HashMap<GenderKey, Settings>,
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let (usage, genders) = v.into();
    let c_string = std::ffi::CString::new(k).expect("CString::new failed");
    let cname = CNamePool::add_cstr(&c_string);
    let mut key: GenderKey;
    for (gender, Audio { file, settings }) in genders {
        key = GenderKey(cname, gender);
        ensure(k, key, file, m, usage, settings.clone(), set, map, smap)?;
    }
    Ok(())
}

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
    ensure_key_unique(k)?;
    let v: AnyVoice = v.into();
    let c_string = std::ffi::CString::new(k).expect("CString::new failed");
    let cname = CNamePool::add_cstr(&c_string);
    let mut simple_key: LocaleKey;
    let mut complex_key: BothKey;
    match v {
        Either::Left((aud, usage, subs)) => {
            for (locale, Audio { file, settings }) in aud {
                simple_key = LocaleKey(cname, locale);
                if let Some(ref subs) = subs {
                    ensure_store_subtitle::<LocaleKey>(
                        simple_key.clone(),
                        subs.get(&locale).unwrap().clone(),
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
                )?;
            }
        }
        Either::Right((aud, usage, subs)) => {
            for (locale, genders) in aud {
                for (gender, Audio { file, settings }) in genders {
                    complex_key = BothKey(cname, locale, gender);
                    if let Some(ref subs) = subs {
                        ensure_store_subtitle::<BothKey>(
                            complex_key.clone(),
                            subs.get(&locale).unwrap().get(&gender).unwrap().clone(),
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
                    )?;
                }
            }
        }
    }
    Ok(())
}

pub fn ensure_music<'a>(
    k: &'a str,
    v: Music,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let Audio { file, settings } = v.into();
    ensure_valid_audio(&file, m, Usage::Streaming, settings.as_ref())?;
    let c_string = std::ffi::CString::new(k).expect("CString::new failed");
    let cname = CNamePool::add_cstr(&c_string);
    let key = UniqueKey(cname);
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = Id::OnDemand(crate::Usage::Streaming(
        crate::Key::Unique(key.clone()),
        m.as_ref().join(file),
    ));
    if let Some(settings) = settings {
        ensure_store_settings::<UniqueKey>(&key, settings, smap)?;
    }
    ensure_store_id(id, set)?;
    Ok(())
}
