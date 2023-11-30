mod addresses;
mod audio;
mod banks;
mod frame;
mod hook;
mod interop;
mod locale;

// use std::borrow::BorrowMut;

use hook::Hook;
use red4ext_rs::plugin::Plugin;
use red4ext_rs::plugin::Version;
use red4ext_rs::prelude::*;

use crate::hook::HookAudioSystemPlay;
use crate::hook::HookAudioSystemStop;
use crate::hook::HookEntAudioEvent;
use crate::hook::HookMusicEvent;
use crate::hook::HookVoiceEvent;

pub struct Audioware;
impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);
    fn post_register() {
        info!("on attach audioware");
        HookEntAudioEvent::load();
        HookMusicEvent::load();
        HookVoiceEvent::load();
        HookAudioSystemPlay::load();
        HookAudioSystemStop::load();
    }
    fn unload() {
        info!("on detach audioware");
        HookEntAudioEvent::unload();
        HookMusicEvent::unload();
        HookVoiceEvent::unload();
        HookAudioSystemPlay::unload();
        HookAudioSystemStop::unload();
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
