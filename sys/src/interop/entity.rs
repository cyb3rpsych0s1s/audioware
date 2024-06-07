use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    macros::{redscript_global, redscript_import},
    types::{CName, EntityId, MaybeUninitRef, Ref, ScriptRef, Variant, VariantExt},
};
use snafu::OptionExt;

use crate::{
    error::{FromVariantSnafu, ReflectionError, UnknownClassSnafu, UnknownStaticFuncSnafu},
    interop::codeware::reflection::Reflection,
};

use super::{
    game::{GameInstance, PlayerPuppet},
    icomponent::IComponent,
    iscriptable::IScriptable,
    quaternion::Quaternion,
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
        self.is_exactly_a(CName::new(PlayerPuppet::NATIVE_NAME))
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
