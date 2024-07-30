use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};
use std::mem;

use crate::{
    types::{Event, SoundParameter, StopTaggedSounds},
    Audioware,
};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut Event) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr =
        addr_hashes::resolve(crate::hooks::offsets::STOP_TAGGED_SOUNDS_OR_SOUND_PARAMETER_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(
        env,
        "attached hook for StopTaggedSounds/SoundParameter event handler"
    );
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut Event,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut Event),
) {
    if !a2.is_null() {
        let event = unsafe { &*a2 };
        if event.as_ref().as_serializable().is_a::<StopTaggedSounds>() {
            let &StopTaggedSounds { audio_tag, .. } =
                unsafe { mem::transmute::<&Event, &StopTaggedSounds>(event) };
            log::info!(
                Audioware::env(),
                "intercepted StopTaggedSounds:
- audio_tag: {audio_tag}",
            );
        } else if event.as_ref().as_serializable().is_a::<SoundParameter>() {
            let &SoundParameter {
                parameter_name,
                parameter_value,
                ..
            } = unsafe { mem::transmute::<&Event, &SoundParameter>(event) };
            // this one fires repeatedly
            if parameter_name.as_str() != "player_health" {
                log::info!(
                    Audioware::env(),
                    "intercepted SoundParameter:
    - parameter_name: {parameter_name}
    - parameter_value: {parameter_value}",
                );
            }
        } else {
            log::info!(
                Audioware::env(),
                "intercepted unknown event: {}",
                event.as_ref().as_serializable().class().name()
            );
        }
    } else {
        log::info!(
            Audioware::env(),
            "intercepted StopTaggedSounds or SoundParameter (null)"
        );
    }

    cb(a1, a2);
}
