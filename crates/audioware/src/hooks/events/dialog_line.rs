use red4ext_rs::{
    ScriptClass,
    types::{CName, IScriptable},
};

use crate::{DialogLine, DialogLineEventData, Entity, attach_native_event};

attach_native_event!(super::super::offsets::EVENT_DIALOGLINE, crate::DialogLine);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut DialogLine,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut DialogLine),
) {
    unsafe {
        let this = &*a1;
        let event = &*a2;
        let DialogLine { data, .. } = event;
        let DialogLineEventData {
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
        let id = this
            .as_ref()
            .class()
            .name()
            .eq(&CName::new(Entity::NAME))
            .then(|| std::mem::transmute::<&IScriptable, &Entity>(this))
            .map(|x| x.entity_id);
        crate::utils::lifecycle!(
            "intercepted {} on {id:?}:
- data.string_id {string_id:?}
- data.context {context}
- data.expression {expression}
- data.is_player {is_player}
- data.is_rewind {is_rewind}
- data.is_holocall {is_holocall}
- data.custom_vo_event {custom_vo_event}
- data.seek_time {seek_time}
- data.playback_speed_parameter {playback_speed_parameter}",
            DialogLine::NAME
        );
        cb(a1, a2);
    }
}
