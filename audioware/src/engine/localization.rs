use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use lazy_static::lazy_static;
use red4ext_rs::types::CName;

lazy_static! {
    static ref GENDER: Arc<Mutex<PlayerGender>> = Arc::new(Mutex::new(PlayerGender::default()));
    static ref VOICE: Arc<Mutex<Locale>> = Arc::new(Mutex::new(Locale::default()));
    static ref SUBTITLES: Arc<Mutex<Locale>> = Arc::new(Mutex::new(Locale::default()));
}

pub fn update_gender(gender: PlayerGender) {
    if let Ok(mut guard) = GENDER.clone().borrow_mut().try_lock() {
        *guard = gender;
    } else {
        red4ext_rs::error!("unable to reach player gender");
    }
}

pub fn update_locales(voice: CName, subtitle: CName) {
    if let Ok(voice) = Locale::try_from(voice.clone()) {
        if let Ok(mut guard) = VOICE.clone().borrow_mut().try_lock() {
            if *guard != voice {
                *guard = voice;
            }
        } else {
            red4ext_rs::error!("unable to reach voice language");
        }
    } else {
        red4ext_rs::error!(
            "unsupported voice language ({})",
            red4ext_rs::ffi::resolve_cname(&voice)
        );
    }
    if let Ok(subtitle) = Locale::try_from(subtitle.clone()) {
        if let Ok(mut guard) = SUBTITLES.clone().borrow_mut().try_lock() {
            if *guard != subtitle {
                *guard = subtitle;
            }
        } else {
            red4ext_rs::error!("unable to reach subtitles language");
        }
    } else {
        red4ext_rs::error!(
            "unsupported subtitles language ({})",
            red4ext_rs::ffi::resolve_cname(&subtitle)
        );
    }
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
