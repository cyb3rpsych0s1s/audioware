use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable, LocalizationString, Ref},
    RttiSystem, ScriptClass,
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

pub trait AsGameObject {
    fn is_player(&self) -> bool;
}

impl AsGameObject for Ref<GameObject> {
    fn is_player(&self) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(GameObject::NAME)).unwrap();
        let method = cls.get_method(CName::new("IsPlayer;")).ok().unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}

const PADDING_240: usize = 0x25C - 0x240;
const PADDING_25D: usize = 0x3A0 - 0x25D;
const PADDING_3B8: usize = 0x6D2 - 0x3B8;
const PADDING_6D3: usize = 0xB90 - 0x6D3;

#[repr(C)]
pub struct VehicleObject {
    base: GameObject,
    unk240: [u8; PADDING_240],        // 240
    is_on_ground: bool,               // 25C
    unk25d: [u8; PADDING_25D],        // 25D
    archetype: [u8; 0x18],            // 3A0 (TODO: Ref<AI::Archetype>)
    unk3b8: [u8; PADDING_3B8],        // 3B8
    is_vehicle_on_state_locked: bool, // 6D2
    unk6d3: [u8; PADDING_6D3],        // 6D3
}

unsafe impl ScriptClass for VehicleObject {
    type Kind = Native;
    const NAME: &'static str = "vehicleBaseObject";
}

impl AsRef<GameObject> for VehicleObject {
    fn as_ref(&self) -> &GameObject {
        &self.base
    }
}

impl AsRef<IScriptable> for VehicleObject {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
