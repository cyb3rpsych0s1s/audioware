pub mod frame;
mod hook;
mod module;

pub use module::*;

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

pub type LocalFnRustRegisteredFunc = unsafe fn(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) -> ();

/// define function requirements for detouring.
///
/// # Safety
/// - `offset` must point to valid function in binary
/// - `Inputs` must list function's parameters with correct type in the right order
pub unsafe trait Detour {
    const OFFSET: usize;
    type Inputs;
    /// read function parameters from `C` stack frame.
    ///
    /// # Safety
    /// - memory representation must be valid for each parameter
    /// - parameters must be read in order
    /// - stack must not be further manipulated
    unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs;
}

pub type Trampoline = fn(Box<dyn Fn(&RawDetour)>);

/// define `native function` detouring.
///
/// e.g. [AudioSystem::Play](https://jac3km4.github.io/cyberdoc/#33326)
pub trait NativeFunc: Detour {
    const HOOK: fn(Self::Inputs) -> ();
    const CONDITION: fn(&Self::Inputs) -> bool;
    const STORE: fn(Option<RawDetour>);
    const TRAMPOLINE: Trampoline;
    /// runtime hook.
    ///
    /// # Safety
    /// This function is safe as long as safety invariants for [`crate::Detour`] are upheld.
    ///
    /// Extra care must be taken if you manipulate the stack.
    unsafe fn hook(
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
            Self::TRAMPOLINE(Box::new(trampoline));
        } else {
            Self::HOOK(inputs);
        }
    }
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
                    Self::STORE(Some(detour));
                }
                Err(e) => {
                    ::red4ext_rs::error!("could not enable detour ({e})");
                }
            },
            Err(e) => {
                ::red4ext_rs::error!("could not initialize detour ({e})");
            }
        }
    }

    fn unload()
    where
        Self: Sized,
    {
        Self::STORE(None);
    }
}
