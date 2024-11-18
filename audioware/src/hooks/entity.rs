use red4ext_rs::{
    types::{IScriptable, StackArgsState, StackFrame},
    ScriptClass,
};

use crate::{utils::lifecycle, Entity};

use super::NativeFunc;

#[allow(non_camel_case_types)]
pub struct Dispose;

impl NativeFunc<{ super::offsets::ENTITY_DISPOSE }> for Dispose {
    fn detour(
        this: *mut IScriptable,
        _: &mut StackFrame,
        state: StackArgsState,
    ) -> Option<StackArgsState> {
        if !this.is_null() {
            let x = unsafe { &*this };
            if x.class().name().as_str() == Entity::NAME {
                let x = unsafe { std::mem::transmute::<&IScriptable, &Entity>(x) };
                lifecycle!("dispose {:?}", x.entity_id);
            }
        }
        Some(state)
    }
}
