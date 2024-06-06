#![feature(arbitrary_self_types)]

use audioware_mem::Hook;
#[allow(unused_imports)]
use audioware_mem::Intercept;

use bank::Banks;
use engine::Engine;
use hook::*;
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
        match Engine::setup() {
            Ok(()) => {
                red4ext_rs::info!("engine successfully initialized");
            }
            Err(e) => {
                red4ext_rs::error!("{e}");
            }
        };
        match Banks::setup() {
            Ok(report) => {
                red4ext_rs::info!("banks successfully initialized:\n{report}");
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
        register_function!("Audioware.SmoothStop", crate::natives::smooth_stop);
    }

    fn post_register() {
        red4ext_rs::info!("on post register audioware");
        HookAudioSystemPlay::load();
        HookAudioSystemStop::load();
        HookAudioSystemGlobalParameter::load();
        HookAudioSystemParameter::load();
        #[cfg(debug_assertions)]
        {
            use audioware_mem::Intercept;
            hook::HookgameaudioeventsMusicEvent::load();
            hook::HookgameaudioeventsVoiceEvent::load();
            hook::HookgameaudioeventsVoicePlayedEvent::load();
            // hook::HookgameaudioeventsDialogLine::load();
            // hook::HookgameaudioeventsDialogLineEnd::load();
            // hook::HookgameaudioeventsStopDialogLine::load();
            // hook::HookgameaudioeventsSound1::load();
            // hook::HookgameaudioeventsSound2::load();
            // hook::HookgameaudioeventsSound3::load();
            // hook::HookgameaudioeventsSound4::load();
            // hook::HookgameaudioeventsSound5::load();
        }
    }

    fn unload() {
        red4ext_rs::info!("on unload audioware");
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
        HookAudioSystemGlobalParameter::unload();
        HookAudioSystemParameter::unload();
        #[cfg(debug_assertions)]
        {
            use audioware_mem::Intercept;
            hook::HookgameaudioeventsMusicEvent::unload();
            hook::HookgameaudioeventsVoiceEvent::unload();
            hook::HookgameaudioeventsVoicePlayedEvent::unload();
            // hook::HookgameaudioeventsDialogLine::unload();
            // hook::HookgameaudioeventsDialogLineEnd::unload();
            // hook::HookgameaudioeventsStopDialogLine::unload();
            // hook::HookgameaudioeventsSound1::unload();
            // hook::HookgameaudioeventsSound2::unload();
            // hook::HookgameaudioeventsSound3::unload();
            // hook::HookgameaudioeventsSound4::unload();
            // hook::HookgameaudioeventsSound5::unload();
        }
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
