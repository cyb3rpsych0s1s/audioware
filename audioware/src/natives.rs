use audioware_types::interop::locale::Locale;
use red4ext_rs::types::CName;
use red4ext_rs::types::Ref;

use audioware_types::interop::gender::PlayerGender;
use audioware_types::interop::localization::LocalizationPackage;

use crate::engine::State;
use crate::types::voice::Subtitle;

pub fn update_engine_state(state: State) {
    crate::engine::update_state(state);
}

pub fn update_engine_locale(voice: CName, subtitle: CName) {
    crate::engine::localization::update_locales(voice, subtitle);
}

pub fn update_engine_gender(gender: PlayerGender) {
    crate::engine::localization::update_gender(gender);
}

pub fn define_engine_subtitles(package: Ref<LocalizationPackage>) {
    let locale = package.subtitle_language();
    if let Ok(locale) = Locale::try_from(locale) {
        let subtitles = crate::engine::banks::subtitles(locale);
        for Subtitle { key, female, male } in subtitles {
            package.subtitle(
                red4ext_rs::ffi::resolve_cname(key).into(),
                female.into(),
                male.into(),
            );
        }
    }
}

pub fn supported_engine_languages() -> Vec<CName> {
    let set = crate::engine::banks::languages();
    let mut supported: Vec<CName> = Vec::with_capacity(set.len());
    for locale in set {
        supported.push(locale.into());
    }
    supported
}
