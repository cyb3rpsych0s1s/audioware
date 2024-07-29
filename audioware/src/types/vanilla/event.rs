use red4ext_rs::{class_kind::Native, types::IScriptable, ScriptClass};

#[derive(Debug)]
#[repr(transparent)]
pub struct Event(IScriptable);

unsafe impl ScriptClass for Event {
    type Kind = Native;
    const NAME: &'static str = "redEvent";
}

impl AsRef<IScriptable> for Event {
    fn as_ref(&self) -> &IScriptable {
        &self.0
    }
}
