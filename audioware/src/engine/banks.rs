use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
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
        id::{AnyId, Id},
        redmod::{ModName, R6Audioware, REDmod},
        voice::Subtitle,
    },
};

static BANKS: OnceCell<HashMap<ModName, Bank>> = OnceCell::new();

fn ids() -> &'static Mutex<HashSet<Id>> {
    static INSTANCE: OnceCell<Mutex<HashSet<Id>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

macro_rules! maybe_ids {
    () => {
        ids()
            .try_lock()
            .map_err(|_| InternalError::Contention { origin: "ids" })
    };
}

macro_rules! maybe_banks {
    () => {
        BANKS.get().ok_or(Error::from(BankError::Uninitialized))
    };
}

/// return either a fully typed ID, or an error
pub fn typed_id(sound_name: &CName) -> Result<Id, Error> {
    let ids = maybe_ids!()?;
    for id in ids.iter() {
        match id {
            Id::Voice(inner) if inner.as_ref() == sound_name => return Ok(id.clone()),
            Id::Sfx(inner) if inner.as_ref() == sound_name => return Ok(id.clone()),
            _ => continue,
        }
    }
    Err(BankError::NotFound {
        id: sound_name.clone(),
    }
    .into())
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
    let guard = maybe_ids!()?;
    for i in guard.iter() {
        match i {
            Id::Voice(x) if x == &id => return Ok(true),
            Id::Sfx(x) if x == &id => return Ok(true),
            Id::Any(x) if x.as_ref() == &id => {
                return Err(RegistryError::Corrupted { id: x.clone() }.into())
            }
            _ => continue,
        }
    }
    Err(RegistryError::NotFound { id }.into())
}

pub fn exist(ids: &[CName]) -> Result<bool, Error> {
    let guard = maybe_ids!()?;
    for id in ids {
        if !guard.contains(&Id::Any(AnyId::from(id.clone()))) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn exists_event(event: &Ref<Event>) -> Result<bool, Error> {
    Ok(maybe_ids!()?.contains(&Id::Any(AnyId::from(event.sound_name()))))
}

pub fn data(id: &CName) -> Result<StaticSoundData, Error> {
    let gender = engine::localization::maybe_gender()?;
    let language = engine::localization::maybe_voice()?;
    let banks = maybe_banks!()?;
    for bank in banks.values() {
        if let Ok(data) = bank.data_from_any_id(gender, language, id) {
            return Ok(data);
        }
    }
    Err(BankError::NotFound { id: id.clone() }.into())
}

pub fn languages() -> Set<Locale> {
    let mut set: Set<Locale> = Set::new();
    match maybe_banks!() {
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
    match maybe_banks!() {
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
    match maybe_banks!() {
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
