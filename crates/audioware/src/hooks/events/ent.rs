use red4ext_rs::{
    ScriptClass,
    types::{CName, IScriptable},
};

use crate::{Entity, SoundPlayVO, attach_native_event};

attach_native_event!(super::super::offsets::SOUND_PLAY_VO, crate::SoundPlayVO);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SoundPlayVO,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SoundPlayVO),
) {
    unsafe {
        let this = &*a1;
        let event = &*a2;
        let SoundPlayVO {
            vo_context,
            is_quest,
            answering_entity_id,
            ..
        } = event;
        let id = this
            .as_ref()
            .class()
            .name()
            .eq(&CName::new(Entity::NAME))
            .then(|| std::mem::transmute::<&IScriptable, &Entity>(this))
            .map(|x| x.entity_id);
        if *is_quest {
            crate::utils::lifecycle!(
                "intercepted {} on {id:?} (quest):
    - vo_context {}
    - answering_entity_id {:?}",
                SoundPlayVO::NAME,
                vo_context.as_str(),
                answering_entity_id
            );
        }
        cb(a1, a2);
    }
}
