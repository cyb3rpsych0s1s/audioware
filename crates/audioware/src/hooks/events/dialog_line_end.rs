use red4ext_rs::ScriptClass;
use red4ext_rs::types::IScriptable;

use crate::{DialogLineEnd, attach_native_event};

attach_native_event!(
    super::super::offsets::EVENT_DIALOGLINEEND,
    crate::DialogLineEnd
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut DialogLineEnd,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut DialogLineEnd),
) {
    unsafe {
        crate::utils::lifecycle!("intercepted {}", DialogLineEnd::NAME);
        cb(a1, a2);
    }
}
