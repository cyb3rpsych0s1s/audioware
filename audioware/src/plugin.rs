use red4rs::{
    log,
    types::{IScriptable, Native, ScriptClass},
    PluginOps,
};

use crate::Audioware;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudiowarePlugin {
    base: IScriptable,
}

unsafe impl ScriptClass for AudiowarePlugin {
    type Kind = Native;
    const CLASS_NAME: &'static str = "Audioware.AudiowarePlugin";
}

impl AudiowarePlugin {
    pub fn yolo(&self) {
        let env = Audioware::env();
        log::info!(env, "yolo!");
    }
}
