#![allow(dead_code)]

use glam::{Quat, Vec3};
use red4ext_rs::{
    class_kind::Native,
    types::{
        CName, EntityId, GameInstance, IScriptable, LocalizationString, Method, NativeGameInstance,
        Opt, RedArray, Ref, ResRef,
    },
    NativeRepr, RttiSystem, ScriptClass, VoidPtr,
};

pub trait AsIScriptable {
    fn is_a(&self, class_name: CName) -> bool;
    fn is_exactly_a(&self, class_name: CName) -> bool;
    fn get_class_name(&self) -> CName;
}

impl AsIScriptable for Ref<IScriptable> {
    fn is_a(&self, class_name: CName) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(IScriptable::NAME)).unwrap();
        let method = cls.get_method(CName::new("IsA")).ok().unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }, (class_name,))
            .unwrap()
    }

    fn is_exactly_a(&self, class_name: CName) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(IScriptable::NAME)).unwrap();
        let method = cls.get_method(CName::new("IsExactlyA")).ok().unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }, (class_name,))
            .unwrap()
    }

    fn get_class_name(&self) -> CName {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(IScriptable::NAME)).unwrap();
        let method = cls.get_method(CName::new("GetClassName")).ok().unwrap();
        method
            .as_function()
            .execute::<_, CName>(unsafe { self.instance() }, ())
            .unwrap()
    }
}

#[repr(C)]
pub struct AudioSystem {
    pub base: IScriptable,
    pub _padding0: [u8; 0x3E0],
}

unsafe impl ScriptClass for AudioSystem {
    const NAME: &'static str = "gameGameAudioSystem";
    type Kind = Native;
}

impl AsRef<IScriptable> for AudioSystem {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

#[allow(dead_code)]
pub trait AsAudioSystem {
    fn play(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>);
    fn stop(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>);
    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
    );
}

impl AsAudioSystem for Ref<AudioSystem> {
    fn play(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("Play")).ok().unwrap();
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (
                    event_name,
                    entity_id.unwrap_or_default(),
                    emitter_name.unwrap_or_default(),
                ),
            )
            .unwrap();
    }

    fn stop(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("Stop")).ok().unwrap();
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (
                    event_name,
                    entity_id.unwrap_or_default(),
                    emitter_name.unwrap_or_default(),
                ),
            )
            .unwrap();
    }

    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
    ) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("Switch")).ok().unwrap();
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (
                    switch_name,
                    switch_value,
                    entity_id.unwrap_or_default(),
                    emitter_name.unwrap_or_default(),
                ),
            )
            .unwrap();
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
#[repr(C, align(16))]
pub struct Vector4 {
    pub x: f32, // 0x0
    pub y: f32, // 0x4
    pub z: f32, // 0x8
    pub w: f32, // 0xC
}

unsafe impl NativeRepr for Vector4 {
    const NAME: &'static str = "Vector4";
}

impl From<Vector4> for mint::Vector3<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector4> for glam::Vec3 {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<glam::Vec3> for Vector4 {
    fn from(value: glam::Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: 1.,
        }
    }
}

impl From<mint::Vector4<f32>> for Vector4 {
    fn from(value: mint::Vector4<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<Vector4> for mint::Vector4<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
#[repr(C, align(16))]
pub struct Quaternion {
    pub i: f32, // 0x0
    pub j: f32, // 0x4
    pub k: f32, // 0x8
    pub r: f32, // 0xC
}

unsafe impl NativeRepr for Quaternion {
    const NAME: &'static str = "Quaternion";
}

impl From<glam::Quat> for Quaternion {
    fn from(value: glam::Quat) -> Self {
        Self {
            i: value.x,
            j: value.y,
            k: value.z,
            r: value.w,
        }
    }
}

impl From<mint::Quaternion<f32>> for Quaternion {
    fn from(value: mint::Quaternion<f32>) -> Self {
        Self {
            i: value.v.x,
            j: value.v.y,
            k: value.v.z,
            r: value.s,
        }
    }
}

impl From<Quaternion> for mint::Quaternion<f32> {
    fn from(value: Quaternion) -> Self {
        Self {
            v: mint::Vector3 {
                x: value.i,
                y: value.j,
                z: value.k,
            },
            s: value.r,
        }
    }
}

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

#[repr(C, align(8))]
pub struct WorldRuntimeScene {
    pub _padding0: [u8; 0x4B8],
}

unsafe impl NativeRepr for WorldRuntimeScene {
    const NAME: &'static str = "worldRuntimeScene";
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ECustomCameraTarget {
    EcctvAll = 0,
    EcctvOnlyOffscreen = 1,
    EcctvOnlyOnscreen = 2,
}

unsafe impl NativeRepr for ECustomCameraTarget {
    const NAME: &'static str = "ECustomCameraTarget";
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RenderSceneLayerMask(pub [u8; 0x1]);

pub trait AsGameInstance {
    fn find_entity_by_id(game: GameInstance, entity_id: EntityId) -> Ref<Entity>;
}

impl AsGameInstance for GameInstance {
    fn find_entity_by_id(game: GameInstance, entity_id: EntityId) -> Ref<Entity> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(GameInstance::NAME)).unwrap();
        let methods = cls.static_methods();
        let method = methods
            .iter()
            .find(|x| x.as_function().name() == CName::new("FindEntityByID"))
            .unwrap();
        method
            .as_function()
            .execute::<_, Ref<Entity>>(None, (game, entity_id))
            .unwrap()
    }
}

pub trait AsEntity {
    fn get_world_position(&self) -> Vector4;
    fn get_world_orientation(&self) -> Quaternion;
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
}

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

#[repr(C, align(8))]
pub struct GamePlayerSocket {
    pub _padding0: [u8; 0x28],
}

unsafe impl NativeRepr for GamePlayerSocket {
    const NAME: &'static str = "gamePlayerSocket";
}

#[repr(C, align(8))]
pub struct RedTagList {
    pub tags: RedArray<CName>, // 0x0
}

unsafe impl NativeRepr for RedTagList {
    const NAME: &'static str = "redTagList";
}
