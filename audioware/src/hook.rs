use std::borrow::BorrowMut;
use std::ops::Not;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use red4ext_rs::types::{CName, EntityId};
use retour::RawDetour;
use widestring::U16CString;
use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::addresses::{ON_AUDIOSYSTEM_PLAY, ON_MUSIC_EVENT, ON_VOICE_EVENT, ON_AUDIOSYSTEM_STOP};
use crate::interop::{MusicEvent, VoiceEvent};
use crate::{addresses::ON_ENT_AUDIO_EVENT, interop::AudioEvent};
use audioware_types::FromMemory;

macro_rules! make_hook {
    ($name:ident, $address:expr, $fn_ty:ty, $hook:expr, $storage:expr) => {
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
            Ok(())
        }
    };
}

pub(crate) type ExternFnRedEventHandler = unsafe extern "C" fn(usize, usize) -> ();
pub(crate) type ExternFnRedRegisteredFunc = unsafe extern "C" fn(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) -> ();

lazy_static! {
    pub(crate) static ref HOOK_ON_ENT_AUDIO_EVENT: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
    pub(crate) static ref HOOK_ON_MUSIC_EVENT: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
    pub(crate) static ref HOOK_ON_VOICE_EVENT: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
    pub(crate) static ref HOOK_ON_SCN_AUDIO_EVENT: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
    pub(crate) static ref HOOK_ON_AUDIOSYSTEM_PLAY: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
    pub(crate) static ref HOOK_ON_AUDIOSYSTEM_STOP: Arc<Mutex<Option<RawDetour>>> =
        Arc::new(Mutex::new(None));
}

#[allow(unused_variables)]
pub fn on_ent_audio_event(o: usize, a: usize) {
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
                unk64,
            } = AudioEvent::from_memory(a);
            if red4ext_rs::ffi::resolve_cname(&emitter_name) != "None"
                && (event_type == crate::interop::AudioEventActionType::Play
                    || event_type == crate::interop::AudioEventActionType::PlayExternal
                    || event_type == crate::interop::AudioEventActionType::SetSwitch
                    || event_type == crate::interop::AudioEventActionType::SetParameter
                    || event_type == crate::interop::AudioEventActionType::StopSound)
            {
                red4ext_rs::info!(
                    "[on_audio_event][AudioEvent] name {}, emitter {}, data {}, float {float_data}, type {event_type}, flags {event_flags}",
                    red4ext_rs::ffi::resolve_cname(&event_name),
                    red4ext_rs::ffi::resolve_cname(&emitter_name),
                    red4ext_rs::ffi::resolve_cname(&name_data)
                );
            } else if emitter_name == CName::new("ono_v_effort_short") {
                red4ext_rs::info!(
                    "[on_audio_event][AudioEvent] name {}, emitter {}, data {}, float {float_data}, type {event_type}, flags {event_flags}, unk64 {unk64}",
                    red4ext_rs::ffi::resolve_cname(&event_name),
                    red4ext_rs::ffi::resolve_cname(&emitter_name),
                    red4ext_rs::ffi::resolve_cname(&name_data)
                );
            }

            let original: ExternFnRedEventHandler =
                unsafe { std::mem::transmute(detour.trampoline()) };
            unsafe { original(o, a) };
            red4ext_rs::trace!("[on_audio_event] original method called");
        }
    }
}

pub fn on_music_event(o: usize, a: usize) {
    red4ext_rs::trace!("[on_music_event] hooked");
    if let Ok(ref guard) = HOOK_ON_MUSIC_EVENT.clone().try_lock() {
        red4ext_rs::trace!("[on_music_event] hook handle retrieved");
        if let Some(detour) = guard.as_ref() {
            let MusicEvent { event_name } = MusicEvent::from_memory(a);
            red4ext_rs::info!(
                "[on_music_event][MusicEvent] name {}",
                red4ext_rs::ffi::resolve_cname(&event_name)
            );

            let original: ExternFnRedEventHandler =
                unsafe { std::mem::transmute(detour.trampoline()) };
            unsafe { original(o, a) };
            red4ext_rs::trace!("[on_music_event] original method called");
        }
    }
}

pub fn on_voice_event(o: usize, a: usize) {
    red4ext_rs::trace!("[on_voice_event] hooked");
    if let Ok(ref guard) = HOOK_ON_VOICE_EVENT.clone().try_lock() {
        red4ext_rs::trace!("[on_voice_event] hook handle retrieved");
        if let Some(detour) = guard.as_ref() {
            let VoiceEvent {
                event_name,
                grunt_type,
                grunt_interrupt_mode,
                is_v,
            } = VoiceEvent::from_memory(a);
            if is_v {
                red4ext_rs::info!(
                    "[on_voice_event][VoiceEvent for V] name {}, grunt_type {grunt_type}, grunt_interrupt_mode {grunt_interrupt_mode}",
                    red4ext_rs::ffi::resolve_cname(&event_name)
                );
            }

            let original: ExternFnRedEventHandler =
                unsafe { std::mem::transmute(detour.trampoline()) };
            unsafe { original(o, a) };
            red4ext_rs::trace!("[on_voice_event] original method called");
        }
    }
}

pub fn on_audiosystem_play(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) {
    let rewind = unsafe {
        std::mem::transmute::<*mut red4ext_rs::ffi::CStackFrame, &mut crate::frame::StackFrame>(
            frame,
        )
        .code
    };
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
                    std::mem::transmute::<
                        *mut red4ext_rs::ffi::CStackFrame,
                        &mut crate::frame::StackFrame,
                    >(frame)
                    .code = rewind;
                    std::mem::transmute::<
                        *mut red4ext_rs::ffi::CStackFrame,
                        &mut crate::frame::StackFrame,
                    >(frame)
                    .currentParam = 0;
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
    let rewind = unsafe {
        std::mem::transmute::<*mut red4ext_rs::ffi::CStackFrame, &mut crate::frame::StackFrame>(
            frame,
        )
        .code
    };
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
                    std::mem::transmute::<
                        *mut red4ext_rs::ffi::CStackFrame,
                        &mut crate::frame::StackFrame,
                    >(frame)
                    .code = rewind;
                    std::mem::transmute::<
                        *mut red4ext_rs::ffi::CStackFrame,
                        &mut crate::frame::StackFrame,
                    >(frame)
                    .currentParam = 0;
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

pub fn is_vanilla(event_name: CName) -> bool {
    match event_name {
        // TODO: match from loaded infos
        v if v == CName::new("ono_v_effort_short") => false,
        _ => true,
    }
}

pub fn custom_engine_play(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    red4ext_rs::info!(
        "would have call custom engine Play method with: event_name {}, entity_id {:#?}, emitter_name {}",
        red4ext_rs::ffi::resolve_cname(&event_name),
        entity_id,
        red4ext_rs::ffi::resolve_cname(&emitter_name)
    );
}

pub fn custom_engine_stop(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    red4ext_rs::info!(
        "would have call custom engine Stop method with: event_name {}, entity_id {:#?}, emitter_name {}",
        red4ext_rs::ffi::resolve_cname(&event_name),
        entity_id,
        red4ext_rs::ffi::resolve_cname(&emitter_name)
    );
}

make_hook!(
    hook_ent_audio_event,
    ON_ENT_AUDIO_EVENT,
    ExternFnRedEventHandler,
    on_ent_audio_event,
    HOOK_ON_ENT_AUDIO_EVENT
);

make_hook!(
    hook_on_music_event,
    ON_MUSIC_EVENT,
    ExternFnRedEventHandler,
    on_music_event,
    HOOK_ON_MUSIC_EVENT
);

make_hook!(
    hook_on_voice_event,
    ON_VOICE_EVENT,
    ExternFnRedEventHandler,
    on_voice_event,
    HOOK_ON_VOICE_EVENT
);

make_hook!(
    hook_on_audiosystem_play,
    ON_AUDIOSYSTEM_PLAY,
    ExternFnRedRegisteredFunc,
    on_audiosystem_play,
    HOOK_ON_AUDIOSYSTEM_PLAY
);

make_hook!(
    hook_on_audiosystem_stop,
    ON_AUDIOSYSTEM_STOP,
    ExternFnRedRegisteredFunc,
    on_audiosystem_stop,
    HOOK_ON_AUDIOSYSTEM_STOP
);

unsafe fn get_module(module: &str) -> Option<HMODULE> {
    let module = U16CString::from_str_truncate(module);
    let res = GetModuleHandleW(module.as_ptr());
    res.is_null().not().then_some(res)
}
