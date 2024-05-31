use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use audioware_sys::interop::{audio::ScnDialogLineType, gender::PlayerGender, locale::Locale};
use either::Either;
use kira::sound::{static_sound::StaticSoundData, streaming::StreamingSoundData, FromFileError};
use red4ext_rs::types::CName;

use crate::{
    bank::{BothKey, GenderKey, Id, LocaleKey, UniqueKey},
    manifest::{
        de::{Base, DialogLine, Usage},
        error::{
            ensure_key_unique, ensure_store_gender_data, ensure_store_id, ensure_store_unique_data,
            ensure_valid_audio,
        },
    },
};

use super::{
    de::{Music, Ono, Sfx, Voice},
    depot::Mod,
    error::{
        ensure_key_no_conflict, ensure_store_both_key, ensure_store_gender_subtitle,
        ensure_store_locale_data, ensure_store_subtitle, Error,
    },
};

impl From<Sfx> for (Usage, PathBuf) {
    fn from(value: Sfx) -> Self {
        match value {
            Sfx::Inline(path) => (Usage::InMemory, path),
            Sfx::Multi {
                props: Base { file: path, usage },
            } => (usage, path),
        }
    }
}

pub fn ensure_sfx<'a>(
    k: &'a str,
    v: Sfx,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<UniqueKey, StaticSoundData>,
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let (usage, path) = v.into();
    let data = ensure_valid_audio(&path, m, usage)?;
    let cname = CName::new_pooled(k);
    let key = UniqueKey(cname);
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = match usage {
        Usage::InMemory => Id::InMemory(crate::bank::Key::Unique(key.clone())),
        Usage::OnDemand => Id::OnDemand(crate::bank::Usage::Static(
            crate::bank::Key::Unique(key.clone()),
            m.as_ref().join(path.clone()),
        )),
        Usage::Streaming => Id::OnDemand(crate::bank::Usage::Streaming(
            crate::bank::Key::Unique(key.clone()),
            m.as_ref().join(path.clone()),
        )),
    };
    if usage == Usage::InMemory {
        ensure_store_unique_data(key, data.left().unwrap(), &path, map)?;
    }
    ensure_store_id(id, set)?;
    Ok(())
}

impl From<Ono> for (Usage, HashMap<PlayerGender, PathBuf>) {
    fn from(value: Ono) -> Self {
        let usage = value.usage.unwrap_or(Usage::InMemory);
        (usage, value.genders)
    }
}

pub fn ensure_ono<'a>(
    k: &'a str,
    v: Ono,
    m: &Mod,
    set: &'a mut HashSet<Id>,
    map: &'a mut HashMap<GenderKey, StaticSoundData>,
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let (usage, genders) = v.into();
    let cname = CName::new_pooled(k);
    let mut key: GenderKey;
    let mut id: Id;
    let mut data: Either<StaticSoundData, StreamingSoundData<FromFileError>>;
    for (gender, path) in genders {
        data = ensure_valid_audio(&path, m, usage)?;
        key = GenderKey(cname.clone(), gender);
        ensure_key_no_conflict(&key, k, set)?;
        id = match usage {
            Usage::InMemory => Id::InMemory(crate::bank::Key::Gender(key.clone())),
            Usage::OnDemand => Id::OnDemand(crate::bank::Usage::Static(
                crate::bank::Key::Gender(key.clone()),
                m.as_ref().join(path.clone()),
            )),
            Usage::Streaming => Id::OnDemand(crate::bank::Usage::Streaming(
                crate::bank::Key::Gender(key.clone()),
                m.as_ref().join(path.clone()),
            )),
        };
        if usage == Usage::InMemory {
            ensure_store_gender_data(key, data.left().unwrap(), &path, map)?;
        }
        ensure_store_id(id, set)?;
    }
    Ok(())
}

/// ultimately a voice is just either file path for each locale
/// with optional gender,
/// optional corresponding dialog lines
/// and audio usage
type AnyVoice = Either<
    (
        HashMap<Locale, PathBuf>,
        Usage,
        Option<HashMap<Locale, DialogLine>>,
    ),
    (
        HashMap<Locale, HashMap<PlayerGender, PathBuf>>,
        Usage,
        Option<HashMap<Locale, HashMap<PlayerGender, DialogLine>>>,
    ),
>;

impl From<Voice> for AnyVoice {
    fn from(value: Voice) -> Self {
        let default_usage = Usage::OnDemand;
        let default_line = ScnDialogLineType::Regular;
        match value {
            Voice::SingleInline { dialogs, usage } => {
                Either::Left((dialogs, usage.unwrap_or(default_usage), None))
            }
            Voice::SingleMulti {
                dialogs,
                usage,
                line,
            } => {
                let mut aud: HashMap<Locale, PathBuf> = HashMap::with_capacity(dialogs.len());
                let mut sub: HashMap<Locale, DialogLine> = HashMap::with_capacity(dialogs.len());
                for (k, v) in dialogs.into_iter() {
                    aud.insert(k, v.file);
                    sub.insert(
                        k,
                        DialogLine {
                            msg: v.subtitle,
                            line: line.unwrap_or(default_line),
                        },
                    );
                }
                Either::Left((aud, usage.unwrap_or(default_usage), Some(sub)))
            }
            Voice::DualInline { dialogs, usage } => {
                Either::Right((dialogs, usage.unwrap_or(default_usage), None))
            }
            Voice::DualMulti {
                dialogs,
                usage,
                line,
            } => {
                let mut aud: HashMap<Locale, HashMap<PlayerGender, PathBuf>> =
                    HashMap::with_capacity(dialogs.len());
                let mut sub: HashMap<Locale, HashMap<PlayerGender, DialogLine>> =
                    HashMap::with_capacity(dialogs.len());
                for (k, v) in dialogs.into_iter() {
                    match v {
                        crate::manifest::de::Dialogs::Different { dialogs } => {
                            let (fem, male) = (
                                dialogs.get(&PlayerGender::Female).unwrap(),
                                dialogs.get(&PlayerGender::Male).unwrap(),
                            );
                            aud.insert(
                                k,
                                HashMap::from([
                                    (PlayerGender::Female, fem.file.clone()),
                                    (PlayerGender::Male, male.file.clone()),
                                ]),
                            );
                            sub.insert(
                                k,
                                HashMap::from([
                                    (
                                        PlayerGender::Female,
                                        DialogLine {
                                            msg: fem.subtitle.clone(),
                                            line: line.unwrap_or(default_line),
                                        },
                                    ),
                                    (
                                        PlayerGender::Male,
                                        DialogLine {
                                            msg: male.subtitle.clone(),
                                            line: line.unwrap_or(default_line),
                                        },
                                    ),
                                ]),
                            );
                        }
                        crate::manifest::de::Dialogs::Shared { paths, subtitle } => {
                            let (fem, male) = (
                                paths.get(&PlayerGender::Female).unwrap(),
                                paths.get(&PlayerGender::Male).unwrap(),
                            );
                            aud.insert(
                                k,
                                HashMap::from([
                                    (PlayerGender::Female, fem.clone()),
                                    (PlayerGender::Male, male.clone()),
                                ]),
                            );
                            let same = DialogLine {
                                msg: subtitle.clone(),
                                line: line.unwrap_or(default_line),
                            };
                            sub.insert(
                                k,
                                HashMap::from([
                                    (PlayerGender::Female, same.clone()),
                                    (PlayerGender::Male, same),
                                ]),
                            );
                        }
                    }
                }
                Either::Right((aud, usage.unwrap_or(default_usage), Some(sub)))
            }
        }
    }
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
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let v: AnyVoice = v.into();
    let cname = CName::new_pooled(k);
    let mut simple_key: LocaleKey;
    let mut complex_key: BothKey;
    let mut id: Id;
    let mut data: Either<StaticSoundData, StreamingSoundData<FromFileError>>;
    match v {
        Either::Left((aud, usage, subs)) => {
            for (locale, path) in aud {
                data = ensure_valid_audio(&path, m, usage)?;
                simple_key = LocaleKey(cname.clone(), locale);
                ensure_key_no_conflict(&simple_key, k, set)?;
                id = match usage {
                    Usage::InMemory => Id::InMemory(crate::bank::Key::Locale(simple_key.clone())),
                    Usage::OnDemand => Id::OnDemand(crate::bank::Usage::Static(
                        crate::bank::Key::Locale(simple_key.clone()),
                        m.as_ref().join(path.clone()),
                    )),
                    Usage::Streaming => Id::OnDemand(crate::bank::Usage::Streaming(
                        crate::bank::Key::Locale(simple_key.clone()),
                        m.as_ref().join(path.clone()),
                    )),
                };
                if usage == Usage::InMemory {
                    ensure_store_locale_data(
                        simple_key.clone(),
                        data.left().unwrap(),
                        &path,
                        simple,
                    )?;
                }
                if let Some(ref subs) = subs {
                    ensure_store_subtitle(
                        simple_key.clone(),
                        subs.get(&locale).unwrap().clone(),
                        simple_subs,
                    )?;
                }
                ensure_store_id(id, set)?;
            }
        }
        Either::Right((aud, usage, subs)) => {
            for (locale, genders) in aud {
                for (gender, path) in genders {
                    data = ensure_valid_audio(&path, m, usage)?;
                    complex_key = BothKey(cname.clone(), locale, gender);
                    ensure_key_no_conflict(&complex_key, k, set)?;
                    id = match usage {
                        Usage::InMemory => {
                            Id::InMemory(crate::bank::Key::Both(complex_key.clone()))
                        }
                        Usage::OnDemand => Id::OnDemand(crate::bank::Usage::Static(
                            crate::bank::Key::Both(complex_key.clone()),
                            m.as_ref().join(path.clone()),
                        )),
                        Usage::Streaming => Id::OnDemand(crate::bank::Usage::Streaming(
                            crate::bank::Key::Both(complex_key.clone()),
                            m.as_ref().join(path.clone()),
                        )),
                    };
                    if usage == Usage::InMemory {
                        ensure_store_both_key(
                            complex_key.clone(),
                            data.left().unwrap(),
                            &path,
                            complex,
                        )?;
                    }
                    if let Some(ref subs) = subs {
                        ensure_store_gender_subtitle(
                            complex_key.clone(),
                            subs.get(&locale).unwrap().get(&gender).unwrap().clone(),
                            complex_subs,
                        )?;
                    }
                    ensure_store_id(id, set)?;
                }
            }
        }
    }
    Ok(())
}

impl From<Music> for PathBuf {
    fn from(value: Music) -> Self {
        value.0
    }
}

pub fn ensure_music<'a>(
    k: &'a str,
    v: Music,
    m: &Mod,
    set: &'a mut HashSet<Id>,
) -> Result<(), Error> {
    ensure_key_unique(k)?;
    let path: PathBuf = v.into();
    ensure_valid_audio(&path, m, Usage::Streaming)?;
    let cname = CName::new_pooled(k);
    let key = UniqueKey(cname);
    ensure_key_no_conflict(&key, k, set)?;
    let id: Id = Id::OnDemand(crate::bank::Usage::Streaming(
        crate::bank::Key::Unique(key.clone()),
        m.as_ref().join(path),
    ));
    ensure_store_id(id, set)?;
    Ok(())
}
