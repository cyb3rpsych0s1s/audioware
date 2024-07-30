use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};
use std::mem;

use crate::{
    types::{Event, PlaySound, StopSound},
    Audioware,
};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut Event) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::PLAY_OR_STOP_SOUND_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for PlaySound/StopSound event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut Event,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut Event),
) {
    if !a2.is_null() {
        let event = unsafe { &*a2 };
        if event.as_ref().as_serializable().is_a::<PlaySound>() {
            let &PlaySound {
                sound_name,
                emitter_name,
                audio_tag,
                seek_time,
                play_unique,
                ..
            } = unsafe { mem::transmute(event) };
            log::info!(
                Audioware::env(),
                "intercepted PlaySound:
- sound_name: {sound_name}
- emitter_name: {emitter_name}
- audio_tag: {audio_tag}
- seek_time: {seek_time}
- play_unique: {play_unique}",
            );
        } else if event.as_ref().as_serializable().is_a::<StopSound>() {
            let &StopSound { sound_name, .. } = unsafe { mem::transmute(event) };
            log::info!(
                Audioware::env(),
                "intercepted StopSound:
- sound_name: {sound_name}",
            );
        } else {
            log::info!(
                Audioware::env(),
                "intercepted unknown event: {}",
                event.as_ref().as_serializable().class().name()
            );
        }
    } else {
        log::info!(Audioware::env(), "intercepted PlaySound/StopSound (null)");
    }

    cb(a1, a2);
}
