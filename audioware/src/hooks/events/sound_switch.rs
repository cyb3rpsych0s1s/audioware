use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::SoundSwitch;

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut SoundSwitch) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SOUND_SWITCH_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for SoundSwitch event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SoundSwitch,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SoundSwitch),
) {
    if !a2.is_null() {
        let &SoundSwitch {
            switch_name,
            switch_value,
            ..
        } = unsafe { &*a2 };
        crate::utils::lifecycle!(
            "intercepted SoundSwitch:
- switch_name: {switch_name}
- switch_value: {switch_value}",
        );
    } else {
        crate::utils::lifecycle!("intercepted SoundSwitch (null)");
    }

    cb(a1, a2);
}
