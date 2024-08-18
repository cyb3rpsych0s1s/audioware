use red4ext_rs::{class_kind::Native, types::{IScriptable, ScriptableSystem}, ScriptClass};

#[derive(Debug, Default, Clone)]
#[repr(transparent)]
pub struct ExtSystem(ScriptableSystem);

unsafe impl ScriptClass for ExtSystem {
    type Kind = Native;
    const NAME: &'static str = "Audioware.ExtSystem";
}

impl AsRef<ScriptableSystem> for ExtSystem {
    fn as_ref(&self) -> &ScriptableSystem {
        &self.0
    }
}

impl AsRef<IScriptable> for ExtSystem {
    fn as_ref(&self) -> &IScriptable {
        self.0.as_ref()
    }
}
