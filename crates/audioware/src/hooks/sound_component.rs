use red4ext_rs::VoidPtr;

use crate::{StopDialogLine, utils::intercept};

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr,
    a2: *const StopDialogLine) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr = ::red4ext_rs::addr_hashes::resolve(super::offsets::SOUNDCOMPONENT_ONSTOPDIALOGLINE);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for SoundComponent::OnStopDialogLine( StopDialogLine )"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: *const StopDialogLine,
    cb: unsafe extern "C" fn(a1: VoidPtr, a2: *const StopDialogLine) -> (),
) {
    unsafe {
        intercept!("SoundComponent::OnStopDialogLine( {} )", (&*a2),);
        cb(a1, a2)
    }
}
