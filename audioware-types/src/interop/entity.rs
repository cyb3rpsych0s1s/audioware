use red4ext_rs::{
    conv::ClassType,
    macros::redscript_import,
    types::{CName, Ref},
};

use super::iscriptable::IScriptable;

#[derive(Debug)]
pub struct Entity;

impl ClassType for Entity {
    type BaseClass = IScriptable;
    const NAME: &'static str = "Entity";
    const NATIVE_NAME: &'static str = "entEntity";
}

#[redscript_import]
impl Entity {
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool;
}

impl Entity {
    pub fn is_player(self: &Ref<Self>) -> bool {
        self.is_exactly_a(CName::new("PlayerPuppet"))
    }
}
