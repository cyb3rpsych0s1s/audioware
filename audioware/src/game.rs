use audioware_types::event::IScriptable;
use red4ext_rs::types::Ref;
use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    macros::redscript_import,
    types::{CName, MaybeUninitRef},
};
use std::ffi::c_void;

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

#[redscript_import]
impl GameInstance {
    /// public static native GetScriptableSystemsContainer(self: GameInstance): ScriptableSystemsContainer
    #[redscript(native)]
    pub fn get_scriptable_systems_container(this: Self) -> Ref<ScriptableSystemsContainer>;
}

#[derive(Debug)]
pub struct ScriptableSystemsContainer;

impl ClassType for ScriptableSystemsContainer {
    type BaseClass = IScriptable;

    const NAME: &'static str = "ScriptableSystemsContainer";
    const NATIVE_NAME: &'static str = "gameScriptableSystemsContainer";
}

#[redscript_import]
impl ScriptableSystemsContainer {
    /// public native Get(systemName: CName): ScriptableSystem
    #[redscript(native)]
    pub fn get(self: &Ref<Self>, system_name: CName) -> MaybeUninitRef<ScriptableSystem>;
}

#[derive(Debug)]
pub struct ScriptableSystem;

impl ClassType for ScriptableSystem {
    type BaseClass = IScriptable;

    const NAME: &'static str = "ScriptableSystem";
    const NATIVE_NAME: &'static str = "gameScriptableSystem";
}
