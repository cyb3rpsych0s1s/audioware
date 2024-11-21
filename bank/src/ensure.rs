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
use kira::{
    sound::{
        static_sound::StaticSoundData, streaming::StreamingSoundData, EndPosition, FromFileError,
        PlaybackPosition,
    },
    Volume,
};
use red4ext_rs::types::{CName, CNamePool};
use snafu::ensure;

use crate::{
    error::validation::{self, *},
    Id,
};

use super::{
    conflict::{Conflict, Conflictual},
    BothKey, Error, GenderKey, Key, LocaleKey, UniqueKey,
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

/// Ensure [CName] does not already exist in [game pool](CNamePool).
#[inline]
pub fn ensure_key_unique(
    cname: &str,
    #[cfg(debug_assertions)] hot_reload: bool,
) -> Result<(), Error> {
    #[cfg(debug_assertions)]
    if hot_reload {
        return Ok(());
    }
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

/// Ensure [Manifest] does not contain duplicate keys among
/// [Sfx]
/// [Ono]
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

/// Ensure audio file [Path](std::path::Path) is located inside [Mod] depot.
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
pub fn ensure_valid_audio(
    path: &impl AsRef<std::path::Path>,
    m: &Mod,
    usage: Usage,
    settings: Option<&Settings>,
    captions: Option<&[Caption]>,
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
    ensure_valid_audio_settings(&data, settings, path.as_ref())?;
    if let Some(captions) = captions {
        ensure_valid_jingle_captions(&data, captions)?;
    }
    Ok(data)
}

/// Ensure given settings are valid for audio.
pub fn ensure_valid_audio_settings(
    audio: &Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    settings: Option<&Settings>,
    path: &Path,
) -> Result<(), Error> {
    if let Some(settings) = settings {
        let duration = match audio {
            Either::Left(x) => x.duration(),
            Either::Right(x) => x.duration(),
        }
        .as_secs_f64();
        if let Some(start_position) = settings.start_position.map(|x| x.as_secs_f64()) {
            ensure!(
                start_position < duration,
                InvalidAudioSettingSnafu {
                    which: "start_position",
                    why: format!("greater than audio duration: {}", path.display())
                }
            );
        }
        if let Some(panning) = settings.panning {
            ensure!(
                (0.0..=1.0).contains(&panning),
                InvalidAudioSettingSnafu {
                    which: "panning",
                    why: format!(
                        "must be a value between 0.0 and 1.0 (inclusive): {}",
                        path.display()
                    )
                }
            );
        }
        if let Some(volume) = settings.volume {
            ensure!(
                Volume::Amplitude(volume).as_decibels() <= 85.0,
                InvalidAudioSettingSnafu {
                    which: "volume",
                    why: format!(
                        "audio should not be louder than 85.0 dB: {}",
                        path.display()
                    )
                }
            );
        }
        if let Some(ref region) = settings.region {
            let start: f64 = match (region.starts(), audio) {
                (Some(PlaybackPosition::Seconds(seconds)), _) => seconds,
                (Some(PlaybackPosition::Samples(samples)), Either::Left(data)) => {
                    samples as f64 / data.sample_rate as f64
                }
                // no sample rate method yet
                (Some(PlaybackPosition::Samples(_)), Either::Right(_)) => {
                    return InvalidAudioSettingSnafu {
                        which: "region.starts",
                        why: format!(
                            "samples unit is not supported with streaming sound yet: {}",
                            path.display()
                        ),
                    }
                    .fail()
                    .map_err(Into::<Error>::into)
                }
                // none implicitly means beginning of the audio
                (None, _) => 0.0,
            };
            let end: f64 = match (region.ends(), audio) {
                (Some(EndPosition::Custom(PlaybackPosition::Seconds(x))), _) => x,
                (
                    Some(EndPosition::Custom(PlaybackPosition::Samples(samples))),
                    Either::Left(data),
                ) => samples as f64 / data.sample_rate as f64,
                // no sample rate method yet
                (Some(EndPosition::Custom(PlaybackPosition::Samples(_))), Either::Right(_)) => {
                    return InvalidAudioSettingSnafu {
                        which: "region.ends",
                        why: format!("samples unit is not supported with streaming sound yet: {}", path.display()),
                    }
                    .fail()
                    .map_err(Into::<Error>::into)
                }
                (Some(EndPosition::EndOfAudio), Either::Left(_)) |
                (Some(EndPosition::EndOfAudio), Either::Right(_)) |
                // none implicitly means end of the audio
                (None, _) => duration,
            };
            ensure!(
                start >= 0.0 && end > 0.0 && start < duration && end <= duration && start < end,
                InvalidAudioSettingSnafu {
                    which: "region",
                    why: format!(
                        "must be within audio duration and starts before it ends: {}",
                        path.display()
                    )
                }
            );
        }
    }
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

/// Ensure subtitle is properly stored.
pub fn ensure_store_subtitle<T: PartialEq + Eq + Hash + Clone + Into<Key>>(
    key: T,
    value: DialogLine,
    store: &mut HashMap<T, DialogLine>,
) -> Result<(), Error> {
    ensure!(store.insert(key, value).is_none(), CannotStoreSubtitleSnafu);
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
    let data = ensure_valid_audio(&path, m, usage, settings.as_ref(), None)?.map_either_with(
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

/// Ensure [Sfx] guarantees are upheld.
pub fn ensure_sfx<'a>(
    k: &'a str,
    v: Sfx,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<UniqueKey, StaticSoundData>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
    #[cfg(debug_assertions)] hot_reload: bool,
) -> Result<(), Error> {
    ensure_key_unique(
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    )?;
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

    add_name_to_pool(
        &c_string,
        #[cfg(debug_assertions)]
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    );
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
    #[cfg(debug_assertions)] hot_reload: bool,
) -> Result<(), Error> {
    ensure_key_unique(
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    )?;
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

    add_name_to_pool(
        &c_string,
        #[cfg(debug_assertions)]
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    );
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
    #[cfg(debug_assertions)] hot_reload: bool,
) -> Result<(), Error> {
    ensure_key_unique(
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    )?;
    let v: AnyVoice = v.into();
    let c_string = std::ffi::CString::new(k)?;
    let cname = CName::new(k);
    let mut simple_key: LocaleKey;
    let mut complex_key: BothKey;
    match v {
        Either::Left((aud, usage, subs)) => {
            for (locale, Audio { file, settings }) in aud {
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

    add_name_to_pool(
        &c_string,
        #[cfg(debug_assertions)]
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    );
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
    #[cfg(debug_assertions)] hot_reload: bool,
) -> Result<(), Error> {
    ensure_key_unique(
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    )?;
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
        Source::Sfx,
    )?;

    add_name_to_pool(
        &c_string,
        #[cfg(debug_assertions)]
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    );
    Ok(())
}

#[doc(hidden)]
pub fn ensure_jingles<'a>(
    k: &'a str,
    v: Jingle,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    smap: &'a mut HashMap<UniqueKey, Settings>,
    #[cfg(debug_assertions)] hot_reload: bool,
) -> Result<(), Error> {
    ensure_key_unique(
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    )?;
    let Audio { file, settings } = (&v).into();
    ensure_valid_audio(&file, m, Usage::Streaming, settings.as_ref(), v.captions())?;
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

    add_name_to_pool(
        &c_string,
        #[cfg(debug_assertions)]
        k,
        #[cfg(debug_assertions)]
        hot_reload,
    );
    Ok(())
}

fn add_name_to_pool(
    c_string: &std::ffi::CString,
    #[cfg(debug_assertions)] str: &str,
    #[cfg(debug_assertions)] hot_reload: bool,
) {
    if !hot_reload || CName::new(str).as_str() != str {
        CNamePool::add_cstr(c_string);
    }
}
