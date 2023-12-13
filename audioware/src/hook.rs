use std::ops::Not;
use std::{borrow::BorrowMut, ops::Deref};

use audioware_types::FromMemory;
use red4ext_rs::conv::ClassType;
use red4ext_rs::types::{CName, EntityId, MaybeUninitRef};
use widestring::U16CString;
use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::interop::audio::{AudioEvent, AudioEventActionType};
use crate::interop::event::Event;
use crate::interop::sound_play_event::SoundPlayEvent;

pub(crate) trait Hook {
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

pub(crate) type ExternFnRedEventHandler = unsafe extern "C" fn(usize, usize) -> ();
pub(crate) type ExternFnRedRegisteredFunc = unsafe extern "C" fn(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) -> ();

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
                ..
            } = AudioEvent::from_memory(a);
            if red4ext_rs::ffi::resolve_cname(&emitter_name) != "None"
                && (event_type == AudioEventActionType::Play
                    || event_type == AudioEventActionType::PlayExternal
                    || event_type == AudioEventActionType::SetSwitch
                    || event_type == AudioEventActionType::SetParameter
                    || event_type == AudioEventActionType::StopSound)
            {
                red4ext_rs::info!(
                    "[on_audio_event][AudioEvent] name {}, emitter {}, data {}, float {float_data}, type {event_type}, flags {event_flags}",
                    red4ext_rs::ffi::resolve_cname(&event_name),
                    red4ext_rs::ffi::resolve_cname(&emitter_name),
                    red4ext_rs::ffi::resolve_cname(&name_data)
                );
            } else if emitter_name == CName::new("ono_v_effort_short") {
                red4ext_rs::info!(
                    "[on_audio_event][AudioEvent] name {}, emitter {}, data {}, float {float_data}, type {event_type}, flags {event_flags}",
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

pub fn on_entity_queue_event(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) {
    let rewind = unsafe { (*frame.cast::<crate::frame::StackFrame>()).code };
    // read stack frame
    let mut event: MaybeUninitRef<Event> = MaybeUninitRef::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut event)) };
    let event = event.into_ref().unwrap();
    red4ext_rs::info!(
        "event class name: {}",
        red4ext_rs::ffi::resolve_cname(&event.get_class_name())
    );
    if event.is_exactly_a(CName::new("entAudioEvent")) {
        let ent_audio_event: red4ext_rs::types::Ref<AudioEvent> =
            unsafe { std::mem::transmute(event.clone()) };
        red4ext_rs::info!(
            "                  -> entAudioEvent: event_name '{}', emitter_name '{}', name_data: '{}', event_type: '{}', event_flags '{}'",
            red4ext_rs::ffi::resolve_cname(&ent_audio_event.deref().event_name),
            red4ext_rs::ffi::resolve_cname(&ent_audio_event.deref().emitter_name),
            red4ext_rs::ffi::resolve_cname(&ent_audio_event.deref().name_data),
            ent_audio_event.deref().event_type,
            ent_audio_event.deref().event_flags
        );
    } else if event.is_exactly_a(CName::new(SoundPlayEvent::NATIVE_NAME)) {
        let sound_play_event: red4ext_rs::types::Ref<SoundPlayEvent> =
            unsafe { std::mem::transmute(event) };
        red4ext_rs::info!(
            "                  -> gameaudioeventsPlaySound: sound_name '{}', emitter_name '{}'",
            red4ext_rs::ffi::resolve_cname(&sound_play_event.deref().sound_name),
            red4ext_rs::ffi::resolve_cname(&sound_play_event.deref().emitter_name)
        );
    }
    if let Ok(ref guard) = HOOK_ON_ENTITY_QUEUE_EVENT.clone().try_lock() {
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

pub fn on_icomponent_queue_entity_event(
    ctx: *mut red4ext_rs::ffi::IScriptable,
    frame: *mut red4ext_rs::ffi::CStackFrame,
    out: *mut std::ffi::c_void,
    a4: i64,
) {
    let rewind = unsafe { (*frame.cast::<crate::frame::StackFrame>()).code };
    // read stack frame
    let mut event: MaybeUninitRef<Event> = MaybeUninitRef::default();
    unsafe { red4ext_rs::ffi::get_parameter(frame, std::mem::transmute(&mut event)) };
    let event = event.into_ref().unwrap();
    let event_name = event.sound_name();
    if !is_vanilla(event_name) {
    } else if let Ok(ref guard) = HOOK_ON_ICOMPONENT_QUEUE_ENTITY_EVENT.clone().try_lock() {
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
    !crate::engine::banks::exists(event_name).unwrap_or(false)
}

pub fn custom_engine_play(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    red4ext_rs::info!(
        "call custom engine Play method with: event_name {}, entity_id {:#?}, emitter_name {}",
        red4ext_rs::ffi::resolve_cname(&event_name),
        entity_id,
        red4ext_rs::ffi::resolve_cname(&emitter_name)
    );
    crate::engine::play(event_name);
}

pub fn custom_engine_stop(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    red4ext_rs::info!(
        "call custom engine Stop method with: event_name {}, entity_id {:#?}, emitter_name {}",
        red4ext_rs::ffi::resolve_cname(&event_name),
        entity_id,
        red4ext_rs::ffi::resolve_cname(&emitter_name)
    );
    crate::engine::stop(event_name);
}

make_hook!(
    HookEntAudioEvent,
    ON_ENT_AUDIO_EVENT,
    ExternFnRedEventHandler,
    on_ent_audio_event,
    HOOK_ON_ENT_AUDIO_EVENT
);

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
    HookEntityQueueEvent,
    ON_ENTITY_QUEUE_EVENT,
    ExternFnRedRegisteredFunc,
    on_entity_queue_event,
    HOOK_ON_ENTITY_QUEUE_EVENT
);

make_hook!(
    HookIComponentQueueEntityEvent,
    ON_ICOMPONENT_QUEUE_ENTITY_EVENT,
    ExternFnRedRegisteredFunc,
    on_icomponent_queue_entity_event,
    HOOK_ON_ICOMPONENT_QUEUE_ENTITY_EVENT
);

unsafe fn get_module(module: &str) -> Option<HMODULE> {
    let module = U16CString::from_str_truncate(module);
    let res = GetModuleHandleW(module.as_ptr());
    res.is_null().not().then_some(res)
}
