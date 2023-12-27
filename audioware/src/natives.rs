use red4ext_rs::types::CName;
use red4ext_rs::types::Ref;

use audioware_types::interop::gender::PlayerGender;
use audioware_types::interop::localization::LocalizationPackage;

use crate::engine::State;

pub fn update_engine_state(state: State) {
    crate::engine::update_state(state);
}

pub fn update_engine_locale(voice: CName, subtitle: CName) {
    crate::engine::localization::update_locales(voice, subtitle);
}

pub fn update_engine_gender(gender: PlayerGender) {
    crate::engine::localization::update_gender(gender);
}

pub fn define_engine_subtitles(_package: Ref<LocalizationPackage>) {
    // package.subtitle(key, value_f, value_m) ...
}

pub fn supported_engine_languages() -> Vec<CName> {
    let set = crate::engine::banks::languages();
    let mut supported: Vec<CName> = Vec::with_capacity(set.len());
    for locale in set {
        supported.push(locale.into());
    }
    supported
}
