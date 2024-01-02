use std::borrow::BorrowMut;
use std::ops::Not;

use red4ext_rs::types::{CName, EntityId};
use widestring::U16CString;
use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::GetModuleHandleW;

pub trait Hook {
    fn load()
    where
        Self: Sized;
    fn unload()
    where
        Self: Sized;
}

macro_rules! make_hook {
    ($name:ident, $address:ident, $fn_ty:ty, $hook:expr, $storage:ident) => {
        ::lazy_static::lazy_static! {
            static ref $storage: ::std::sync::Arc<::std::sync::Mutex<::std::option::Option<::retour::RawDetour>>> =
                ::std::sync::Arc::new(::std::sync::Mutex::new(None));
        }
        pub struct $name;
        impl Hook for $name {
            fn load() {
                let relative: usize = $crate::addresses::$address;
                unsafe {
                    let base: usize = self::get_module("Cyberpunk2077.exe").unwrap() as usize;
                    let address = base + relative;
                    ::red4ext_rs::debug!(
                        "[{}] base address:       0x{base:X}",
                        ::std::stringify! {$name}
                    ); // e.g. 0x7FF6C51B0000
                    ::red4ext_rs::debug!(
                        "[{}] relative address:   0x{relative:X}",
                        ::std::stringify! {$name}
                    ); // e.g. 0x1419130
                    ::red4ext_rs::debug!(
                        "[{}] calculated address: 0x{address:X}",
                        ::std::stringify! {$name}
                    ); // e.g. 0x7FF6C65C9130
                    let target: $fn_ty = ::std::mem::transmute(address);
                    match ::retour::RawDetour::new(target as *const (), $hook as *const ()) {
                        Ok(detour) => match detour.enable() {
                            Ok(_) => {
                                if let Ok(mut guard) = $storage.clone().borrow_mut().try_lock() {
                                    *guard = Some(detour);
                                } else {
                                    ::red4ext_rs::error!("could not store detour");
                                }
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
            }
            fn unload() {
                let _ = $storage
                .clone()
                .borrow_mut()
                .lock()
                .unwrap()
                .take();
            }
        }
    };
}

pub type ExternFnRedRegisteredFunc = unsafe extern "C" fn(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) -> ();

pub fn on_audiosystem_play(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) {
    let rewind = unsafe { (*frame.cast::<crate::frame::StackFrame>()).code };
    // read stack frame
    let mut event_name: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut event_name)) };
    let mut entity_id: EntityId = EntityId::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut entity_id)) };
    let mut emitter_name: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut emitter_name)) };
    // compare event name
    if is_vanilla(event_name.clone()) {
        if let Ok(ref guard) = HOOK_ON_AUDIOSYSTEM_PLAY.clone().try_lock() {
            if let Some(detour) = guard.as_ref() {
                // rewind the stack and call vanilla
                unsafe {
                    (*frame.cast::<crate::frame::StackFrame>()).code = rewind;
                    (*frame.cast::<crate::frame::StackFrame>()).currentParam = 0;
                }
                let original: ExternFnRedRegisteredFunc =
                    unsafe { std::mem::transmute(detour.trampoline()) };
                unsafe { original(ctx, frame, out, a4) };
            }
        }
    // if event name is not vanilla
    } else {
        // jump to custom func
        custom_engine_play(event_name, entity_id, emitter_name);
    }
}

pub fn on_audiosystem_stop(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) {
    let rewind = unsafe { (*frame.cast::<crate::frame::StackFrame>()).code };
    // read stack frame
    let mut event_name: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut event_name)) };
    let mut entity_id: EntityId = EntityId::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut entity_id)) };
    let mut emitter_name: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut emitter_name)) };
    // compare event name
    if is_vanilla(event_name.clone()) {
        if let Ok(ref guard) = HOOK_ON_AUDIOSYSTEM_STOP.clone().try_lock() {
            if let Some(detour) = guard.as_ref() {
                // rewind the stack and call vanilla
                unsafe {
                    (*frame.cast::<crate::frame::StackFrame>()).code = rewind;
                    (*frame.cast::<crate::frame::StackFrame>()).currentParam = 0;
                }
                let original: ExternFnRedRegisteredFunc =
                    unsafe { std::mem::transmute(detour.trampoline()) };
                unsafe { original(ctx, frame, out, a4) };
            }
        }
    // if event name is not vanilla
    } else {
        // jump to custom func
        custom_engine_stop(event_name, entity_id, emitter_name);
    }
}

pub fn on_audiosystem_switch(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) {
    // switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName
    let rewind = unsafe { (*frame.cast::<crate::frame::StackFrame>()).code };
    // read stack frame
    let mut switch_name: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut switch_name)) };
    let mut switch_value: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut switch_value)) };
    let mut entity_id: EntityId = EntityId::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut entity_id)) };
    let mut emitter_name: CName = CName::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut emitter_name)) };
    // red4ext_rs::info!(
    //     "AudioSystem.switch: switch_name {}, switch_value {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&switch_name),
    //     red4ext_rs::ffi::resolve_cname(&switch_value),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    if let Ok(ref guard) = HOOK_ON_AUDIOSYSTEM_SWITCH.clone().try_lock() {
        if let Some(detour) = guard.as_ref() {
            // rewind the stack and call vanilla
            unsafe {
                (*frame.cast::<crate::frame::StackFrame>()).code = rewind;
                (*frame.cast::<crate::frame::StackFrame>()).currentParam = 0;
            }
            let original: ExternFnRedRegisteredFunc =
                unsafe { std::mem::transmute(detour.trampoline()) };
            unsafe { original(ctx, frame, out, a4) };
        }
    }
}

pub fn is_vanilla(event_name: CName) -> bool {
    if let Ok(exists) = crate::engine::banks::exists(event_name.clone()) {
        // red4ext_rs::info!("sound {event_name} is vanilla ? {}", !exists);
        return !exists;
    } else {
        red4ext_rs::error!("unable to find sound {event_name} existence in banks");
    }
    true
}

pub fn custom_engine_play(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    // red4ext_rs::info!(
    //     "call custom engine Play method with: event_name {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&event_name),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    let entity_id = if entity_id == EntityId::default() {
        None
    } else {
        Some(entity_id)
    };
    let emitter_name = if emitter_name == CName::default() {
        None
    } else {
        Some(emitter_name)
    };
    crate::engine::play(event_name, entity_id, emitter_name);
}

pub fn custom_engine_stop(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    // red4ext_rs::info!(
    //     "call custom engine Stop method with: event_name {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&event_name),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    let entity_id = if entity_id == EntityId::default() {
        None
    } else {
        Some(entity_id)
    };
    let emitter_name = if emitter_name == CName::default() {
        None
    } else {
        Some(emitter_name)
    };
    crate::engine::stop(event_name, entity_id, emitter_name);
}

make_hook!(
    HookAudioSystemPlay,
    ON_AUDIOSYSTEM_PLAY,
    ExternFnRedRegisteredFunc,
    on_audiosystem_play,
    HOOK_ON_AUDIOSYSTEM_PLAY
);

make_hook!(
    HookAudioSystemStop,
    ON_AUDIOSYSTEM_STOP,
    ExternFnRedRegisteredFunc,
    on_audiosystem_stop,
    HOOK_ON_AUDIOSYSTEM_STOP
);

make_hook!(
    HookAudioSystemSwitch,
    ON_AUDIOSYSTEM_SWITCH,
    ExternFnRedRegisteredFunc,
    on_audiosystem_switch,
    HOOK_ON_AUDIOSYSTEM_SWITCH
);

unsafe fn get_module(module: &str) -> Option<HMODULE> {
    let module = U16CString::from_str_truncate(module);
    let res = GetModuleHandleW(module.as_ptr());
    res.is_null().not().then_some(res)
}
