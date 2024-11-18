use red4ext_rs::{
    types::{IScriptable, StackFrame},
    ScriptClass,
};

use crate::{utils::lifecycle, Entity};

use super::NativeFunc;

pub struct Dispose;

impl NativeFunc<{ super::offsets::ENTITY_DISPOSE }> for Dispose {
    #[inline(always)]
    fn detour(this: *mut IScriptable, frame: &mut StackFrame) -> Option<&mut StackFrame> {
        let x = unsafe { &*this };
        if x.class().name().as_str() == Entity::NAME {
            let x = unsafe { std::mem::transmute::<&IScriptable, &Entity>(x) };
            lifecycle!("dispose {:?}", x.entity_id);
        }
        Some(frame)
    }

    #[cfg(debug_assertions)]
    fn name() -> &'static str {
        "Entity::Dispose"
    }
}
