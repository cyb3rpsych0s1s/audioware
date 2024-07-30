use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};
use std::mem;

use crate::{
    types::{Event, SoundSwitch, StopSound},
    Audioware,
};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut Event) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::STOP_OR_SWITCH_SOUND_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for StopSound/SoundSwitch event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut Event,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut Event),
) {
    if !a2.is_null() {
        let event = unsafe { &*a2 };
        if event.as_ref().as_serializable().is_a::<SoundSwitch>() {
            let &SoundSwitch {
                switch_name,
                switch_value,
                ..
            } = unsafe { mem::transmute(event) };
            log::info!(
                Audioware::env(),
                "intercepted SoundSwitch:
- switch_name: {switch_name}
- switch_value: {switch_value}",
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
        log::info!(Audioware::env(), "intercepted StopSound/SoundSwitch (null)");
    }

    cb(a1, a2);
}
