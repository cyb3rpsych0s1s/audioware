use std::sync::{Arc, Mutex};

use audioware_types::interop::{gender::PlayerGender, locale::Locale};
use lazy_static::lazy_static;

lazy_static! {
    static ref GENDER: Arc<Mutex<PlayerGender>> = Arc::new(Mutex::new(PlayerGender::default()));
    static ref VOICE: Arc<Mutex<Locale>> = Arc::new(Mutex::new(Locale::default()));
    static ref SUBTITLES: Arc<Mutex<Locale>> = Arc::new(Mutex::new(Locale::default()));
}

pub fn gender() -> anyhow::Result<PlayerGender> {
    if let Ok(guard) = GENDER.clone().try_lock() {
        return Ok(*guard);
    }
    anyhow::bail!("unable to reach player gender");
}

pub fn voice() -> anyhow::Result<Locale> {
    if let Ok(guard) = VOICE.clone().try_lock() {
        return Ok(*guard);
    }
    anyhow::bail!("unable to reach voice language");
}

pub fn subtitles() -> anyhow::Result<Locale> {
    if let Ok(guard) = SUBTITLES.clone().try_lock() {
        return Ok(*guard);
    }
    anyhow::bail!("unable to reach subtitles language");
}
