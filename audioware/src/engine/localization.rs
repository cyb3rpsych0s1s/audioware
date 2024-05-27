use std::sync::Mutex;

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use once_cell::sync::OnceCell;
use red4ext_rs::types::CName;

use crate::types::error::{Error, InternalError};

macro_rules! maybe_gender {
    () => {
        self::gender()
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "player gender",
            })
    };
}
macro_rules! maybe_voice {
    () => {
        self::voice()
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "player voice locale",
            })
    };
}
macro_rules! maybe_subtitles {
    () => {
        self::subtitles()
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "player written locale",
            })
    };
}

fn gender() -> &'static Mutex<PlayerGender> {
    static INSTANCE: OnceCell<Mutex<PlayerGender>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn voice() -> &'static Mutex<Locale> {
    static INSTANCE: OnceCell<Mutex<Locale>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn subtitles() -> &'static Mutex<Locale> {
    static INSTANCE: OnceCell<Mutex<Locale>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn update_gender(gender: PlayerGender) {
    if let Ok(mut guard) = self::gender().try_lock() {
        *guard = gender;
    } else {
        red4ext_rs::error!("unable to reach player gender");
    }
}

pub fn update_locales(voice: CName, subtitle: CName) {
    if let Ok(voice) = Locale::try_from(voice.clone()) {
        match maybe_voice!() {
            Ok(mut guard) if *guard != voice => {
                *guard = voice;
            }
            Err(e) => {
                red4ext_rs::error!("{e}");
            }
            _ => {}
        }
    } else {
        red4ext_rs::error!(
            "unsupported voice language ({})",
            red4ext_rs::ffi::resolve_cname(&voice)
        );
    }
    if let Ok(subtitle) = Locale::try_from(subtitle.clone()) {
        match maybe_subtitles!() {
            Ok(mut guard) if *guard != subtitle => {
                *guard = subtitle;
            }
            Err(e) => {
                red4ext_rs::error!("{e}");
            }
            _ => {}
        }
    } else {
        red4ext_rs::error!(
            "unsupported subtitles language ({})",
            red4ext_rs::ffi::resolve_cname(&subtitle)
        );
    }
}

pub fn maybe_gender() -> Result<PlayerGender, Error> {
    Ok(*maybe_gender!()?)
}

pub fn maybe_voice() -> Result<Locale, Error> {
    Ok(*maybe_voice!()?)
}

pub fn maybe_subtitles() -> Result<Locale, Error> {
    Ok(*maybe_subtitles!()?)
}
