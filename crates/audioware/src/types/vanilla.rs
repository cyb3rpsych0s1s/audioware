//! Interop types for Cyberpunk 2077: see [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).

#![allow(dead_code)]

use red4ext_rs::{
    NativeRepr, RttiSystem, ScriptClass,
    types::{CName, EntityId, GameEngine, GameInstance, Ref},
};

mod audio_system;
pub use audio_system::*;
mod device;
pub use device::*;
mod entity;
pub use entity::*;
mod events;
pub use events::*;
mod helper;
pub use helper::*;
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
mod maths;
pub use maths::*;
mod scene_system;
pub use scene_system::*;
mod sound_component;
pub use sound_component::*;
mod time_dilatable;
pub use time_dilatable::*;
mod world_position;
pub use world_position::*;
mod world_transform;
pub use world_transform::*;

pub trait AsGameInstance {
    /// `public static native func FindEntityByID(self: GameInstance, entityId: EntityID) -> ref<Entity>`
    fn find_entity_by_id(game: GameInstance, entity_id: EntityId) -> Ref<Entity>;
    fn get_audio_system() -> Ref<AudioSystem>;
    fn get_scene_system() -> Ref<SceneSystem>;
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

    fn get_audio_system() -> Ref<AudioSystem> {
        let rtti = RttiSystem::get();
        let class = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
        let engine = GameEngine::get();
        let game = engine.game_instance();
        game.get_system(class.as_type())
            .cast::<AudioSystem>()
            .unwrap_or_default()
    }

    fn get_scene_system() -> Ref<SceneSystem> {
        let rtti = RttiSystem::get();
        let class = rtti.get_class(CName::new(SceneSystem::NAME)).unwrap();
        let engine = GameEngine::get();
        let game = engine.game_instance();
        game.get_system(class.as_type())
            .cast::<SceneSystem>()
            .unwrap_or_default()
    }
}
