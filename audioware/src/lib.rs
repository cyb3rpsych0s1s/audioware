#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../README.md")]

include!(concat!(env!("OUT_DIR"), "/version.rs"));

use std::{
    ops::Deref,
    sync::{atomic::AtomicU8, Arc, LazyLock, OnceLock},
};

use audioware_bank::{Banks, Initialization};
use kira::manager::AudioManager;
use manager::Manager;
use parking_lot::RwLock;
use red4ext_rs::{wcstr, Plugin, SemVer, U16CStr};

mod manager;
mod queue;

static GAME_STATE: AtomicU8 = AtomicU8::new(GameState::Uninitialized as u8);
static PLUGIN_STATE: LazyLock<RwLock<PluginState>> =
    LazyLock::new(|| PluginState::Uninitialized.into());

/// Audio [Plugin] for Cyberpunk 2077.
pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    fn on_init(_env: &red4ext_rs::SdkEnv) {
        load_banks();
    }
}

fn load_banks() {
    *PLUGIN_STATE.write() = PluginState::Ready {
        banks: Banks,
        manager: Manager,
    };
}
fn load_queue() {
    match PLUGIN_STATE.read().deref() {
        PluginState::Uninitialized | PluginState::Reloading => return,
        PluginState::Ready { manager, .. } => {
            manager.stop();
            *PLUGIN_STATE.write() = PluginState::Reloading;
        }
        PluginState::Loaded { banks } => todo!(),
    }
}
fn reload() {
    // match PLUGIN_STATE.load(std::sync::atomic::Ordering::Acquire) {
    //     x if x == PluginState::Ready as u8 || x == PluginState::InGame as u8 => {
    //         load_banks();
    //     },
    //     _ => return,
    // }
}

enum PluginState {
    Uninitialized,
    Loaded { banks: Banks },
    Ready { banks: Banks, manager: Manager },
    Reloading,
}

// #[repr(u8)]
// enum PluginState {
//     Uninitialized,
//     Loading,
//     Loaded,
//     Initializing,
//     Ready,
//     InGame,
//     Unloaded,
//     Failed,
// }

#[repr(u8)]
enum GameState {
    Uninitialized,
    MainMenu,
    Attached,
    InGame,
    Detached,
    Exit,
}
