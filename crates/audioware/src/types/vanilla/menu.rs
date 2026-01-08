use red4ext_rs::{ScriptClass, class_kind::Native, types::IScriptable};

#[repr(C)]
pub struct InkMenuScenario {
    pub base: IScriptable,
    pub _padding0: [u8; 0x20],
}

unsafe impl ScriptClass for InkMenuScenario {
    const NAME: &'static str = "inkMenuScenario";
    type Kind = Native;
}

impl AsRef<IScriptable> for InkMenuScenario {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}
