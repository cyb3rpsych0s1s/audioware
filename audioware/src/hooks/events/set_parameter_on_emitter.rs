use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::{EmitterEvent, SetParameterOnEmitter};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut SetParameterOnEmitter) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SET_PARAMETER_ON_EMITTER_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for SetParameterOnEmitter event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SetParameterOnEmitter,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SetParameterOnEmitter),
) {
    if !a2.is_null() {
        let event = unsafe { &*a2 };
        let &SetParameterOnEmitter {
            param_name,
            param_value,
            ..
        } = event;
        let &EmitterEvent { emitter_name, .. } = event.as_ref();
        crate::utils::lifecycle!(
            "intercepted SetParameterOnEmitter:
- base.emitter_name: {emitter_name}
- param_name: {param_name}
- param_value: {param_value}",
        );
    } else {
        crate::utils::lifecycle!("intercepted SetParameterOnEmitter (null)");
    }

    cb(a1, a2);
}
