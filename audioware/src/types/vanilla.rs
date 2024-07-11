use std::fmt::Debug;

use red4rs::types::{Native, ScriptClass};

#[repr(transparent)]
pub struct ScriptableSystem(red4rs_bindings::ScriptableSystem);

impl Debug for ScriptableSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ScriptableSystem")
            .field(&self.0._padding0)
            .finish()
    }
}

impl Default for ScriptableSystem {
    fn default() -> Self {
        Self(red4rs_bindings::ScriptableSystem {
            _padding0: [0; 0x530],
        })
    }
}

impl Clone for ScriptableSystem {
    fn clone(&self) -> Self {
        Self(red4rs_bindings::ScriptableSystem {
            _padding0: self.0._padding0,
        })
    }
}

unsafe impl ScriptClass for ScriptableSystem {
    const CLASS_NAME: &'static str = "gameScriptableSystem";
    type Kind = Native;
}
