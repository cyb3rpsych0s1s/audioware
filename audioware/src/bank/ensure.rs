use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use audioware_manifest::{DialogLine, Manifest, Mod, Settings, Usage};
use either::Either;
use kira::sound::{
    static_sound::{StaticSoundData, StaticSoundSettings},
    streaming::{StreamingSoundData, StreamingSoundSettings},
    FromFileError,
};
use red4ext_rs::types::CName;
use snafu::ensure;

use crate::bank::{error::validation::*, Id};

use super::{
    conflict::{Conflict, Conflictual},
    Error, Key,
};

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

/// ensure [`Key`](crate::bank::Key) variants do not [`Conflict`].
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
