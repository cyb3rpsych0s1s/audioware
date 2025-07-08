use red4ext_rs::{
    RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable, LocalizationString, Ref},
};

use super::{Entity, GamePlayerSocket, RedTagList};

#[repr(C)]
pub struct GameObject {
    pub base: Entity,
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

impl AsRef<Entity> for GameObject {
    #[inline]
    fn as_ref(&self) -> &Entity {
        &self.base
    }
}

impl AsRef<IScriptable> for GameObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

pub trait AsGameObject {
    fn is_player(&self) -> bool;
    fn get_display_name(&self) -> String;
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

    fn get_display_name(&self) -> String {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(GameObject::NAME)).unwrap();
        let method = cls.get_method(CName::new("GetDisplayName")).ok().unwrap();
        method
            .as_function()
            .execute::<_, String>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}

pub trait AsGameObjectExt {
    /// Provide a simple interface to resolve
    /// the name the most likely to be displayed
    /// for any game object and descendants.
    fn resolve_display_name(&self) -> String;
}

impl AsGameObjectExt for Ref<GameObject> {
    fn resolve_display_name(&self) -> String {
        self.get_display_name()
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

impl AsRef<Entity> for VehicleObject {
    fn as_ref(&self) -> &Entity {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for VehicleObject {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

impl AsGameObjectExt for Ref<VehicleObject> {
    fn resolve_display_name(&self) -> String {
        let go = unsafe { std::mem::transmute::<&Ref<VehicleObject>, &Ref<GameObject>>(self) };
        go.resolve_display_name()
    }
}

#[repr(C)]
pub struct WheeledObject {
    pub base: VehicleObject,
}

unsafe impl ScriptClass for WheeledObject {
    const NAME: &'static str = "vehicleWheeledBaseObject";
    type Kind = Native;
}

impl AsRef<VehicleObject> for WheeledObject {
    #[inline]
    fn as_ref(&self) -> &VehicleObject {
        &self.base
    }
}

impl AsRef<IScriptable> for WheeledObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct BikeObject {
    pub base: WheeledObject,
}

unsafe impl ScriptClass for BikeObject {
    const NAME: &'static str = "vehicleBikeBaseObject";
    type Kind = Native;
}

impl AsRef<VehicleObject> for BikeObject {
    #[inline]
    fn as_ref(&self) -> &VehicleObject {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for BikeObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct CarObject {
    pub base: WheeledObject,
}

unsafe impl ScriptClass for CarObject {
    const NAME: &'static str = "vehicleCarBaseObject";
    type Kind = Native;
}

impl AsRef<VehicleObject> for CarObject {
    #[inline]
    fn as_ref(&self) -> &VehicleObject {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for CarObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct AvObject {
    pub base: VehicleObject,
}

unsafe impl ScriptClass for AvObject {
    const NAME: &'static str = "vehicleAVBaseObject";
    type Kind = Native;
}

impl AsRef<VehicleObject> for AvObject {
    #[inline]
    fn as_ref(&self) -> &VehicleObject {
        &self.base
    }
}

impl AsRef<IScriptable> for AvObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct TankObject {
    pub base: VehicleObject,
}

unsafe impl ScriptClass for TankObject {
    const NAME: &'static str = "vehicleTankBaseObject";
    type Kind = Native;
}

impl AsRef<VehicleObject> for TankObject {
    #[inline]
    fn as_ref(&self) -> &VehicleObject {
        &self.base
    }
}

impl AsRef<IScriptable> for TankObject {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
