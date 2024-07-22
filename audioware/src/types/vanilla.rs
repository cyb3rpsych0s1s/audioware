#![allow(dead_code)]

use red4ext_rs::{
    types::{CName, EntityId, GameInstance, IScriptable, Ref},
    NativeRepr, RttiSystem, ScriptClass,
};

mod audio_system;
pub use audio_system::*;
mod entity;
pub use entity::*;
mod fixed_point;
pub use fixed_point::*;
mod game_object;
pub use game_object::*;
mod orphans;
pub use orphans::*;
mod quaternion;
pub use quaternion::*;
mod vector4;
pub use vector4::*;
mod world_position;
pub use world_position::*;
mod world_transform;
pub use world_transform::*;

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
