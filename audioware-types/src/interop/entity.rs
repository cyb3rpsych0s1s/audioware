use red4ext_rs::{
    conv::ClassType,
    macros::redscript_import,
    types::{CName, Ref},
};

use super::{icomponent::IComponent, iscriptable::IScriptable, vector4::Vector4};

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
    /// public native func GetWorldPosition() -> Vector4
    #[redscript(native)]
    pub fn get_world_position(self: &Ref<Self>) -> Vector4;
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
