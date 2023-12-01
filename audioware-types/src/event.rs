use red4ext_rs::types::Ref;
use red4ext_rs::{conv::ClassType, macros::redscript_import, types::CName};

#[repr(transparent)]
pub struct IScriptable(red4ext_rs::types::IScriptable);

impl ClassType for IScriptable {
    type BaseClass = red4ext_rs::types::IScriptable;

    const NAME: &'static str = red4ext_rs::types::IScriptable::NAME;
    const NATIVE_NAME: &'static str = red4ext_rs::types::IScriptable::NATIVE_NAME;
}

#[redscript_import]
impl IScriptable {
    /// public native func GetClassName() -> CName
    #[redscript(native)]
    pub fn get_class_name(self: &Ref<Self>) -> CName;
}

#[derive(Debug)]
pub struct Event;

impl ClassType for Event {
    type BaseClass = IScriptable;
    const NAME: &'static str = "redEvent";
}

impl Event {
    pub fn get_class_name(self: &Ref<Self>) -> CName {
        red4ext_rs::prelude::Ref::<Event>::upcast(self.clone()).get_class_name()
    }
}
