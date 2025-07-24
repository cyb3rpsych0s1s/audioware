use red4ext_rs::{
    VoidPtr,
    types::{IScriptable, StackFrame},
};

use crate::{abi::lifecycle::Lifecycle, attach_native_func, engine::queue, utils::intercept};

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
    unsafe {
        let frame = &mut *f;
        let state = frame.args_state();

        let save_id: i32 = StackFrame::get_arg(frame);
        frame.restore_args(state);
        intercept!("gameuiSaveHandlingController::LoadSaveInGame/LoadModdedSave: {save_id}");
        queue::notify(Lifecycle::LoadSave);
        cb(i, frame as *mut _, a3, a4);
    }
}
