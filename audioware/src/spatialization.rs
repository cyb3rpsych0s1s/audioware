use red4ext_rs::{class_kind::{Native, Scripted}, types::ScriptableSystem, ScriptClass};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct SpatializationSystem {
    base: ScriptableSystem,
}

unsafe impl ScriptClass for SpatializationSystem {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.SpatializationSystem";
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ExtSystem {
    base: SpatializationSystem,
}

unsafe impl ScriptClass for ExtSystem {
    type Kind = Native;
    const NAME: &'static str = "Audioware.ExtSystem";
}
