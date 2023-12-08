#![feature(arbitrary_self_types)]

mod addresses;
mod audio;
mod banks;
mod engine;
mod frame;
mod game;
mod gender;
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
use crate::hook::HookIComponentQueueEntityEvent;
use crate::hook::HookMusicEvent;
use crate::hook::HookVoiceEvent;

#[derive(Default)]
pub struct Audioware(Option<AudioManager<DefaultBackend>>);
impl Audioware {
    pub(crate) fn unload(&mut self) {
        let _ = self.0.take();
    }
}
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
        HookIComponentQueueEntityEvent::load();

        let _ = SoundBanks::initialize();
        if let Err(e) = Self::setup() {
            red4ext_rs::error!("{e}");
        }
    }
    fn unload() {
        info!("on detach audioware");
        HookEntAudioEvent::unload();
        HookMusicEvent::unload();
        HookVoiceEvent::unload();
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
        HookEntityQueueEvent::unload();
        HookIComponentQueueEntityEvent::unload();

        if let Err(e) = Self::teardown() {
            red4ext_rs::error!("{e}");
        }
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
