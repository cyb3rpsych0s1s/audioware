use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};
use std::mem;

use crate::types::{Event, SoundSwitch, StopSound};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut Event) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::STOP_OR_SWITCH_SOUND_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for StopSound/SoundSwitch event handler");
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
            } = unsafe { mem::transmute::<&Event, &SoundSwitch>(event) };
            crate::utils::lifecycle!(
                "intercepted SoundSwitch:
- switch_name: {switch_name}
- switch_value: {switch_value}",
            );
        } else if event.as_ref().as_serializable().is_a::<StopSound>() {
            let &StopSound { sound_name, .. } =
                unsafe { mem::transmute::<&Event, &StopSound>(event) };
            crate::utils::lifecycle!(
                "intercepted StopSound:
- sound_name: {sound_name}",
            );
        } else {
            crate::utils::lifecycle!(
                "intercepted unknown event: {}",
                event.as_ref().as_serializable().class().name()
            );
        }
    } else {
        crate::utils::lifecycle!("intercepted StopSound/SoundSwitch (null)");
    }

    cb(a1, a2);
}
