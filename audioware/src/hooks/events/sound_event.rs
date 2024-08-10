use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::SoundEvent, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut SoundEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SOUND_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for SoundEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SoundEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SoundEvent),
) {
    if !a2.is_null() {
        let &SoundEvent {
            event_name,
            ref switches,
            ref params,
            ref dynamic_params,
            ..
        } = unsafe { &*a2 };
        log::info!(
            Audioware::env(),
            "intercepted SoundEvent
- event_name: {event_name}
- switches: {}
- params: {}
- dynamic_params: {}",
            switches
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            params
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            dynamic_params
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
    } else {
        log::info!(Audioware::env(), "intercepted SoundEvent (null)");
    }

    cb(a1, a2);
}
