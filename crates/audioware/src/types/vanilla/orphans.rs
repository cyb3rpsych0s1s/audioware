use red4ext_rs::{
    NativeRepr, RttiSystem,
    types::{CName, GameInstance, RedArray, Ref},
};

use super::PlayerPuppet;

/// `public static func GetPlayer(gameInstance: GameInstance) -> ref<PlayerPuppet>`
///
/// Returns a null `Ref<PlayerPuppet>` if the global `GetPlayer;GameInstance`
/// function cannot be resolved or invoked, or if the player entity is not yet
/// attached to the game instance. Callers should null-check the result before
/// dereferencing — for example, `Scene::sync_listener` and `Scene::try_new`
/// both rely on this to avoid crashing the engine thread (and the host game
/// process) when the player is briefly unavailable during load transitions.
pub fn get_player(game: GameInstance) -> Ref<PlayerPuppet> {
    let rtti = RttiSystem::get();
    let methods = rtti.get_global_functions();
    let Some(method) = methods
        .iter()
        .find(|x| x.name() == CName::new("GetPlayer;GameInstance"))
    else {
        return Ref::default();
    };
    match method.execute::<_, Ref<PlayerPuppet>>(None, (game,)) {
        Ok(player) => player,
        Err(_) => Ref::default(),
    }
}

#[repr(C, align(8))]
pub struct WorldRuntimeScene {
    pub _padding0: [u8; 0x4B8],
}

unsafe impl NativeRepr for WorldRuntimeScene {
    const NAME: &'static str = "worldRuntimeScene";
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

#[repr(C, align(8))]
pub struct GamePlayerSocket {
    pub _padding0: [u8; 0x28],
}

unsafe impl NativeRepr for GamePlayerSocket {
    const NAME: &'static str = "gamePlayerSocket";
}

#[repr(C, align(8))]
pub struct RedTagList {
    pub tags: RedArray<CName>, // 0x0
}

unsafe impl NativeRepr for RedTagList {
    const NAME: &'static str = "redTagList";
}

impl std::fmt::Display for RedTagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.tags
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
