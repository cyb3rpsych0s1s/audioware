use red4ext_rs::{
    ScriptClass,
    types::{CName, IScriptable},
};

use crate::{Entity, VORequestEvt, attach_native_event};

attach_native_event!(
    super::super::offsets::INK_VO_REQUEST_EVT,
    crate::VORequestEvt
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut VORequestEvt,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut VORequestEvt),
) {
    unsafe {
        let this = &*a1;
        let event = &*a2;
        let VORequestEvt {
            speaker_name,
            vo_id,
            ..
        } = event;
        let id = this
            .as_ref()
            .class()
            .name()
            .eq(&CName::new(Entity::NAME))
            .then(|| std::mem::transmute::<&IScriptable, &Entity>(this))
            .map(|x| x.entity_id);
        crate::utils::lifecycle!(
            "intercepted {} on {id:?}:
- speaker_name {}
- vo_id {vo_id:?}",
            VORequestEvt::NAME,
            speaker_name.to_string()
        );
        cb(a1, a2);
    }
}
