use glam::{Quat, Vec3};
use red4ext_rs::{
    class_kind::Native,
    types::{CName, EntityId, IScriptable, Method, NativeGameInstance, RedArray, Ref, ResRef},
    RttiSystem, ScriptClass, VoidPtr,
};

use super::{
    ECustomCameraTarget, Quaternion, RedTagList, RenderSceneLayerMask, Vector4, WorldRuntimeScene,
    WorldTransform,
};

const EVENT_MANAGER_PADDING: usize = 0x138 - 0xD8;
const PADDING_UNK148: usize = 0x154 - 0x148;
const PADDING_UNK157: usize = 0x15B - 0x157;

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
    fn get_world_position(&self) -> Vector4;
    fn get_world_orientation(&self) -> Quaternion;
    fn get_world_transform(&self) -> WorldTransform;
}

impl AsEntity for Ref<Entity> {
    fn get_world_position(&self) -> Vector4 {
        let attached = unsafe { self.instance() }
            .map(|x| x.status == EntityStatus::Attached)
            .unwrap_or(false);
        if !attached {
            return Vec3::NEG_Z.into();
        }
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("GetWorldPosition")).ok().unwrap();
        method
            .as_function()
            .execute::<_, Vector4>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }

    fn get_world_orientation(&self) -> Quaternion {
        let attached = unsafe { self.instance() }
            .map(|x| x.status == EntityStatus::Attached)
            .unwrap_or(false);
        if !attached {
            return Quat::IDENTITY.into();
        }
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetWorldOrientation"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, Quaternion>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }

    fn get_world_transform(&self) -> WorldTransform {
        let attached = unsafe { self.instance() }
            .map(|x| x.status == EntityStatus::Attached)
            .unwrap_or(false);
        if !attached {
            return WorldTransform::default();
        }
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetWorldTransform;"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, WorldTransform>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}
