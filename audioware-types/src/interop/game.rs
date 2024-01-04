use std::ffi::c_void;

use red4ext_rs::{
    conv::NativeRepr,
    macros::{redscript_global, redscript_import},
    types::{EntityId, MaybeUninitRef},
};

use super::entity::Entity;

/// public static native GetGameInstance(): GameInstance
#[redscript_global(native)]
pub fn get_game_instance() -> GameInstance;

#[derive(Clone)]
#[repr(C)]
pub struct GameInstance {
    instance: *mut c_void,
    unk8: i8,
    unk10: i64,
}

#[cfg(test)]
static_assertions::const_assert_eq!(std::mem::size_of::<GameInstance>(), 0x18);

impl Default for GameInstance {
    fn default() -> Self {
        Self {
            instance: std::ptr::null_mut(),
            unk8: 0,
            unk10: 0,
        }
    }
}

unsafe impl NativeRepr for GameInstance {
    const NAME: &'static str = "GameInstance";
    const NATIVE_NAME: &'static str = "ScriptGameInstance";
}

#[redscript_import]
impl GameInstance {
    /// public static native func FindEntityByID(self: GameInstance, entityId: EntityID) -> Entity
    pub fn find_entity_by_id(this: Self, entity_id: EntityId) -> MaybeUninitRef<Entity>;
}
