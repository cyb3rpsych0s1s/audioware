use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use fixed_map::Map;

use crate::types::{
    bank::Bank,
    voice::{AudioSubtitle, Voice, Voices},
};

pub trait Supports {
    fn supports(&self, locale: Locale) -> bool;
}

pub trait SupportsBy<T> {
    fn supports_by(&self, locale: Locale, by: T) -> bool;
}

impl Supports for Map<Locale, AudioSubtitle> {
    fn supports(&self, locale: Locale) -> bool {
        self.contains_key(locale)
    }
}

impl SupportsBy<PlayerGender> for Voice {
    fn supports_by(&self, locale: Locale, gender: PlayerGender) -> bool {
        match gender {
            PlayerGender::Female => self.female.supports(locale),
            PlayerGender::Male => self.male.supports(locale),
        }
    }
}

impl SupportsBy<PlayerGender> for Voices {
    fn supports_by(&self, locale: Locale, gender: PlayerGender) -> bool {
        self.voices.values().any(|x| x.supports_by(locale, gender))
    }
}

impl Supports for Bank {
    fn supports(&self, locale: Locale) -> bool {
        self.voices().supports_by(locale, PlayerGender::Female)
            || self.voices().supports_by(locale, PlayerGender::Male)
    }
}
