use red4ext_rs::VoidPtr;

use crate::{
    AudioEventId, AudioInternalEvent, EventName,
    engine::{Mute, Replacements},
};

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr,
    a2: *const AudioInternalEvent,
    a3: *const AudioEventId) -> ();
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
    a2: *const AudioInternalEvent,
    a3: *const AudioEventId,
    cb: unsafe extern "C" fn(
        a1: VoidPtr,
        a2: *const AudioInternalEvent,
        a3: *const AudioEventId,
    ) -> (),
) {
    unsafe {
        if !a2.is_null() && !a1.offset(8).is_null() {
            let event = &*a2;
            if let Ok(event_name) = EventName::try_from(event.event_name())
                && !Replacements.is_muted(event_name)
            {
                crate::utils::intercept!("AudioInterface::PostEvent( {{ {event} }}, .. )");
                cb(a1, a2, a3);
            }
        }
    }
}
