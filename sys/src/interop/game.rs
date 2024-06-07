use std::ffi::c_void;

use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    macros::{redscript_global, redscript_import},
    types::{CName, Ref},
};
use serde::Deserialize;

use crate::impl_safe_downcast;

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

#[derive(Debug)]
pub struct GameObject;

impl ClassType for GameObject {
    type BaseClass = Entity;
    const NAME: &'static str = "GameObject";
    const NATIVE_NAME: &'static str = "gameObject";
}

#[redscript_import]
impl GameObject {
    /// public const func IsDevice() -> Bool
    pub fn is_device(self: &Ref<Self>) -> bool;
    /// public const func IsPuppet() -> Bool
    pub fn is_puppet(self: &Ref<Self>) -> bool;
    /// public const func IsPlayer() -> Bool
    pub fn is_player(self: &Ref<Self>) -> bool;
}

#[derive(Debug)]
pub struct ScriptedPuppet;

impl ClassType for ScriptedPuppet {
    type BaseClass = GameObject;
    const NAME: &'static str = "ScriptedPuppet";
}

#[redscript_import]
impl ScriptedPuppet {
    /// public final const func GetGender() -> CName
    pub fn get_gender(self: &Ref<Self>) -> CName;
    /// public final const func GetNPCType() -> gamedataNPCType
    #[redscript(name = "GetNPCType")]
    pub fn get_npc_type(self: &Ref<Self>) -> NPCType;
    // public final const func IsAndroid() -> Bool
    pub fn is_android(self: &Ref<Self>) -> bool;
    /// public final const func IsMech() -> Bool
    pub fn is_mech(self: &Ref<Self>) -> bool;
    /// public final const func IsHuman() -> Bool
    pub fn is_human(self: &Ref<Self>) -> bool;
    /// public final const func IsCerberus() -> Bool
    pub fn is_cerberus(self: &Ref<Self>) -> bool;
    /// public final const func IsHumanoid() -> Bool
    pub fn is_humanoid(self: &Ref<Self>) -> bool;
    /// public final const func IsMechanical() -> Bool
    pub fn is_mechanical(self: &Ref<Self>) -> bool;
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    strum_macros::Display,
    strum_macros::FromRepr,
    PartialEq,
)]
#[repr(i64)]
pub enum NPCType {
    Android = 0,
    Any = 1,
    Cerberus = 2,
    Chimera = 3,
    Device = 4,
    Drone = 5,
    Human = 6,
    Mech = 7,
    Spiderbot = 8,
    Count = 9,
    #[default]
    Invalid = 10,
}

unsafe impl NativeRepr for NPCType {
    const NAME: &'static str = "gamedataNPCType";
}

#[derive(Debug)]
pub struct Device;

impl ClassType for Device {
    type BaseClass = GameObject;
    const NAME: &'static str = "Device";
}

#[derive(Debug)]
pub struct PlayerPuppet;

impl ClassType for PlayerPuppet {
    type BaseClass = ScriptedPuppet;
    const NAME: &'static str = "PlayerPuppet";
}

#[derive(Debug)]
pub struct WeaponObject;

impl ClassType for WeaponObject {
    type BaseClass = GameObject;
    const NAME: &'static str = "WeaponObject";
    const NATIVE_NAME: &'static str = "gameweaponObject";
}

#[derive(Debug)]
pub struct NPCPuppet;

impl ClassType for NPCPuppet {
    type BaseClass = ScriptedPuppet;
    const NAME: &'static str = "NPCPuppet";
}

impl_safe_downcast!(Entity, GameObject);
impl_safe_downcast!(Entity, Device);
impl_safe_downcast!(Entity, ScriptedPuppet);
impl_safe_downcast!(Entity, NPCPuppet);
impl_safe_downcast!(Entity, PlayerPuppet);

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(std::mem::size_of::<super::GameInstance>(), 0x18);
    }
}
