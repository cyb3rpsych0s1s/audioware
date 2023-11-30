mod addresses;
mod audio;
mod banks;
mod frame;
mod hook;
mod interop;
mod locale;

use std::borrow::BorrowMut;

use red4ext_rs::plugin::Plugin;
use red4ext_rs::plugin::Version;
use red4ext_rs::prelude::*;

use crate::hook::hook_ent_audio_event;
use crate::hook::hook_on_audiosystem_play;
use crate::hook::hook_on_audiosystem_stop;
use crate::hook::hook_on_music_event;
use crate::hook::hook_on_voice_event;
use crate::hook::HOOK_ON_AUDIOSYSTEM_PLAY;
use crate::hook::HOOK_ON_AUDIOSYSTEM_STOP;

use crate::hook::HOOK_ON_ENT_AUDIO_EVENT;

use crate::hook::HOOK_ON_MUSIC_EVENT;
use crate::hook::HOOK_ON_VOICE_EVENT;

pub struct Audioware;
impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);
    fn post_register() {
        info!("on attach audioware");
        match hook_ent_audio_event() {
            Ok(_) => {}
            Err(e) => {
                error!("error {e}")
            }
        };
        match hook_on_music_event() {
            Ok(_) => {}
            Err(e) => {
                error!("error {e}")
            }
        };
        match hook_on_voice_event() {
            Ok(_) => {}
            Err(e) => {
                error!("error {e}")
            }
        };
        match hook_on_audiosystem_play() {
            Ok(_) => {
                info!("successfully hooked into AudioSystem::Play");
            }
            Err(e) => {
                error!("error {e}")
            }
        };
        match hook_on_audiosystem_stop() {
            Ok(_) => {
                info!("successfully hooked into AudioSystem::Stop");
            }
            Err(e) => {
                error!("error {e}")
            }
        };
    }
    fn unload() {
        info!("on detach audioware");
        let _ = HOOK_ON_ENT_AUDIO_EVENT
            .clone()
            .borrow_mut()
            .lock()
            .unwrap()
            .take();
        let _ = HOOK_ON_MUSIC_EVENT
            .clone()
            .borrow_mut()
            .lock()
            .unwrap()
            .take();
        let _ = HOOK_ON_VOICE_EVENT
            .clone()
            .borrow_mut()
            .lock()
            .unwrap()
            .take();
        let _ = HOOK_ON_AUDIOSYSTEM_PLAY
            .clone()
            .borrow_mut()
            .lock()
            .unwrap()
            .take();
        let _ = HOOK_ON_AUDIOSYSTEM_STOP
            .clone()
            .borrow_mut()
            .lock()
            .unwrap()
            .take();
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
