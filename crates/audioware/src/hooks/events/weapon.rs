use red4ext_rs::types::IScriptable;
use red4ext_rs::ScriptClass;

use crate::{attach_native_event, PreFireEvent};

attach_native_event!(
    super::super::offsets::WEAPON_PRE_FIRE_EVENT,
    crate::PreFireEvent
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut PreFireEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut PreFireEvent),
) {
    let event = &*a2;
    let PreFireEvent { .. } = event;
    crate::utils::lifecycle!("intercepted {}", PreFireEvent::NAME);
    cb(a1, a2);
}
