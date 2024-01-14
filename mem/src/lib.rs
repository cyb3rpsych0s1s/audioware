pub mod frame;
mod hook;
mod module;

use std::sync::MutexGuard;

pub use module::*;
use red4ext_rs::types::{CName, EntityId};
use retour::RawDetour;

/// Read a struct directly from memory at given offset.
///
/// # Safety
/// this is only safe as long as it matches memory representation specified in [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).
///
/// It usually implies that:
/// - it must be annotated with `#[repr(C)]` to guarantee that the order of its fields will be preserved
/// - fields are defined in correct order
/// - padding is preserved
/// - fields type match underlying memory representation
pub unsafe trait FromMemory {
    fn from_memory(address: usize) -> Self;
}

/// hook function lifecycle
pub trait Hook {
    fn load()
    where
        Self: Sized;
    fn unload()
    where
        Self: Sized;
}

pub type ExternFnRedRegisteredFunc = unsafe extern "C" fn(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) -> ();

pub type LocalFnRustRegisteredFunc = fn(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) -> ();

pub trait NativeFunc {
    const OFFSET: usize;
    const HOOK: fn(Self::Inputs) -> ();
    const CONDITION: fn(&Self::Inputs) -> bool;
    //fn bar<F>(mut f: F) where F: MyTrait<i32>
    // const STORAGE: fn(MutexGuard<'_, RawDetour>);
    const STORAGE: fn(Box<dyn Fn(&RawDetour)>);
    type Inputs;
    fn hook(
        ctx: *mut red4ext_rs::ffi::IScriptable,
        frame: *mut red4ext_rs::ffi::CStackFrame,
        out: *mut std::ffi::c_void,
        a4: i64,
    ) {
        let rewind = unsafe { (*frame.cast::<frame::StackFrame>()).code };
        // read stack frame
        let inputs: Self::Inputs = unsafe { Self::from_frame(frame) };
        if !Self::CONDITION(&inputs) {
            let trampoline = move |detour: &RawDetour| {
                // rewind the stack and call vanilla
                unsafe {
                    (*frame.cast::<frame::StackFrame>()).code = rewind;
                    (*frame.cast::<frame::StackFrame>()).currentParam = 0;
                }
                let original: ExternFnRedRegisteredFunc =
                    unsafe { ::std::mem::transmute(detour.trampoline()) };
                unsafe { original(ctx, frame, out, a4) };
            };
            Self::STORAGE(Box::new(trampoline));
        } else {
        }
    }
    unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs;
    fn store(detour: RawDetour) {}
    fn trampoline(
        rewind: i64,
        detour: RawDetour,
        ctx: *mut red4ext_rs::ffi::IScriptable,
        frame: *mut red4ext_rs::ffi::CStackFrame,
        out: *mut std::ffi::c_void,
        a4: i64,
    ) {
        // rewind the stack and call vanilla
        unsafe {
            (*frame.cast::<frame::StackFrame>()).code = rewind;
            (*frame.cast::<frame::StackFrame>()).currentParam = 0;
        }
        let original: ExternFnRedRegisteredFunc =
            unsafe { ::std::mem::transmute(detour.trampoline()) };
        unsafe { original(ctx, frame, out, a4) };
    }
    fn clear() {}
}

#[repr(C)]
pub struct AudioSystemPlayParams(CName, EntityId, CName);
pub struct AudioSystemPlay;
impl NativeFunc for AudioSystemPlay {
    const OFFSET: usize = 0x123;
    const HOOK: fn(Self::Inputs) -> () = audiosystem_play;
    const CONDITION: fn(&Self::Inputs) -> bool = should_detour_audiosystem_play;
    const STORAGE: fn(Box<dyn Fn(&RawDetour)>) = storage_audiosystem_play;
    type Inputs = AudioSystemPlayParams;

    unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs {
        todo!()
    }
}
::lazy_static::lazy_static! {
    static ref AUDIOSYSTEM_PLAY_STORAGE: ::std::sync::Arc<::std::sync::Mutex<::std::option::Option<::retour::RawDetour>>> =
        ::std::sync::Arc::new(::std::sync::Mutex::new(None));
}
fn storage_audiosystem_play(closure: Box<dyn Fn(&RawDetour)>) {
    if let Ok(Some(guard)) = AUDIOSYSTEM_PLAY_STORAGE.clone().try_lock().as_deref() {
        closure(guard);
    }
}
fn audiosystem_play(params: AudioSystemPlayParams) {}
fn should_detour_audiosystem_play(params: &AudioSystemPlayParams) -> bool {
    false
}
impl<T> Hook for T
where
    T: NativeFunc,
{
    fn load()
    where
        Self: Sized,
    {
        match unsafe { load_native_func(Self::OFFSET, Self::hook) } {
            Ok(detour) => match unsafe { detour.enable() } {
                Ok(_) => {
                    Self::store(detour);
                }
                Err(e) => {}
            },
            Err(e) => {}
        }
    }

    fn unload()
    where
        Self: Sized,
    {
        Self::clear();
    }
}
