use red4ext_rs::types::{IScriptable, StackFrame};

use super::NativeFunc;

pub struct LoadSaveInGame;

impl NativeFunc<{ super::offsets::SAVEHANDLINGCONTROLLER_LOAD_SAVE_IN_GAME }> for LoadSaveInGame {
    #[inline(always)]
    fn detour(_: *mut IScriptable, frame: &mut StackFrame) -> Option<&mut StackFrame> {
        #[cfg(debug_assertions)]
        {
            let save_id: i32 = unsafe { StackFrame::get_arg(frame) };
            crate::utils::lifecycle!("{}: called ({save_id})", Self::name());
        }
        Some(frame)
    }

    #[cfg(debug_assertions)]
    fn name() -> &'static str {
        "gameuiSaveHandlingController.LoadSaveInGame/LoadModdedSave"
    }
}
