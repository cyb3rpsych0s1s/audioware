use red4ext_rs::types::IScriptable;
use red4ext_rs::ScriptClass;

use crate::{attach_native_event, DialogLineEnd};

attach_native_event!(
    super::super::offsets::EVENT_DIALOGLINEEND,
    crate::DialogLineEnd
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut DialogLineEnd,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut DialogLineEnd),
) {
    crate::utils::lifecycle!("intercepted {}", DialogLineEnd::NAME);
    cb(a1, a2);
}
