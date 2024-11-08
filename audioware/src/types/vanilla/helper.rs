use red4ext_rs::{
    class_kind::Scripted,
    types::{CName, IScriptable, Ref},
    RttiSystem, ScriptClass,
};

use super::GameObject;

#[derive(Debug)]
pub struct AIActionHelper {
    base: IScriptable,
}

unsafe impl ScriptClass for AIActionHelper {
    type Kind = Scripted;
    const NAME: &'static str = "AIActionHelper";
}

impl AIActionHelper {
    pub fn is_in_workspot(owner: Ref<GameObject>) -> bool {
        let rtti = RttiSystem::get();
        let methods = rtti.get_global_functions();
        let method = methods
            .iter()
            .find(|x| x.name() == CName::new("AIActionHelper::IsInWorkspot;GameObject"))
            .unwrap();
        method.execute::<_, bool>(None, (owner,)).unwrap_or(false)
    }
}
