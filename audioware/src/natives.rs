use audioware_sys::interop::audio::ScnDialogLineType;
use audioware_sys::interop::locale::Locale;
use audioware_sys::interop::quaternion::Quaternion;
use audioware_sys::interop::vector4::Vector4;
use red4ext_rs::macros::redscript_global;
use red4ext_rs::types::CName;
use red4ext_rs::types::EntityId;
use red4ext_rs::types::Ref;

use audioware_sys::interop::gender::PlayerGender;
use audioware_sys::interop::localization::LocalizationPackage;

use crate::engine::effects::Preset;
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

#[redscript_global(name = "Audioware.PropagateSubtitle")]
pub fn propagate_subtitle(
    reaction: CName,
    entity_id: EntityId,
    emitter_name: CName,
    scene_dialog_line_type: ScnDialogLineType,
) -> ();

/// get reaction duration (in seconds) from sound, or return default
pub fn get_reaction_duration(sound: CName) -> f32 {
    let gender = crate::engine::localization::maybe_gender().unwrap_or_default();
    let locale = crate::engine::localization::maybe_voice().unwrap_or_default();
    crate::engine::banks::reaction_duration(sound, gender, locale).unwrap_or(3.0)
}

pub fn register_emitter(id: EntityId) {
    crate::engine::tracks::register_emitter(id);
}
pub fn unregister_emitter(id: EntityId) {
    crate::engine::tracks::unregister_emitter(id);
}

pub fn update_actor_location(id: EntityId, position: Vector4, orientation: Quaternion) {
    crate::engine::update_actor_location(id, position, orientation);
}

pub fn emitters_count() -> i32 {
    crate::engine::tracks::emitters_count()
}

pub fn update_player_reverb(value: f32) -> bool {
    crate::engine::tracks::update_player_reverb(value)
}

pub fn update_player_preset(preset: Preset) -> bool {
    match crate::engine::state::update_player_preset(preset) {
        Err(e) => {
            red4ext_rs::warn!("{e}");
            false
        }
        _ => match crate::engine::tracks::update_player_preset(preset) {
            Err(e) => {
                red4ext_rs::warn!("{e}");
                false
            }
            _ => true,
        },
    }
}

pub fn play_over_the_phone(event_name: CName, emitter_name: CName) {
    crate::engine::play(
        event_name,
        None,
        Some(emitter_name),
        Some(ScnDialogLineType::Holocall),
    );
}
