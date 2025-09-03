use kira::backend::cpal::CpalBackend;
use red4ext_rs::{
    VoidPtr,
    types::{CName, EntityId},
};

use crate::{DialogLineEventData, abi::command::Command, engine::Engine};

::red4ext_rs::hooks! {
        static HOOK: fn(
a1: *const DialogLineEventData,
a2: VoidPtr,
a3: EntityId,
a4: CName,
a5: bool,
a6: *const u32,
a7: *const CName,
a8: f32,
a9: CName) -> bool;
    }

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr = ::red4ext_rs::addr_hashes::resolve(super::offsets::AUDIO_PLAY_DIALOG_LINE);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for audio::PlayDialogLine( DialogLineEventData, VoidPtr, EntityId, CName, bool, u32, CName, f32, CName ) -> Bool"
    );
}

unsafe extern "C" fn detour(
    a1: *const DialogLineEventData,
    a2: VoidPtr,
    a3: EntityId,
    a4: CName,
    a5: bool,
    a6: *const u32,
    a7: *const CName,
    a8: f32,
    a9: CName,
    cb: unsafe extern "C" fn(
        a1: *const DialogLineEventData,
        a2: VoidPtr,
        a3: EntityId,
        a4: CName,
        a5: bool,
        a6: *const u32,
        a7: *const CName,
        a8: f32,
        a9: CName,
    ) -> bool,
) -> bool {
    unsafe {
        let data = &*a1;
        let string_id = data.string_id;

        if Engine::<CpalBackend>::exists_for_scene(&string_id) {
            let is_player = data.is_player;
            let is_holocall = data.is_holocall;
            let is_rewind = data.is_rewind;
            crate::utils::intercept!(
                "audio::PlayDialogLine
        - data.string_id: {string_id:?}
        - data.is_player: {is_player}
        - data.is_holocall: {is_holocall}
        - data.is_rewind: {is_rewind}
        - data.custom_vo_event: {}
        - entity_id: {a3:?}
        - vo_event_override: {}
        - played_vo_event: {}
        - tag_event: {}",
                a4.as_str(),
                data.custom_vo_event.as_str(),
                (*a7).as_str(),
                a9.as_str()
            );
            crate::engine::queue::send(Command::PlaySceneDialog { string_id });
        }
        // still let the engine carry on, to handle subtitle
        let out = cb(a1, a2, a3, a4, a5, a6, a7, a8, a9);
        crate::utils::intercept!("audio::PlayDialogLine -> {out}");
        out
    }
}
