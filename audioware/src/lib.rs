#![feature(arbitrary_self_types)]

//! # Detouring natives
//!
//! This crate provides (and uses itself) both traits and derive macros to detour native functions from the game.
//!
//! automatically derive [`audioware_mem::NativeFunc`] for a given struct
//! which already implements [`audioware_mem::Detour`].
//!
//! ## Examples
//!
//! Here's an example on how to detour [AudioSystem::Play](https://jac3km4.github.io/cyberdoc/#33326)
//! whose signature is:
//!
//! ```swift
//! public native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName) -> Void
//! ```
//!
//! Here's how:
//!
//! ```
//! # use audioware_macros::NativeFunc;
//! # use red4ext_rs::types::{CName, EntityId};
//!
//! #[derive(NativeFunc)]
//! #[hook(
//!     // memory offset
//!     offset = 0x975FE4,
//!     // function input parameters
//!     inputs = "(CName, EntityId, CName)",
//!     // control wheter to allow detouring on each call
//!     allow = "allow",
//!     // custom detouring logic
//!     detour = "detour"
//! )]
//! pub struct AudioSystemPlay;
//! # #[allow(unused_variables)]
//! fn detour(params: (CName, EntityId, CName)) {}
//! # #[allow(unused_variables)]
//! fn allow(params: &(CName, EntityId, CName)) -> bool { false }
//! ```

use audioware_mem::Hook;
use hook::{HookAudioSystemPlay, HookAudioSystemStop, HookAudioSystemSwitch};
use red4ext_rs::plugin::Version;
use red4ext_rs::register_function;
use red4ext_rs::types::CName;
use red4ext_rs::{define_trait_plugin, plugin::Plugin};

mod addresses;
pub mod engine;
mod hook;
mod language;
pub mod natives;
mod types;

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl IsValid for CName {
    fn is_valid(&self) -> bool {
        let str = red4ext_rs::ffi::resolve_cname(self);
        !str.is_empty() && str != "None"
    }
}

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 8, 11);

    fn register() {
        red4ext_rs::info!("on register audioware");
        if let Err(e) = engine::setup() {
            red4ext_rs::error!("unable to setup ({e})");
        }
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
        register_function!(
            "Audioware.GetReactionDuration",
            crate::natives::get_reaction_duration
        );
        register_function!(
            "Audioware.RegisterEmitter",
            crate::natives::register_emitter
        );
        register_function!(
            "Audioware.UnregisterEmitter",
            crate::natives::unregister_emitter
        );
        register_function!(
            "Audioware.UpdateActorLocation",
            crate::natives::update_actor_location
        );
        register_function!("Audioware.EmittersCount", crate::natives::emitters_count);
        register_function!(
            "Audioware.UpdatePlayerReverb",
            crate::natives::update_player_reverb
        );
        register_function!(
            "Audioware.UpdatePlayerPreset",
            crate::natives::update_player_preset
        );
        register_function!(
            "Audioware.PlayOverThePhone",
            crate::natives::play_over_the_phone
        );
    }

    fn post_register() {
        red4ext_rs::info!("on post register audioware");
        HookAudioSystemPlay::load();
        HookAudioSystemStop::load();
        HookAudioSystemSwitch::load();
    }

    fn unload() {
        red4ext_rs::info!("on unload audioware");
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
        HookAudioSystemSwitch::unload();
    }

    fn is_version_independent() -> bool {
        false
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
