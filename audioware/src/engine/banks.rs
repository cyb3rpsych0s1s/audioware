use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, MutexGuard},
};

use audioware_sys::interop::{event::Event, gender::PlayerGender, locale::Locale};
use fixed_map::Set;
use kira::sound::static_sound::StaticSoundData;
use once_cell::sync::OnceCell;
use red4ext_rs::types::{CName, Ref};
use strum::IntoEnumIterator;

use crate::{
    engine,
    language::Supports,
    types::{
        bank::Bank,
        error::{BankError, Error, InternalError, RegistryError},
        id::Id,
        redmod::{ModName, R6Audioware, REDmod},
        voice::Subtitle,
    },
};

pub(crate) trait ContainsCName {
    fn contains_cname(&self, any: &CName) -> bool {
        self.get_by_cname(any).is_some()
    }
    /// return an optional owned [`Id`] to uphold **non**-[`Clone`] invariant
    fn get_by_cname(&self, any: &CName) -> Option<Id>;
}

impl ContainsCName for HashSet<Id> {
    fn get_by_cname(&self, any: &CName) -> Option<Id> {
        for key in self.iter() {
            match key {
                Id::Voice(id) if id.as_ref() == any => return Some(Id::from(id)),
                Id::Sfx(id) if id.as_ref() == any => return Some(Id::from(id)),
                _ => continue,
            }
        }
        None
    }
}

static BANKS: OnceCell<HashMap<ModName, Bank>> = OnceCell::new();

fn ids() -> &'static Mutex<HashSet<Id>> {
    static INSTANCE: OnceCell<Mutex<HashSet<Id>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

#[inline(always)]
pub(crate) fn maybe_ids<'guard>() -> Result<MutexGuard<'guard, HashSet<Id>>, InternalError> {
    ids()
        .try_lock()
        .map_err(|_| InternalError::Contention { origin: "ids" })
}

#[inline(always)]
pub(crate) fn maybe_banks<'cell>() -> Result<&'cell HashMap<ModName, Bank>, BankError> {
    BANKS.get().ok_or(BankError::Uninitialized)
}

/// return either a fully typed ID, or an error
pub fn typed_id(sound_name: &CName) -> Result<Id, Error> {
    let ids = maybe_ids()?;
    ids.get_by_cname(sound_name).ok_or(
        BankError::NotFound {
            id: sound_name.clone(),
        }
        .into(),
    )
}

pub fn setup() -> Result<(), Error> {
    let mut mods = Vec::with_capacity(10);
    let mut redmod_exists = false;
    if let Ok(redmod) = REDmod::try_new() {
        mods = redmod.mods();
        redmod_exists = true;
    }
    if let Ok(r6audioware) = R6Audioware::try_new() {
        for m in r6audioware.mods().into_iter() {
            if redmod_exists && mods.iter().any(|x| x.same_folder_name(m.as_ref())) {
                red4ext_rs::error!("duplicate folder across 'r6\\audioware' and 'mods' folders, skipping folder in 'r6\\audioware'");
                continue;
            }
            mods.push(m);
        }
    }
    let mut banks = HashMap::with_capacity(mods.len());
    for ref m in mods {
        if let Ok(mut bank) = Bank::try_from(m) {
            bank.retain_valid_audio();
            bank.retain_unique_ids(self::ids());
            banks.insert(bank.name().clone(), bank);
        }
    }
    banks.shrink_to_fit();
    if BANKS.set(banks).is_err() {
        red4ext_rs::error!("unable to store banks");
    }
    Ok(())
}

pub fn exists(id: CName) -> Result<bool, Error> {
    let guard = maybe_ids()?;
    for i in guard.iter() {
        match i {
            Id::Voice(x) if x == &id => return Ok(true),
            Id::Sfx(x) if x == &id => return Ok(true),
            _ => continue,
        }
    }
    Err(RegistryError::NotFound { id }.into())
}

pub fn exist(ids: &[CName]) -> Result<bool, Error> {
    let guard = maybe_ids()?;
    for id in ids {
        if !guard.contains_cname(id) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn exists_event(event: &Ref<Event>) -> Result<bool, Error> {
    Ok(maybe_ids()?.contains_cname(&event.sound_name()))
}

pub fn data(id: &CName) -> Result<StaticSoundData, Error> {
    let gender = *engine::localization::maybe_gender()?;
    let language = *engine::localization::maybe_voice()?;
    let banks = maybe_banks()?;
    for bank in banks.values() {
        match bank.data_from_any_id(gender, language, id) {
            Err(_) => continue,
            ok => return ok,
        }
    }
    Err(BankError::NotFound { id: id.clone() }.into())
}

pub fn languages() -> Set<Locale> {
    let mut set: Set<Locale> = Set::new();
    match maybe_banks() {
        Ok(banks) => {
            for locale in Locale::iter() {
                if banks.values().any(|x| x.supports(locale)) {
                    set.insert(locale);
                }
            }
        }
        Err(e) => {
            red4ext_rs::error!("{e}");
        }
    }
    set
}

pub fn subtitles<'a>(locale: Locale) -> Vec<Subtitle<'a>> {
    let mut subtitles: Vec<Subtitle<'_>> = vec![];
    match maybe_banks() {
        Ok(banks) => {
            for bank in banks.values() {
                if let Some(voices) = bank.voices() {
                    for subtitle in voices.subtitles(locale) {
                        subtitles.push(subtitle);
                    }
                }
            }
        }
        Err(e) => {
            red4ext_rs::error!("{e}");
        }
    }
    subtitles
}

pub fn reaction_duration(sound: CName, gender: PlayerGender, locale: Locale) -> Option<f32> {
    match maybe_banks() {
        Ok(banks) => {
            for bank in banks.values() {
                if let Ok(data) = bank.data_from_any_id(gender, locale, &sound) {
                    return Some(data.duration().as_secs_f32());
                }
            }
        }
        Err(e) => {
            red4ext_rs::error!("{e}");
        }
    }
    None
}
