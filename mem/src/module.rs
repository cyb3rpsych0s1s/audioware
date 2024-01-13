use retour::RawDetour;
use widestring::U16CString;
use winapi::{shared::minwindef::HMODULE, um::libloaderapi::GetModuleHandleW};

/// get a Windows module address in memory.
///
/// # Safety
/// module must be a valid process.
unsafe fn get_module(module: &str) -> Option<HMODULE> {
    let module = U16CString::from_str_truncate(module);
    let res = GetModuleHandleW(module.as_ptr());
    std::ops::Not::not(res.is_null()).then_some(res)
}

/// locate memory address in Cyberpunk2077.exe
///
/// # Safety
/// Cyberpunk 2077 must be installed and this binary installed correctly
#[inline]
unsafe fn locate(offset: usize) -> usize {
    let base: usize = unsafe { self::get_module("Cyberpunk2077.exe").unwrap() as usize };
    base + offset
}

/// load a hook for a native event handler
///
/// # Safety
/// memory offset must be for a valid native event handler function.
pub unsafe fn load_native_event_handler(
    offset: usize,
    hook: fn(usize, usize) -> (),
) -> Result<RawDetour, ::retour::Error> {
    let address = unsafe { self::locate(offset) };
    let vanilla: extern "C" fn(usize, usize) -> () = unsafe { ::std::mem::transmute(address) };
    unsafe { ::retour::RawDetour::new(vanilla as *const (), hook as *const ()) }
}

/// load a hook for a native function
///
/// # Safety
/// memory offset must be for a valid native function.
pub unsafe fn load_native_func(
    offset: usize,
    hook: fn(
        *mut red4ext_rs::ffi::IScriptable,
        *mut red4ext_rs::ffi::CStackFrame,
        *mut std::ffi::c_void,
        i64,
    ) -> (),
) -> Result<RawDetour, ::retour::Error> {
    let address = unsafe { self::locate(offset) };
    let vanilla: extern "C" fn(
        *mut red4ext_rs::ffi::IScriptable,
        *mut red4ext_rs::ffi::CStackFrame,
        *mut std::ffi::c_void,
        i64,
    ) -> () = unsafe { ::std::mem::transmute(address) };
    unsafe { ::retour::RawDetour::new(vanilla as *const (), hook as *const ()) }
}
