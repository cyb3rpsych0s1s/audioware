use red4ext_rs::conv::ClassType;

use super::iscriptable::IScriptable;

#[derive(Debug)]
pub struct IComponent;

impl ClassType for IComponent {
    type BaseClass = IScriptable;

    const NAME: &'static str = "IComponent";
    const NATIVE_NAME: &'static str = "entIComponent";
}
