use std::ops::Not;

use red4ext_rs::VoidPtr;

use crate::{AudioEventId, AudioInternalEvent};

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr,
    a2: *mut AudioInternalEvent,
    a3: *mut AudioEventId) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr = ::red4ext_rs::addr_hashes::resolve(super::offsets::AUDIOINTERFACE_POST_EVENT);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for AudioInterface::PostEvent( AudioInternalEvent, AudioEventId )"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: *mut AudioInternalEvent,
    a3: *mut AudioEventId,
    cb: unsafe extern "C" fn(a1: VoidPtr, a2: *mut AudioInternalEvent, a3: *mut AudioEventId) -> (),
) {
    unsafe {
        let interface = a2.is_null().not().then(|| &*a2);
        let id = a3.is_null().not().then(|| *a3);
        if let (Some(interface), Some(id)) = (interface, id) {
            crate::utils::inspect!("AudioInterface::PostEvent( {{ {interface} }}, {id} )");
        }
        cb(a1, a2, a3);
    }
}
