use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, OnceLock},
};

use kira::sound::static_sound::StaticSoundData;
use lazy_static::lazy_static;
use red4ext_rs::types::CName;

use crate::{
    engine,
    types::{
        bank::Bank,
        redmod::{ModName, REDmod},
    },
};

use super::SoundId;

lazy_static! {
    static ref BANKS: OnceLock<HashMap<ModName, Bank>> = OnceLock::default();
    static ref IDS: Arc<Mutex<HashSet<SoundId>>> = Arc::new(Mutex::new(HashSet::new()));
}

pub(super) fn setup() -> anyhow::Result<()> {
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
    if let Err(_) = BANKS.set(banks) {
        red4ext_rs::error!("unable to store banks");
    }
    Ok(())
}

pub(super) fn exists(id: CName) -> anyhow::Result<bool> {
    if let Ok(guard) = IDS.clone().try_lock() {
        return Ok(guard.contains(&id.into()));
    }
    anyhow::bail!("unable to reach sound ids");
}

pub fn data(id: SoundId) -> anyhow::Result<StaticSoundData> {
    let gender = engine::localization::gender()?;
    let language = engine::localization::voice()?;
    if let Some(banks) = BANKS.get() {
        for bank in banks.values() {
            if let Some(data) = bank.data(gender, language, id.clone()) {
                return Ok(data);
            }
        }
    }
    anyhow::bail!("unable to retrieve static sound data from sound id");
}
