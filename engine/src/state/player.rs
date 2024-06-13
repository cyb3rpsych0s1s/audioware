//! player state

use std::sync::RwLock;

use audioware_core::error::{Error, InvalidLocaleSnafu};
use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use once_cell::sync::OnceCell;
use red4ext_rs::types::CName;
use snafu::ResultExt;

pub fn gender() -> &'static RwLock<PlayerGender> {
    static INSTANCE: OnceCell<RwLock<PlayerGender>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn spoken_language() -> &'static RwLock<Locale> {
    static INSTANCE: OnceCell<RwLock<Locale>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn written_language() -> &'static RwLock<Locale> {
    static INSTANCE: OnceCell<RwLock<Locale>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn update_gender(gender: PlayerGender) -> Result<(), Error> {
    *self::gender().try_write()? = gender;
    Ok(())
}

pub fn update_locales(spoken: CName, written: CName) -> Result<(), Error> {
    let spoken = Locale::try_from(spoken).context(InvalidLocaleSnafu)?;
    let written = Locale::try_from(written).context(InvalidLocaleSnafu)?;
    *spoken_language().try_write()? = spoken;
    *written_language().try_write()? = written;
    Ok(())
}
