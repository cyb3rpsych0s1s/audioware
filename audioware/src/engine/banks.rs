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

pub fn setup() -> anyhow::Result<()> {
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

pub fn exists(id: CName) -> anyhow::Result<bool> {
    if let Ok(guard) = self::ids().try_lock() {
        for i in guard.iter() {
            match i {
                Id::Voice(x) => {
                    if x == &id {
                        return Ok(true);
                    }
                }
                Id::Any(x) => anyhow::bail!("invalid id in set ({x})"),
            }
        }
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn exist(ids: &[CName]) -> anyhow::Result<bool> {
    if let Ok(guard) = self::ids().try_lock() {
        for id in ids {
            if !guard.contains(&Id::Any(AnyId::from(id.clone()))) {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn exists_event(event: &Ref<Event>) -> anyhow::Result<bool> {
    if let Ok(guard) = self::ids().try_lock() {
        return Ok(guard.contains(&Id::Any(AnyId::from(event.sound_name()))));
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn data(id: &CName) -> anyhow::Result<StaticSoundData> {
    let gender = engine::localization::maybe_gender()?;
    let language = engine::localization::maybe_voice()?;
    if let Some(banks) = BANKS.get() {
        for bank in banks.values() {
            if let Some(data) = bank.data(gender, language, id) {
                return Ok(data);
            }
        }
    }
    anyhow::bail!("unable to retrieve static sound data from sound id");
}

pub fn languages() -> Set<Locale> {
    let mut set: Set<Locale> = Set::new();
    if let Some(banks) = BANKS.get() {
        for locale in Locale::iter() {
            if banks.values().any(|x| x.supports(locale)) {
                set.insert(locale);
            }
        }
    }
    set
}

pub fn subtitles<'a>(locale: Locale) -> Vec<Subtitle<'a>> {
    let mut subtitles: Vec<Subtitle<'_>> = vec![];
    if let Some(banks) = BANKS.get() {
        for bank in banks.values() {
            if let Some(voices) = bank.voices() {
                for subtitle in voices.subtitles(locale) {
                    subtitles.push(subtitle);
                }
            }
        }
    }
    subtitles
}

pub fn reaction_duration(sound: CName, gender: PlayerGender, locale: Locale) -> Option<f32> {
    if let Some(banks) = BANKS.get() {
        for bank in banks.values() {
            if let Some(data) = bank.data(gender, locale, &sound) {
                return Some(data.duration().as_secs_f32());
            }
        }
    }
    None
}
