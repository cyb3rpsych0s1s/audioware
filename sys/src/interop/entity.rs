use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    macros::{redscript_global, redscript_import},
    types::{CName, EntityId, MaybeUninitRef, Ref, ScriptRef, Variant, VariantExt},
};
use serde::Deserialize;
use snafu::OptionExt;

use crate::{
    error::{FromVariantSnafu, ReflectionError, UnknownClassSnafu, UnknownStaticFuncSnafu},
    impl_safe_downcast,
    interop::reflection::Reflection,
};

use super::{
    game::GameInstance, icomponent::IComponent, iscriptable::IScriptable, quaternion::Quaternion,
    vector4::Vector4,
};

impl<'a> From<&'a EntityId> for Display<'a> {
    fn from(inner: &'a EntityId) -> Self {
        Self { inner }
    }
}

pub struct Display<'a> {
    inner: &'a EntityId,
}

impl<'a> std::fmt::Display for self::Display<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.inner.is_defined() {
            return write!(f, "undefined");
        }
        let hash = u64::from(self.inner.clone());
        let suffix = match (self.inner.is_dynamic(), self.inner.is_persistable()) {
            (true, true) => "dynamic+persistable",
            (false, false) => "static",
            (false, true) => "static+persistable",
            (true, false) => "dynamic",
        };
        write!(f, "{hash}>{{{suffix}}}")
    }
}

/// there's a weird bug when using `#[redscript_import]` or direct `call!`,
/// so call it indirectly.
#[redscript_global(name = "Audioware.FindEntityByID")]
pub fn find_entity_by_id(gi: GameInstance, id: EntityId) -> MaybeUninitRef<Entity>;

#[derive(Debug)]
pub struct Entity;

impl ClassType for Entity {
    type BaseClass = IScriptable;
    const NAME: &'static str = "Entity";
    const NATIVE_NAME: &'static str = "entEntity";
}

#[redscript_import]
impl Entity {
    #[redscript(native)]
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool;
    #[redscript(native)]
    pub fn is_a(self: &Ref<Self>, class_name: CName) -> bool;
    /// public native func GetWorldPosition() -> Vector4
    #[redscript(native)]
    pub fn get_world_position(self: &Ref<Self>) -> Vector4;
    /// public native func GetWorldOrientation() -> Quaternion
    #[redscript(native)]
    pub fn get_world_orientation(self: &Ref<Self>) -> Quaternion;
}

impl Entity {
    pub fn is_player(self: &Ref<Self>) -> bool {
        self.is_exactly_a(CName::new("PlayerPuppet"))
    }
}

#[cfg(feature = "codeware")]
#[redscript_import]
impl Entity {
    pub fn get_components(self: &Ref<Self>) -> Vec<Ref<IComponent>>;
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct EntityGameInterface {
    unk00: [u8; 0x10],
}

unsafe impl NativeRepr for EntityGameInterface {
    const NAME: &'static str = "EntityGameInterface";
    const NATIVE_NAME: &'static str = "entEntityGameInterface";
}

impl EntityGameInterface {
    /// public final static native func GetEntity(self: EntityGameInterface) -> ref<Entity>
    pub fn get_entity(self) -> Result<Ref<Entity>, ReflectionError> {
        let class = Reflection::get_class(CName::new(EntityGameInterface::NATIVE_NAME))
            .into_ref()
            .context(UnknownClassSnafu {
                name: EntityGameInterface::NATIVE_NAME,
            })?;
        let func_name = "GetEntity";
        let func = class
            .get_static_function(CName::new(func_name))
            .into_ref()
            .context(UnknownStaticFuncSnafu {
                name: func_name,
                owner: EntityGameInterface::NATIVE_NAME,
            })?;
        let mut inner = false;
        let status = ScriptRef::new(&mut inner);
        let mut out = func.call(vec![Variant::new(self)], status);
        let out: Option<Ref<Entity>> = out.try_take();
        out.context(FromVariantSnafu {
            name: std::any::type_name::<Ref<Entity>>(),
        })
    }
}

#[derive(Debug)]
pub struct GameObject;

impl ClassType for GameObject {
    type BaseClass = Entity;
    const NAME: &'static str = "GameObject";
    const NATIVE_NAME: &'static str = "gameObject";
}

#[redscript_import]
impl GameObject {
    /// public const func IsDevice() -> Bool
    pub fn is_device(self: &Ref<Self>) -> bool;
    /// public const func IsPuppet() -> Bool
    pub fn is_puppet(self: &Ref<Self>) -> bool;
    /// public const func IsPlayer() -> Bool
    pub fn is_player(self: &Ref<Self>) -> bool;
}

#[derive(Debug)]
pub struct ScriptedPuppet;

impl ClassType for ScriptedPuppet {
    type BaseClass = GameObject;
    const NAME: &'static str = "ScriptedPuppet";
}

#[redscript_import]
impl ScriptedPuppet {
    /// public final const func GetGender() -> CName
    pub fn get_gender(self: &Ref<Self>) -> CName;
    /// public final const func GetNPCType() -> gamedataNPCType
    #[redscript(name = "GetNPCType")]
    pub fn get_npc_type(self: &Ref<Self>) -> NPCType;
    // public final const func IsAndroid() -> Bool
    pub fn is_android(self: &Ref<Self>) -> bool;
    /// public final const func IsMech() -> Bool
    pub fn is_mech(self: &Ref<Self>) -> bool;
    /// public final const func IsHuman() -> Bool
    pub fn is_human(self: &Ref<Self>) -> bool;
    /// public final const func IsCerberus() -> Bool
    pub fn is_cerberus(self: &Ref<Self>) -> bool;
    /// public final const func IsHumanoid() -> Bool
    pub fn is_humanoid(self: &Ref<Self>) -> bool;
    /// public final const func IsMechanical() -> Bool
    pub fn is_mechanical(self: &Ref<Self>) -> bool;
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(i64)]
pub enum NPCType {
    Android = 0,
    Any = 1,
    Cerberus = 2,
    Chimera = 3,
    Device = 4,
    Drone = 5,
    Human = 6,
    Mech = 7,
    Spiderbot = 8,
    Count = 9,
    #[default]
    Invalid = 10,
}

unsafe impl NativeRepr for NPCType {
    const NAME: &'static str = "gamedataNPCType";
}

#[derive(Debug)]
pub struct Device;

impl ClassType for Device {
    type BaseClass = GameObject;

    const NAME: &'static str = "Device";
}

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(
            std::mem::size_of::<super::EntityGameInterface>(),
            0x10
        );
    }
}

impl_safe_downcast!(Entity, GameObject);
impl_safe_downcast!(Entity, Device);
impl_safe_downcast!(Entity, ScriptedPuppet);
