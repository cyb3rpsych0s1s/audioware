use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::PathBuf,
};

use either::Either;
use kira::sound::static_sound::StaticSoundData;
use red4ext_rs::types::CName;

use crate::{
    bank::{
        conflict::{Conflict, Conflictual},
        BothKey, GenderKey, Id, Key, LocaleKey, UniqueKey,
    },
    manifest::{
        de::{DialogLine, Usage},
        error::{ensure_key_unique, ensure_store_id, ensure_valid_audio},
    },
};

use super::{
    de::{AnyVoice, Audio, Music, Ono, Settings, Sfx, UsableAudio, Voice},
    depot::Mod,
    error::{
        ensure_key_no_conflict, ensure_store_data, ensure_store_settings, ensure_store_subtitle,
        Error,
    },
};

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
    let data = ensure_valid_audio(&path, m, usage)?;
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = match usage {
        Usage::InMemory => Id::InMemory(key.clone().into()),
        Usage::OnDemand => Id::OnDemand(crate::bank::Usage::Static(
            key.clone().into(),
            m.as_ref().join(path.clone()),
        )),
        Usage::Streaming => Id::OnDemand(crate::bank::Usage::Streaming(
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
    let cname = CName::new_pooled(k);
    let key = UniqueKey(cname.clone());
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
    let cname = CName::new_pooled(k);
    let mut key: GenderKey;
    for (gender, Audio { file, settings }) in genders {
        key = GenderKey(cname.clone(), gender);
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
    let cname = CName::new_pooled(k);
    let mut simple_key: LocaleKey;
    let mut complex_key: BothKey;
    match v {
        Either::Left((aud, usage, subs)) => {
            for (locale, Audio { file, settings }) in aud {
                simple_key = LocaleKey(cname.clone(), locale);
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
                    complex_key = BothKey(cname.clone(), locale, gender);
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
    let (path, settings) = v.into();
    ensure_valid_audio(&path, m, Usage::Streaming)?;
    let cname = CName::new_pooled(k);
    let key = UniqueKey(cname);
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = Id::OnDemand(crate::bank::Usage::Streaming(
        crate::bank::Key::Unique(key.clone()),
        m.as_ref().join(path),
    ));
    if let Some(settings) = settings {
        ensure_store_settings::<UniqueKey>(&key, settings, smap)?;
    }
    ensure_store_id(id, set)?;
    Ok(())
}
