#![allow(dead_code)]

use red4ext_rs::{
    class_kind::Native,
    types::{CName, EntityId, GameInstance, IScriptable, Method, Opt, Ref},
    NativeRepr, RttiSystem, ScriptClass,
};

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

impl From<Vector4> for mint::Vector3<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
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

#[repr(C)]
pub struct Entity {
    pub base: IScriptable,
    pub _padding0: [u8; 0x114],
    pub custom_camera_target: ECustomCameraTarget, // 0x154
    pub _padding1: [u8; 0x6],
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
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("GetWorldPosition")).ok().unwrap();
        method
            .as_function()
            .execute::<_, Vector4>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }

    fn get_world_orientation(&self) -> Quaternion {
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
