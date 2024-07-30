#![allow(dead_code)]

use red4ext_rs::{
    types::{CName, EntityId, GameEngine, GameInstance, Ref},
    NativeRepr, RttiSystem, ScriptClass,
};

mod audio_system;
pub use audio_system::*;
mod dialog_line;
pub use dialog_line::*;
mod entity;
pub use entity::*;
mod emitter_event;
pub use emitter_event::*;
mod event;
pub use event::*;
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
mod sound_play_vo;
pub use sound_play_vo::*;
mod vector4;
pub use vector4::*;
mod voice;
pub use voice::*;
mod world_position;
pub use world_position::*;
mod world_transform;
pub use world_transform::*;

pub trait AsGameInstance {
    /// `public static native func FindEntityByID(self: GameInstance, entityId: EntityID) -> ref<Entity>`
    fn find_entity_by_id(game: GameInstance, entity_id: EntityId) -> Ref<Entity>;
    fn get_audio_system() -> Ref<AudioSystem>;
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
}
