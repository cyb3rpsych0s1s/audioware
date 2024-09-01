use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::SoundPlayVO;

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut SoundPlayVO) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SOUND_PLAY_VO_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for SoundPlayVO event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SoundPlayVO,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SoundPlayVO),
) {
    if !a2.is_null() {
        let &SoundPlayVO {
            vo_context,
            is_quest,
            ignore_frustum_check,
            ignore_distance_check,
            ignore_global_vo_limit_check,
            debug_initial_context,
            answering_entity_id,
            overriding_voiceover_context,
            overriding_voiceover_expression,
            override_voiceover_expression,
            overriding_visual_style_value,
            override_visual_style,
            ..
        } = unsafe { &*a2 };
        crate::utils::lifecycle!(
            "intercepted SoundPlayVO:
- vo_context: {vo_context}
- is_quest: {is_quest}
- ignore_frustum_check: {ignore_frustum_check}
- ignore_distance_check: {ignore_distance_check}
- ignore_global_vo_limit_check: {ignore_global_vo_limit_check}
- debug_initial_context: {debug_initial_context}
- answering_entity_id: {answering_entity_id:?}
- overriding_voiceover_context: {overriding_voiceover_context}
- overriding_voiceover_expression: {overriding_voiceover_expression}
- override_voiceover_expression: {override_voiceover_expression}
- overriding_visual_style_value: {overriding_visual_style_value}
- override_visual_style: {override_visual_style}",
        );
    } else {
        crate::utils::lifecycle!("intercepted SoundPlayVO (null)");
    }

    cb(a1, a2);
}
