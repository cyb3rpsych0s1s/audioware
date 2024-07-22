use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable, LocalizationString, Ref},
    ScriptClass,
};

use super::{ECustomCameraTarget, GamePlayerSocket, RedTagList, RenderSceneLayerMask};

#[repr(C)]
pub struct GameObject {
    pub base: IScriptable,
    pub _padding0: [u8; 0x114],
    pub custom_camera_target: ECustomCameraTarget, // 0x154
    pub _padding1: [u8; 0x6],
    pub render_scene_layer_mask: RenderSceneLayerMask, // 0x15B
    pub _padding2: [u8; 0xC],
    pub persistent_state: Ref<IScriptable>,      // 0x168
    pub display_name: LocalizationString,        // 0x178
    pub display_description: LocalizationString, // 0x1A0
    pub audio_resource_name: CName,              // 0x1C8
    pub player_socket: GamePlayerSocket,         // 0x1D0
    pub visibility_check_distance: f32,          // 0x1F8
    pub _padding3: [u8; 0x1C],
    pub ui_slot_component: Ref<IScriptable>, // 0x218
    pub _padding4: [u8; 0x8],
    pub tags: RedTagList, // 0x230
}

unsafe impl ScriptClass for GameObject {
    const NAME: &'static str = "gameObject";
    type Kind = Native;
}

impl AsRef<IScriptable> for GameObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}
