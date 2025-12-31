use std::sync::atomic::{AtomicU8, AtomicU32};

use audioware_manifest::{Locale, LocaleExt};
use red4ext_rs::types::EntityId;

use crate::{PlayerPuppet, resolve_any_entity, utils::warns};

use super::scene::AsEntityExt;

const NO_PLAYER_GENDER: u8 = 2;
const ENGLISH: u32 = LocaleExt::English as u32;

static SPOKEN_LOCALE: SpokenLocale = SpokenLocale(AtomicU32::new(ENGLISH));
static WRITTEN_LOCALE: WrittenLocale = WrittenLocale(AtomicU32::new(ENGLISH));
static PLAYER_GENDER: PlayerGender = PlayerGender(AtomicU8::new(NO_PLAYER_GENDER));

pub struct SpokenLocale(AtomicU32);

pub struct WrittenLocale(AtomicU32);

pub struct PlayerGender(AtomicU8);

impl SpokenLocale {
    pub fn set(value: Locale) {
        SPOKEN_LOCALE
            .0
            .store(value.into(), std::sync::atomic::Ordering::Release);
    }
    pub fn get() -> audioware_manifest::SpokenLocale {
        match SPOKEN_LOCALE
            .0
            .load(std::sync::atomic::Ordering::Acquire)
            .try_into()
        {
            Ok(x) => x,
            Err(e) => {
                warns!("invalid spoken locale in state: {}", e);
                Locale::English.into()
            }
        }
    }
}

impl WrittenLocale {
    pub fn set(value: Locale) {
        WRITTEN_LOCALE
            .0
            .store(value.into(), std::sync::atomic::Ordering::Release);
    }
    pub fn get() -> audioware_manifest::WrittenLocale {
        match WRITTEN_LOCALE
            .0
            .load(std::sync::atomic::Ordering::Acquire)
            .try_into()
        {
            Ok(x) => x,
            Err(e) => {
                warns!("invalid written locale in state: {}", e);
                Locale::English.into()
            }
        }
    }
}

impl PlayerGender {
    pub fn set(gender: audioware_manifest::PlayerGender) {
        PLAYER_GENDER
            .0
            .store(gender.into(), std::sync::atomic::Ordering::Release);
    }
    pub fn unset() {
        PLAYER_GENDER
            .0
            .store(NO_PLAYER_GENDER, std::sync::atomic::Ordering::Release);
    }
    pub fn get() -> Option<audioware_manifest::PlayerGender> {
        match PLAYER_GENDER.0.load(std::sync::atomic::Ordering::Acquire) {
            0 => Some(audioware_manifest::PlayerGender::Female),
            1 => Some(audioware_manifest::PlayerGender::Male),
            _ => None,
        }
    }
}

pub trait ToGender {
    fn to_gender(&self) -> Option<audioware_manifest::PlayerGender>;
}

impl ToGender for EntityId {
    fn to_gender(&self) -> Option<audioware_manifest::PlayerGender> {
        let entity = resolve_any_entity(*self);
        if entity.is_null() {
            return None;
        }
        if entity.is_a::<PlayerPuppet>() {
            return self::PlayerGender::get();
        }
        entity.get_gender()
    }
}
