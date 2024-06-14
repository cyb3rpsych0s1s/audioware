//! player state

use std::sync::RwLock;

use audioware_core::{Error, InvalidLocaleSnafu, SpokenLocale, WrittenLocale};
use audioware_sys::interop::gender::PlayerGender;
use once_cell::sync::OnceCell;
use red4ext_rs::types::CName;
use snafu::ResultExt;

pub fn gender() -> &'static RwLock<PlayerGender> {
    static INSTANCE: OnceCell<RwLock<PlayerGender>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn spoken_language() -> &'static RwLock<SpokenLocale> {
    static INSTANCE: OnceCell<RwLock<SpokenLocale>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn written_language() -> &'static RwLock<WrittenLocale> {
    static INSTANCE: OnceCell<RwLock<WrittenLocale>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn update_gender(gender: PlayerGender) -> Result<(), Error> {
    *self::gender().try_write()? = gender;
    Ok(())
}

pub fn update_locales(spoken: CName, written: CName) -> Result<(), Error> {
    let spoken = SpokenLocale::try_from(spoken).context(InvalidLocaleSnafu)?;
    let written = WrittenLocale::try_from(written).context(InvalidLocaleSnafu)?;
    *spoken_language().try_write()? = spoken;
    *written_language().try_write()? = written;
    Ok(())
}
