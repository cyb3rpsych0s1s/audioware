use std::{collections::HashMap, sync::OnceLock};

use lazy_static::lazy_static;

use crate::types::{
    bank::Bank,
    redmod::{ModName, REDmod},
};

lazy_static! {
    static ref BANKS: OnceLock<HashMap<ModName, Bank>> = OnceLock::default();
}

pub(super) fn setup() -> anyhow::Result<()> {
    let redmod = REDmod::try_new()?;
    let mods = redmod.mods();
    let mut banks = HashMap::with_capacity(mods.len());
    for ref m in mods {
        if let Some(mut bank) = Bank::try_from(m).ok() {
            bank.cleanup();
            banks.insert(bank.name().clone(), bank);
        }
    }
    banks.shrink_to_fit();
    if let Err(_) = BANKS.set(banks) {
        red4ext_rs::error!("unable to store banks");
    }
    Ok(())
}
