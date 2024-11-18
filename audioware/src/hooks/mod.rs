use std::ffi::c_void;

use red4ext_rs::{
    addr_hashes, hooks,
    types::{IScriptable, StackArgsState, StackFrame},
    SdkEnv, VoidPtr,
};

mod entity;
mod time_dilatable;

pub fn attach(env: &SdkEnv) {
    entity::Dispose::attach(env);
    time_dilatable::SetIndividualTimeDilation::attach(env);
    time_dilatable::UnsetIndividualTimeDilation::attach(env);
}

#[rustfmt::skip]
#[doc(hidden)]
mod offsets {
    pub const ENTITY_DISPOSE: u32                               = 0x3221A80;    // 0x14232C744 (2.13)
    pub const TIMEDILATABLE_SETINDIVIDUALTIMEDILATION: u32      = 0x80102488;   // 0x1423AF554 (2.13)
    pub const TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION: u32    = 0xDA20256B;   // 0x14147B424 (2.13)
}

pub type NativeFuncHook = *mut red4ext_rs::Hook<
    unsafe extern "C" fn(*mut IScriptable, *mut StackFrame, *mut c_void, *mut c_void),
    unsafe extern "C" fn(
        *mut IScriptable,
        *mut StackFrame,
        *mut c_void,
        *mut c_void,
        unsafe extern "C" fn(*mut IScriptable, *mut StackFrame, *mut c_void, *mut c_void),
    ),
>;

pub trait NativeFunc<const OFFSET: u32> {
    fn storage() -> NativeFuncHook {
        hooks! {
           static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
        }
        unsafe { HOOK }
    }
    #[allow(clippy::missing_transmute_annotations)]
    fn attach(env: &SdkEnv) {
        let addr = addr_hashes::resolve(OFFSET);
        let addr = unsafe { std::mem::transmute(addr) };
        unsafe {
            env.attach_hook(
                <Self as NativeFunc<OFFSET>>::storage(),
                addr,
                <Self as NativeFunc<OFFSET>>::hook,
            )
        };
    }
    unsafe extern "C" fn hook(
        i: *mut IScriptable,
        f: *mut StackFrame,
        a3: VoidPtr,
        a4: VoidPtr,
        cb: unsafe extern "C" fn(
            i: *mut IScriptable,
            f: *mut StackFrame,
            a3: VoidPtr,
            a4: VoidPtr,
        ) -> (),
    ) {
        let frame = &mut *f;
        let state = frame.args_state();
        if let Some(state) = <Self as NativeFunc<OFFSET>>::detour(i, frame, state) {
            frame.restore_args(state);
            cb(i, f, a3, a4);
        }
    }
    fn detour(
        this: *mut IScriptable,
        frame: &mut StackFrame,
        state: StackArgsState,
    ) -> Option<StackArgsState>;
}
