use red4ext_rs::types::CName;
use red4ext_rs::types::Ref;

use crate::{engine::State, interop::{gender::PlayerGender, localization::LocalizationPackage}};

pub fn update_engine_state(state: State) {
    crate::engine::update_state(state);
}

#[allow(unused_variables)]
pub fn update_engine_locale(voice: CName, subtitle: CName, interface: CName) {}

#[allow(unused_variables)]
pub fn update_engine_gender(gender: PlayerGender) {}

#[allow(unused_variables)]
pub fn define_engine_subtitles(package: Ref<LocalizationPackage>) {
    // package.subtitle(key, value_f, value_m) ...
}

pub fn supported_engine_languages() -> Vec<CName> {
    vec![]
}
