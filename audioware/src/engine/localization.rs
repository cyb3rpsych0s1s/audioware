use std::sync::Mutex;

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use once_cell::sync::OnceCell;
use red4ext_rs::types::CName;

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
        if let Ok(mut guard) = self::voice().try_lock() {
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
        if let Ok(mut guard) = self::subtitles().try_lock() {
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

pub fn maybe_gender() -> anyhow::Result<PlayerGender> {
    if let Ok(guard) = self::gender().try_lock() {
        return Ok(*guard);
    }
    anyhow::bail!("unable to reach player gender");
}

pub fn maybe_voice() -> anyhow::Result<Locale> {
    if let Ok(guard) = self::voice().try_lock() {
        return Ok(*guard);
    }
    anyhow::bail!("unable to reach voice language");
}

pub fn maybe_subtitles() -> anyhow::Result<Locale> {
    if let Ok(guard) = self::subtitles().try_lock() {
        return Ok(*guard);
    }
    anyhow::bail!("unable to reach subtitles language");
}
