use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::SoundParameter, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut SoundParameter) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SOUND_PARAMETER_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for SoundParameter event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SoundParameter,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SoundParameter),
) {
    if !a2.is_null() {
        let &SoundParameter {
            parameter_name,
            parameter_value,
            ..
        } = unsafe { &*a2 };
        crate::utils::lifecycle!(
            Audioware::env(),
            "intercepted SoundParameter:
- parameter_name: {parameter_name}
- parameter_value: {parameter_value}",
        );
    } else {
        crate::utils::lifecycle!(Audioware::env(), "intercepted SoundParameter (null)");
    }

    cb(a1, a2);
}
