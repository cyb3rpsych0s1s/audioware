use red4ext_rs::{VoidPtr, types::Cruid};

use crate::utils::intercept;

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr, a2: Cruid, a3: i32, out: VoidPtr) -> u8;
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr =
        ::red4ext_rs::addr_hashes::resolve(super::offsets::LOCALIZATIONMANAGER_RESOLVEFILENAME);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for Localization::ResolveVoFileName( Cruid, GenderVariant, ResRef ) -> ResolveVoFileResult"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: Cruid,
    a3: i32,
    out: VoidPtr,
    cb: unsafe extern "C" fn(a1: VoidPtr, a2: Cruid, a3: i32, out: VoidPtr) -> u8,
) -> u8 {
    unsafe {
        intercept!("Localization::ResolveVoFileName( x, {:?}, {}, x )", a2, a3);
        cb(a1, a2, a3, out)
    }
}
