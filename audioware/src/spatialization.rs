use red4ext_rs::{class_kind::{Native, Scripted}, types::{IScriptable, ScriptableSystem}, NativeRepr, ScriptClass};

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

#[derive(Debug, Default, Clone)]
#[repr(transparent)]
pub struct AudiowareSystem(ExtSystem);

unsafe impl NativeRepr for AudiowareSystem {
    const NAME: &'static str = "Audioware.AudiowareSystem";
}
unsafe impl ScriptClass for AudiowareSystem {
    type Kind = Scripted;
    const NAME: &'static str = <Self as NativeRepr>::NAME;
}

impl AsRef<ScriptableSystem> for AudiowareSystem {
    fn as_ref(&self) -> &ScriptableSystem {
        &self.0.0
    }
}

impl AsRef<IScriptable> for AudiowareSystem {
    fn as_ref(&self) -> &IScriptable {
        self.0.as_ref()
    }
}
