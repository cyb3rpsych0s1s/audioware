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
    /// public native func IsExactlyA(className: CName) -> Bool
    #[redscript(native)]
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool;
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
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool {
        red4ext_rs::prelude::Ref::<Event>::upcast(self.clone()).is_exactly_a(class_name)
    }
}
