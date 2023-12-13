#![feature(arbitrary_self_types)]

use hook::{
    Hook, HookAudioSystemPlay, HookAudioSystemStop, HookEntAudioEvent, HookEntityQueueEvent,
    HookIComponentQueueEntityEvent,
};
use red4ext_rs::plugin::Version;
use red4ext_rs::register_function;
use red4ext_rs::types::CName;
use red4ext_rs::{define_trait_plugin, plugin::Plugin};

mod addresses;
pub(crate) mod engine;
mod frame;
mod hook;
mod interop;
pub mod natives;
mod types;

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl IsValid for CName {
    fn is_valid(&self) -> bool {
        !red4ext_rs::ffi::resolve_cname(self).is_empty()
            && red4ext_rs::ffi::resolve_cname(self) != "None"
    }
}

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);

    fn register() {
        let _ = engine::setup();
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

    fn post_register() {
        HookAudioSystemPlay::load();
        HookAudioSystemStop::load();
        HookEntAudioEvent::load();
        HookEntityQueueEvent::load();
        HookIComponentQueueEntityEvent::load();
    }

    fn unload() {
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
        HookEntAudioEvent::unload();
        HookEntityQueueEvent::unload();
        HookIComponentQueueEntityEvent::unload();
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
