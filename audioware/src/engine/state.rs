use std::sync::atomic::{AtomicU32, AtomicU8};

use red4ext_rs::types::{CName, EntityId, GameInstance};

use crate::{error::Error, AsGameInstance, PlayerPuppet};

use super::scene::AsEntityExt;

const NO_PLAYER_GENDER: u8 = 2;
const ENGLISH: u32 = audioware_manifest::Locale::English as u32;

static SPOKEN_LOCALE: SpokenLocale = SpokenLocale(AtomicU32::new(ENGLISH));
static WRITTEN_LOCALE: WrittenLocale = WrittenLocale(AtomicU32::new(ENGLISH));
static PLAYER_GENDER: PlayerGender = PlayerGender(AtomicU8::new(NO_PLAYER_GENDER));

pub struct SpokenLocale(AtomicU32);

pub struct WrittenLocale(AtomicU32);

pub struct PlayerGender(AtomicU8);

impl SpokenLocale {
    pub fn try_set(value: CName) -> Result<(), Error> {
        SPOKEN_LOCALE.0.store(
            audioware_manifest::Locale::try_from(value)?.into(),
            std::sync::atomic::Ordering::Release,
        );
        Ok(())
    }
    pub fn get() -> audioware_manifest::SpokenLocale {
        SPOKEN_LOCALE
            .0
            .load(std::sync::atomic::Ordering::Acquire)
            .try_into()
            .expect("checked on set")
    }
}

impl WrittenLocale {
    pub fn try_set(value: CName) -> Result<(), Error> {
        WRITTEN_LOCALE.0.store(
            audioware_manifest::Locale::try_from(value)?.into(),
            std::sync::atomic::Ordering::Release,
        );
        Ok(())
    }
    pub fn get() -> audioware_manifest::WrittenLocale {
        WRITTEN_LOCALE
            .0
            .load(std::sync::atomic::Ordering::Acquire)
            .try_into()
            .expect("checked on set")
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
    fn into_gender(&self) -> Option<audioware_manifest::PlayerGender>;
}

impl ToGender for EntityId {
    fn into_gender(&self) -> Option<audioware_manifest::PlayerGender> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, *self);
        if entity.is_null() {
            return None;
        }
        if entity.is_a::<PlayerPuppet>() {
            return self::PlayerGender::get();
        }
        entity.get_gender()
    }
}
