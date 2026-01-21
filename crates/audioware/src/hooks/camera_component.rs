use red4ext_rs::VoidPtr;

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr,
    a2: bool) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr = ::red4ext_rs::addr_hashes::resolve(
        super::offsets::CAMERACOMPONENT_OVERRIDE_AUDIO_LISTENERS,
    );
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for gameCameraComponent::OverrideAudioListeners( Bool )"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: bool,
    cb: unsafe extern "C" fn(a1: VoidPtr, a2: bool) -> (),
) {
    unsafe {
        crate::utils::inspect!("gameCameraComponent::OverrideAudioListeners( {a2} )");
        cb(a1, a2);
    }
}
