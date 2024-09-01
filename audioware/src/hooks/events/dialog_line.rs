use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::{DialogLine, DialogLineEventData};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut DialogLine) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::DIALOG_LINE_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for DialogLine event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut DialogLine,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut DialogLine),
) {
    if !a2.is_null() {
        let DialogLine { data, .. } = unsafe { &*a2 };
        let &DialogLineEventData {
            string_id,
            context,
            expression,
            is_player,
            is_rewind,
            is_holocall,
            custom_vo_event,
            seek_time,
            playback_speed_parameter,
            ..
        } = data;
        crate::utils::lifecycle!(
            "intercepted DialogLine:
- data.string_id {string_id:?}
- data.context {context}
- data.expression {expression}
- data.is_player {is_player}
- data.is_rewind {is_rewind}
- data.is_holocall {is_holocall}
- data.custom_vo_event {custom_vo_event}
- data.seek_time {seek_time}
- data.playback_speed_parameter {playback_speed_parameter}",
        );
    } else {
        crate::utils::lifecycle!("intercepted DialogLine (null)");
    }

    cb(a1, a2);
}
