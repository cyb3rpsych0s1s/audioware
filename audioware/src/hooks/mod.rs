use std::{ffi::c_void, mem, ops::Not};

use red4ext_rs::{
    addr_hashes, hooks,
    types::{IScriptable, StackArgsState, StackFrame},
    SdkEnv, VoidPtr,
};

use crate::Event;

mod entity;
mod events;
mod time_dilatable;
mod time_system;

pub fn attach(env: &SdkEnv) {
    entity::Dispose::attach(env);
    time_system::SetTimeDilation::attach(env);
    time_dilatable::SetIndividualTimeDilation::attach(env);
    time_dilatable::UnsetIndividualTimeDilation::attach(env);

    // #[cfg(debug_assertions)]
    // {
    //     events::dialog_line::Handler::attach(env);
    //     events::dialog_line_end::Handler::attach(env);
    // }
}

#[rustfmt::skip]
#[doc(hidden)]
mod offsets {
    pub const ENTITY_DISPOSE: u32                               = 0x3221A80;    // 0x14232C744 (2.13)
    pub const TIMEDILATABLE_SETINDIVIDUALTIMEDILATION: u32      = 0x80102488;   // 0x1423AF554 (2.13)
    pub const TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION: u32    = 0xDA20256B;   // 0x14147B424 (2.13)
    pub const TIMESYSTEM_SETTIMEDILATION: u32                   = 0xA1DC1F92;   // 0x140A46EE4 (2.13)

    pub const EVENT_DIALOGLINE: u32                             = 0x10E71E89;   // 0x1409C12A8 (2.12a)
    pub const EVENT_DIALOGLINEEND: u32                          = 0x6F24331;    // 0x141188BF4 (2.12a)
}

#[allow(dead_code)]
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
    #[cfg(debug_assertions)]
    fn name() -> &'static str;
    #[allow(clippy::missing_transmute_annotations)]
    fn attach(env: &SdkEnv) {
        hooks! {
           static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
        }
        let addr = addr_hashes::resolve(OFFSET);
        let addr = unsafe { std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, <Self as NativeFunc<OFFSET>>::hook) };
        #[cfg(debug_assertions)]
        crate::utils::lifecycle!("attached hook for {}", Self::name());
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

pub type NativeHandlerHook = *mut red4ext_rs::Hook<
    unsafe extern "C" fn(*mut IScriptable, *mut Event),
    unsafe extern "C" fn(
        *mut IScriptable,
        *mut Event,
        unsafe extern "C" fn(*mut IScriptable, *mut Event),
    ),
>;

pub trait NativeHandler<const OFFSET: u32> {
    type EventClass;
    fn storage() -> NativeHandlerHook {
        hooks! {
           static HOOK: fn(a1: *mut IScriptable, a2: *mut Event) -> ();
        }
        unsafe { HOOK }
    }
    #[allow(clippy::missing_transmute_annotations)]
    fn attach(env: &SdkEnv) {
        let addr = addr_hashes::resolve(OFFSET);
        let addr = unsafe { mem::transmute(addr) };
        unsafe {
            env.attach_hook(
                <Self as NativeHandler<OFFSET>>::storage(),
                addr,
                <Self as NativeHandler<OFFSET>>::hook,
            )
        };
    }
    unsafe extern "C" fn hook(
        a1: *mut IScriptable,
        a2: *mut Event,
        cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut Event) -> (),
    ) {
        let this = a1.is_null().not().then_some(unsafe { &*a1 });
        let event = a2
            .is_null()
            .not()
            .then_some(unsafe { mem::transmute::<&mut Event, &mut Self::EventClass>(&mut *a2) });
        if let Some((this, event)) = this.zip(event) {
            if let Some(a2) = Self::detour(this, event) {
                cb(
                    a1,
                    mem::transmute::<&mut Self::EventClass, &mut Event>(a2) as *mut _,
                );
            }
        } else {
            cb(a1, a2);
        }
    }
    fn detour<'a>(
        this: &IScriptable,
        event: &'a mut Self::EventClass,
    ) -> Option<&'a mut Self::EventClass>;
}
