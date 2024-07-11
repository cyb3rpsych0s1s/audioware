use red4rs::types::{Native, ScriptClass};

use crate::types::ScriptableSystem;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudiowareSystem {
    base: ScriptableSystem,
}

unsafe impl ScriptClass for AudiowareSystem {
    type Kind = Native;
    const CLASS_NAME: &'static str = "Audioware.AudiowareSystem";
}
