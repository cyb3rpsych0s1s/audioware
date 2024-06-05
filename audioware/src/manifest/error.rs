use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use either::Either;
use kira::sound::{static_sound::StaticSoundData, streaming::StreamingSoundData, FromFileError};
use red4ext_rs::types::CName;
use snafu::{ensure, Snafu};

use crate::bank::{
    conflict::{Conflict, Conflictual},
    Id, Key,
};

use super::{
    de::{DialogLine, Manifest, Settings, Usage},
    depot::Mod,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("unable to read binary location"), visibility(pub(crate)))]
    BinaryLocation { source: std::io::Error },
    #[snafu(
        display("unable to locate parent folder (expected '{folder}')"),
        visibility(pub(crate))
    )]
    NoFolder { folder: &'static str },
    #[snafu(display("cannot read folder: {depot}"), visibility(pub(crate)))]
    CannotReadDepot { depot: String },
    #[snafu(display("cannot read file: {manifest}"), visibility(pub(crate)))]
    CannotReadManifest {
        manifest: String,
        source: std::io::Error,
    },
    #[snafu(
        display("cannot parse file: {manifest}\n{source}"),
        visibility(pub(crate))
    )]
    CannotParseManifest {
        manifest: String,
        source: serde_yaml::Error,
    },
    #[snafu(display("CName already exists: {cname}"), visibility(pub(crate)))]
    NonUniqueKey { cname: String },
    #[snafu(
        display("CName conflicts with existing id: {cname}"),
        visibility(pub(crate))
    )]
    ConflictingKey { cname: String },
    #[snafu(display("cannot load audio: {path}"), visibility(pub(crate)))]
    InvalidAudio {
        path: String,
        source: kira::sound::FromFileError,
    },
    #[snafu(display("cannot store data: {id}"), visibility(pub(crate)))]
    CannotStoreData { id: Id, path: String },
    #[snafu(display("cannot store subtitle"), visibility(pub(crate)))]
    CannotStoreSubtitle,
    #[snafu(display("cannot store audio settings"), visibility(pub(crate)))]
    CannotStoreSettings,
    #[snafu(display("cannot store id: {id}"), visibility(pub(crate)))]
    CannotStoreAgnosticId { id: Id },
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
) -> Result<Either<StaticSoundData, StreamingSoundData<FromFileError>>, Error> {
    use snafu::ResultExt;
    let filepath = m.as_ref().join(path.as_ref());
    match usage {
        Usage::OnDemand | Usage::InMemory => Ok(StaticSoundData::from_file(filepath)
            .map(Either::Left)
            .context(InvalidAudioSnafu {
                path: path.as_ref().display().to_string(),
            })?),
        Usage::Streaming => Ok(StreamingSoundData::from_file(filepath)
            .map(Either::Right)
            .context(InvalidAudioSnafu {
                path: path.as_ref().display().to_string(),
            })?),
    }
}

pub fn ensure_valid_audio_settings<T>(
    audio: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    settings: Option<Settings>,
) -> Result<(), Error> {
    todo!()
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
