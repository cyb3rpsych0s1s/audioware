use glam::{Quat, Vec3};
use red4ext_rs::{
    NativeRepr, RttiSystem, ScriptClass, VoidPtr,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, Method, NativeGameInstance, RedArray, Ref, ResRef},
};

use super::{
    AIActionHelper, ECustomCameraTarget, GameObject, Quaternion, RedTagList, RenderSceneLayerMask,
    Vector4, WorldRuntimeScene, WorldTransform,
};

const EVENT_MANAGER_PADDING: usize = 0x138 - 0xD8;
const PADDING_UNK148: usize = 0x154 - 0x148;
const PADDING_UNK157: usize = 0x15B - 0x157;

const PADDING_UNK00: usize = 0x10;

#[derive(Debug)]
#[repr(C)]
pub struct EntityGameInterface {
    unk00: [u8; PADDING_UNK00], // 0
}

unsafe impl NativeRepr for EntityGameInterface {
    const NAME: &'static str = "entEntityGameInterface";
}

#[repr(C)]
pub struct Entity {
    pub base: IScriptable,
    unk40: u32,                                        // 0x40
    unk44: u32,                                        // 0x44
    pub entity_id: EntityId,                           // 0x48
    pub appearance_name: CName,                        // 0x50
    unk58: u64,                                        // 0x58
    pub template_path: ResRef,                         // 0x60
    unk68: u64,                                        // 0x68
    component_storage: [u8; 0x30],                     // 0x70
    pub components: RedArray<Ref<IScriptable>>,        // 0xA0
    transform_component: *const IScriptable,           // 0xB0
    runtime_scene: *const WorldRuntimeScene,           // 0xB8
    game_instance: *const NativeGameInstance,          // 0xC0
    unk_c8: VoidPtr,                                   // 0xC8
    unk_d0: VoidPtr,                                   // 0xD0
    event_manager: [u8; EVENT_MANAGER_PADDING],        // 0xD8
    visual_tags: RedTagList,                           // 0x138
    unk148: [u8; PADDING_UNK148],                      // 0x148
    pub custom_camera_target: ECustomCameraTarget,     // 0x154
    unk155: u8,                                        // 0x155
    pub status: EntityStatus,                          // 0x156
    unk157: [u8; PADDING_UNK157],                      // 0x157
    pub render_scene_layer_mask: RenderSceneLayerMask, // 0x15B
}

unsafe impl ScriptClass for Entity {
    const NAME: &'static str = "entEntity";
    type Kind = Native;
}

impl AsRef<IScriptable> for Entity {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum EntityStatus {
    Undefined = 0,
    Initializing = 1,
    Detached = 2,
    Attaching = 3,
    Attached = 4,
    Detaching = 5,
    Uninitializing = 6,
    Uninitialized = 7,
}

pub trait AsEntity {
    fn get_entity_id(&self) -> EntityId;
    fn get_world_position(&self) -> Vector4;
    fn get_world_forward(&self) -> Vector4;
    fn get_world_orientation(&self) -> Quaternion;
    fn get_world_transform(&self) -> WorldTransform;
    fn is_in_workspot(&self) -> bool;
}

impl AsEntity for Ref<Entity> {
    fn get_entity_id(&self) -> EntityId {
        if self.is_null() {
            return EntityId::default();
        }
        unsafe { self.instance() }.unwrap().entity_id
    }
    fn get_world_position(&self) -> Vector4 {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("GetWorldPosition")).ok().unwrap();
        match unsafe { self.instance() } {
            Some(x) if x.status == EntityStatus::Attached => method
                .as_function()
                .execute::<_, Vector4>(Some(x.as_ref()), ())
                .unwrap(),
            _ => Vec3::ZERO.into(),
        }
    }

    fn get_world_forward(&self) -> Vector4 {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("GetWorldForward")).ok().unwrap();
        match unsafe { self.instance() } {
            Some(x) if x.status == EntityStatus::Attached => method
                .as_function()
                .execute::<_, Vector4>(Some(x.as_ref()), ())
                .unwrap(),
            _ => Vec3::NEG_Z.into(),
        }
    }

    fn get_world_orientation(&self) -> Quaternion {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetWorldOrientation"))
            .ok()
            .unwrap();
        match unsafe { self.instance() } {
            Some(x) if x.status == EntityStatus::Attached => method
                .as_function()
                .execute::<_, Quaternion>(Some(x.as_ref()), ())
                .unwrap(),
            _ => Quat::IDENTITY.into(),
        }
    }

    fn get_world_transform(&self) -> WorldTransform {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetWorldTransform;"))
            .ok()
            .unwrap();
        match unsafe { self.instance() } {
            Some(x) if x.status == EntityStatus::Attached => method
                .as_function()
                .execute::<_, WorldTransform>(Some(x.as_ref()), ())
                .unwrap(),
            _ => WorldTransform::default(),
        }
    }

    fn is_in_workspot(&self) -> bool {
        self.clone()
            .cast::<GameObject>()
            .map(AIActionHelper::is_in_workspot)
            .unwrap_or(false)
    }
}
