#![feature(arbitrary_self_types)]

mod addresses;
mod audio;
mod banks;
mod engine;
mod frame;
mod hook;
mod interop;
mod locale;

use hook::Hook;
use kira::manager::backend::DefaultBackend;
use kira::manager::AudioManager;
use red4ext_rs::plugin::Plugin;
use red4ext_rs::plugin::Version;
use red4ext_rs::prelude::*;

use crate::banks::SoundBanks;

use crate::hook::HookAudioSystemPlay;
use crate::hook::HookAudioSystemStop;
use crate::hook::HookEntAudioEvent;
use crate::hook::HookEntityQueueEvent;
use crate::hook::HookMusicEvent;
use crate::hook::HookVoiceEvent;

#[derive(Default)]
pub struct Audioware(Option<AudioManager<DefaultBackend>>);
impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);
    fn post_register() {
        info!("on attach audioware");
        HookEntAudioEvent::load();
        HookMusicEvent::load();
        HookVoiceEvent::load();
        HookAudioSystemPlay::load();
        HookAudioSystemStop::load();
        HookEntityQueueEvent::load();

        let _ = SoundBanks::initialize();
        let _ = Self::create();
    }
    fn unload() {
        info!("on detach audioware");
        HookEntAudioEvent::unload();
        HookMusicEvent::unload();
        HookVoiceEvent::unload();
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
        HookEntityQueueEvent::unload();

        let _ = Self::destroy();
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
