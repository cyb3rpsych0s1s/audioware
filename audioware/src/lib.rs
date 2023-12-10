#![feature(arbitrary_self_types)]

use red4ext_rs::plugin::Version;
use red4ext_rs::register_function;
use red4ext_rs::{define_trait_plugin, plugin::Plugin};

mod engine;
mod interop;
pub mod natives;

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);

    fn register() {
        engine::setup();
        register_function!(
            "Audioware.UpdateEngineState",
            crate::natives::update_engine_state
        );
        register_function!(
            "Audioware.UpdateEngineLocale",
            crate::natives::update_engine_locale
        );
        register_function!(
            "Audioware.UpdateEngineGender",
            crate::natives::update_engine_gender
        );
        register_function!(
            "Audioware.DefineEngineSubtitles",
            crate::natives::define_engine_subtitles
        );
        register_function!(
            "Audioware.SupportedEngineLanguages",
            crate::natives::supported_engine_languages
        );
    }

    fn post_register() {}

    fn unload() {}
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
