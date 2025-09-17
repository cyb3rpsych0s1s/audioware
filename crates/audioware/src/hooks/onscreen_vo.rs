use red4ext_rs::{
    VoidPtr,
    types::{Cruid, RedString},
};

use crate::utils::intercept;

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr, a2: *const Cruid, a3: VoidPtr, a4: *const RedString) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr =
        ::red4ext_rs::addr_hashes::resolve(super::offsets::ONSCREENVOPLAYERCONTROLLER_SHOWSUBTITLE);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for OnscreenVOPlayerController::ShowSubtitle( Cruid, VoidPtr, RedString )"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: *const Cruid,
    a3: VoidPtr,
    a4: *const RedString,
    cb: unsafe extern "C" fn(
        a1: VoidPtr,
        a2: *const Cruid,
        a3: VoidPtr,
        a4: *const RedString,
    ) -> (),
) {
    unsafe {
        intercept!(
            "OnscreenVOPlayerController::ShowSubtitle( {:?}, x, {} )",
            (&*a2),
            (&*a4)
        );
        cb(a1, a2, a3, a4)
    }
}
