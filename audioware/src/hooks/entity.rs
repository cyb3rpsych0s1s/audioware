use std::{mem, ops::Not};

use red4ext_rs::{
    types::{IScriptable, StackFrame},
    VoidPtr,
};

use crate::{attach_native_func, utils::intercept, Entity};

attach_native_func!("Entity::Dispose", super::offsets::ENTITY_DISPOSE);

unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let x = i
        .is_null()
        .not()
        .then_some(&*i)
        .map(|x| mem::transmute::<&IScriptable, &Entity>(x))
        .map(|x| x.entity_id);
    intercept!("called Entity::Dispose ({x:?})");
    cb(i, f, a3, a4);
}
