use std::borrow::BorrowMut;
use std::ops::Not;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use retour::RawDetour;
use widestring::U16CString;
use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::{addresses::ON_ENT_AUDIO_EVENT, interop::AudioEvent, FromMemory};

macro_rules! make_hook {
    ($name:ident, $address:expr, $hook:expr, $storage:expr) => {
        pub(crate) fn $name() -> ::anyhow::Result<()> {
            let relative: usize = $address;
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
                let target: ExternFnRedEventHandler = ::std::mem::transmute(address);
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
            Ok(())
        }
    };
}

pub(crate) type ExternFnRedEventHandler = unsafe extern "C" fn(usize, usize) -> ();

lazy_static! {
    pub(crate) static ref HOOK_ON_ENT_AUDIO_EVENT: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
}

pub fn on_audio_event(o: usize, a: usize) {
    red4ext_rs::trace!("[on_audio_event] hooked");
    if let Ok(ref guard) = HOOK_ON_ENT_AUDIO_EVENT.clone().try_lock() {
        red4ext_rs::trace!("[on_audio_event] hook handle retrieved");
        if let Some(detour) = guard.as_ref() {
            let AudioEvent {
                event_name,
                emitter_name,
                name_data,
                float_data,
                event_type,
                event_flags,
            } = AudioEvent::from_memory(a);
            red4ext_rs::info!(
                "[on_audio_event][AudioEvent] name {}, emitter {}, data {}, float {float_data}, type {event_type}, flags {event_flags}",
                red4ext_rs::ffi::resolve_cname(&event_name),
                red4ext_rs::ffi::resolve_cname(&emitter_name),
                red4ext_rs::ffi::resolve_cname(&name_data)
            );

            let original: ExternFnRedEventHandler =
                unsafe { std::mem::transmute(detour.trampoline()) };
            unsafe { original(o, a) };
            red4ext_rs::trace!("[on_audio_event] original method called");
        }
    }
}

make_hook!(
    hook_ent_audio_event,
    ON_ENT_AUDIO_EVENT,
    on_audio_event,
    HOOK_ON_ENT_AUDIO_EVENT
);

unsafe fn get_module(module: &str) -> Option<HMODULE> {
    let module = U16CString::from_str_truncate(module);
    let res = GetModuleHandleW(module.as_ptr());
    res.is_null().not().then_some(res)
}
