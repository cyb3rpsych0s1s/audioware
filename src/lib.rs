mod addresses;
mod audio;
mod banks;
mod hook;
mod interop;
mod locale;

use std::borrow::BorrowMut;

use red4ext_rs::plugin::Plugin;
use red4ext_rs::plugin::Version;
use red4ext_rs::prelude::*;

use crate::hook::hook_ent_audio_event;
use crate::hook::HOOK_ON_ENT_AUDIO_EVENT;

pub struct Audioware;
impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);
    fn register() {
        info!("on attach audioware");
        let _ = hook_ent_audio_event();
    }
    fn unload() {
        info!("on detach audioware");
        let _ = HOOK_ON_ENT_AUDIO_EVENT
            .clone()
            .borrow_mut()
            .lock()
            .unwrap()
            .take();
    }
}

/// # Safety
/// this is only safe as long as it matches memory representation specified in [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).
unsafe trait FromMemory {
    fn from_memory(address: usize) -> Self;
}
