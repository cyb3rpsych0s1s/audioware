#![allow(dead_code)]

use red4ext_rs::{
    types::{CName, EntityId, GameInstance, Ref},
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
mod iplaced_component;
mod orphans;
pub use orphans::*;
mod puppet;
pub use puppet::*;
mod quaternion;
pub use quaternion::*;
mod vector4;
pub use vector4::*;
mod world_position;
pub use world_position::*;
mod world_transform;
pub use world_transform::*;

pub trait AsGameInstance {
    /// `public static native func FindEntityByID(self: GameInstance, entityId: EntityID) -> ref<Entity>`
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
