use red4ext_rs::types::CName;
use red4ext_rs::types::Ref;

use audioware_types::interop::gender::PlayerGender;
use audioware_types::interop::localization::LocalizationPackage;

use crate::engine::State;

pub fn update_engine_state(state: State) {
    crate::engine::update_state(state);
}

#[allow(unused_variables)]
pub fn update_engine_locale(voice: CName, subtitle: CName) {
    crate::engine::localization::update_locales(voice, subtitle);
}

#[allow(unused_variables)]
pub fn update_engine_gender(gender: PlayerGender) {
    crate::engine::localization::update_gender(gender);
}

#[allow(unused_variables)]
pub fn define_engine_subtitles(package: Ref<LocalizationPackage>) {
    // package.subtitle(key, value_f, value_m) ...
}

pub fn supported_engine_languages() -> Vec<CName> {
    vec![]
}
