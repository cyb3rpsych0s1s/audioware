#![feature(arbitrary_self_types)]

use audioware_mem::Hook;
use bank::Banks;
use hook::{HookAudioSystemPlay, HookAudioSystemStop};
use red4ext_rs::{
    define_trait_plugin,
    plugin::{Plugin, Version},
    register_function,
};

mod bank;
mod engine;
mod error;
mod hook;
pub mod manifest;
mod natives;
mod state;
pub mod utils;

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 8, 11);

    fn register() {
        red4ext_rs::info!("on register audioware");
        match Banks::setup() {
            Ok(report) => {
                red4ext_rs::info!("successfully initialized:\n{report}");
            }
            Err(e) => {
                red4ext_rs::error!("{e}");
            }
        };
        register_function!(
            "Audioware.UpdateGameState",
            crate::natives::update_game_state
        );
        register_function!(
            "Audioware.UpdatePlayerLocales",
            crate::natives::update_player_locales
        );
        register_function!(
            "Audioware.UpdatePlayerGender",
            crate::natives::update_player_gender
        );
        register_function!(
            "Audioware.StopEngine",
            crate::natives::audioware_stop_engine
        );
    }

    fn post_register() {
        red4ext_rs::info!("on post register audioware");
        HookAudioSystemPlay::load();
        HookAudioSystemStop::load();
    }

    fn unload() {
        red4ext_rs::info!("on unload audioware");
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
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
