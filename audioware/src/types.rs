use red4rs::types::{Native, ScriptClass};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ScriptableSystem {
    pub _padding0: [u8; 0x530],
}

impl Default for ScriptableSystem {
    fn default() -> Self {
        Self {
            _padding0: [0; 0x530],
        }
    }
}

unsafe impl ScriptClass for ScriptableSystem {
    const CLASS_NAME: &'static str = "gameScriptableSystem";
    type Kind = Native;
}
