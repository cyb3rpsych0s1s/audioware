use std::{cell::Cell, sync::OnceLock};

use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable},
};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicSoundEvent {
    base: IScriptable,
    pub(crate) id: OnceLock<usize>,
    pub(crate) name: Cell<CName>,
}

unsafe impl ScriptClass for DynamicSoundEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicSoundEvent";
}

impl AsRef<IScriptable> for DynamicSoundEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}
