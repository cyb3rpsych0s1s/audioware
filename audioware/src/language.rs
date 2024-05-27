use std::collections::HashMap;

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use fixed_map::Map;

use crate::types::{
    bank::Bank,
    id::VoiceId,
    voice::{AudioSubtitle, DualVoice, Voice},
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
        match self {
            Voice::Dual(DualVoice { female, male }) => match gender {
                PlayerGender::Female => female.supports(locale),
                PlayerGender::Male => male.supports(locale),
            },
            Voice::Single(voice) => voice.supports(locale),
        }
    }
}

impl<'a> SupportsBy<PlayerGender> for &'a HashMap<VoiceId, Voice> {
    fn supports_by(&self, locale: Locale, gender: PlayerGender) -> bool {
        self.values().any(|x| x.supports_by(locale, gender))
    }
}

impl Supports for Bank {
    fn supports(&self, locale: Locale) -> bool {
        if let Some(voices) = self.voices() {
            return voices.supports_by(locale, PlayerGender::Female)
                || voices.supports_by(locale, PlayerGender::Male);
        }
        false
    }
}
