//! Interop types for Cyberpunk 2077: see [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).

#![allow(dead_code)]

use red4ext_rs::{
    NativeRepr, RttiSystem, ScriptClass,
    types::{CName, EntityId, GameEngine, GameInstance, Ref},
};

mod audio_system;
pub use audio_system::*;
mod component;
pub use component::*;
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
mod orphans;
pub use orphans::*;
mod puppet;
pub use puppet::*;
mod quaternion;
pub use quaternion::*;
mod maths;
pub use maths::*;
mod sound;
pub use sound::*;
mod sound_engine;
pub use sound_engine::*;
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

use crate::{
    AsNativeEntitySystem, AsNativeSubSystem, DynamicEntitySystem, StaticEntitySystem,
    WorldStateSystem,
};

pub trait AsGameInstance {
    /// `public static native func FindEntityByID(self: GameInstance, entityId: EntityID) -> ref<Entity>`
    fn find_entity_by_id(game: GameInstance, entity_id: EntityId) -> Ref<Entity>;
    fn get_audio_system() -> Ref<AudioSystem>;
    fn get_scene_system() -> Ref<SceneSystem>;
    fn get_world_state_system() -> Ref<WorldStateSystem>;
    fn get_static_entity_system() -> Ref<StaticEntitySystem>;
    fn get_dynamic_entity_system() -> Ref<DynamicEntitySystem>;
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

    fn get_world_state_system() -> Ref<WorldStateSystem> {
        let rtti = RttiSystem::get();
        let class = rtti.get_class(CName::new(SceneSystem::NAME)).unwrap();
        let engine = GameEngine::get();
        let game = engine.game_instance();
        game.get_system(class.as_type())
            .cast::<WorldStateSystem>()
            .unwrap_or_default()
    }

    fn get_static_entity_system() -> Ref<StaticEntitySystem> {
        let rtti = RttiSystem::get();
        let class = rtti
            .get_class(CName::new(StaticEntitySystem::NAME))
            .unwrap();
        let engine = GameEngine::get();
        let game = engine.game_instance();
        game.get_system(class.as_type())
            .cast::<StaticEntitySystem>()
            .unwrap_or_default()
    }

    fn get_dynamic_entity_system() -> Ref<DynamicEntitySystem> {
        let rtti = RttiSystem::get();
        let class = rtti
            .get_class(CName::new(DynamicEntitySystem::NAME))
            .unwrap();
        let engine = GameEngine::get();
        let game = engine.game_instance();
        game.get_system(class.as_type())
            .cast::<DynamicEntitySystem>()
            .unwrap_or_default()
    }
}

pub fn resolve_any_entity(entity_id: EntityId) -> Ref<Entity> {
    let game = GameInstance::new();
    let entity = GameInstance::find_entity_by_id(game, entity_id);
    if !entity.is_null() {
        return entity;
    }
    let statics = GameInstance::get_static_entity_system();
    if statics.is_ready() {
        let entity = statics.get_entity(entity_id);
        if !entity.is_null() {
            return entity;
        }
    }
    let dynamics = GameInstance::get_dynamic_entity_system();
    if dynamics.is_ready() {
        let entity = dynamics.get_entity(entity_id);
        if !entity.is_null() {
            return entity;
        }
    }
    Ref::default()
}
