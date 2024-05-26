use retour::RawDetour;
use widestring::U16CString;
use winapi::{shared::minwindef::HMODULE, um::libloaderapi::GetModuleHandleW};

use crate::{
    ExternFnRedRegisteredFunc, ExternFnRedRegisteredHandler, LocalFnRustRegisteredFunc,
    LocalFnRustRegisteredHandler,
};

/// get a Windows module address in memory.
///
/// # Safety
/// module must be a valid process.
pub unsafe fn get_module(module: &str) -> Option<HMODULE> {
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
    #[cfg(debug_assertions)]
    {
        ::red4ext_rs::info!("base address:       0x{base:X}"); //            e.g. 0x7FF6C51B0000
        ::red4ext_rs::info!("relative address:   0x{offset:X}"); //          e.g. 0x1419130
        ::red4ext_rs::info!("calculated address: 0x{:X}", base + offset); // e.g. 0x7FF6C65C9130
    }
    base + offset
}

/// load a hook for a native event handler
///
/// # Safety
/// memory offset must be for a valid native event handler function.
pub unsafe fn load_native_event_handler(
    offset: usize,
    hook: LocalFnRustRegisteredHandler,
) -> Result<RawDetour, ::retour::Error> {
    let address = unsafe { self::locate(offset) };
    let vanilla: ExternFnRedRegisteredHandler = unsafe { ::std::mem::transmute(address) };
    unsafe { ::retour::RawDetour::new(vanilla as *const (), hook as *const ()) }
}

/// load a hook for a native function
///
/// # Safety
/// memory offset must be for a valid native function.
pub unsafe fn load_native_func(
    offset: usize,
    hook: LocalFnRustRegisteredFunc,
) -> Result<RawDetour, ::retour::Error> {
    let address = unsafe { self::locate(offset) };
    let vanilla: ExternFnRedRegisteredFunc = unsafe { ::std::mem::transmute(address) };
    unsafe { ::retour::RawDetour::new(vanilla as *const (), hook as *const ()) }
}
