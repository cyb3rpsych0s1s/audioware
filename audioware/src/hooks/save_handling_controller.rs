use red4ext_rs::{
    types::{IScriptable, StackFrame},
    VoidPtr,
};

use crate::{attach_native_func, utils::intercept};

attach_native_func!(
    "gameuiSaveHandlingController::LoadSaveInGame/LoadModdedSave",
    super::offsets::SAVEHANDLINGCONTROLLER_LOAD_SAVE_IN_GAME
);

unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let frame = &mut *f;
    let state = frame.args_state();

    let save_id: i32 = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);
    intercept!("gameuiSaveHandlingController::LoadSaveInGame/LoadModdedSave: {save_id}");
    cb(i, frame as *mut _, a3, a4);
}
