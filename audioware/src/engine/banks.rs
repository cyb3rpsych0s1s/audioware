use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, OnceLock},
};

use audioware_sys::interop::{event::Event, gender::PlayerGender, locale::Locale};
use fixed_map::Set;
use kira::sound::static_sound::StaticSoundData;
use lazy_static::lazy_static;
use red4ext_rs::types::{CName, Ref};
use strum::IntoEnumIterator;

use crate::{
    engine,
    language::Supports,
    types::{
        bank::Bank,
        id::VoiceId,
        redmod::{ModName, REDmod},
        voice::Subtitle,
    },
};

lazy_static! {
    static ref BANKS: OnceLock<HashMap<ModName, Bank>> = OnceLock::default();
    static ref IDS: Arc<Mutex<HashSet<VoiceId>>> = Arc::new(Mutex::new(HashSet::new()));
}

pub fn setup() -> anyhow::Result<()> {
    let redmod = REDmod::try_new()?;
    let mods = redmod.mods();
    let mut banks = HashMap::with_capacity(mods.len());
    for ref m in mods {
        if let Ok(mut bank) = Bank::try_from(m) {
            bank.retain_valid_audio();
            bank.retain_unique_ids(&IDS);
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
    if let Ok(guard) = IDS.clone().try_lock() {
        return Ok(guard.contains(&id.into()));
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn exist(ids: &[CName]) -> anyhow::Result<bool> {
    if let Ok(guard) = IDS.clone().try_lock() {
        for id in ids {
            if !guard.contains(&VoiceId::from(id.clone())) {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn exists_event(event: &Ref<Event>) -> anyhow::Result<bool> {
    if let Ok(guard) = IDS.clone().try_lock() {
        return Ok(guard.contains(&event.sound_name().into()));
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn data(id: &CName) -> anyhow::Result<StaticSoundData> {
    let gender = engine::localization::gender()?;
    let language = engine::localization::voice()?;
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
            for subtitle in bank.voices().subtitles(locale) {
                subtitles.push(subtitle);
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
