use red4ext_rs::{
    conv::ClassType,
    macros::redscript_import,
    types::{CName, Ref},
};

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
