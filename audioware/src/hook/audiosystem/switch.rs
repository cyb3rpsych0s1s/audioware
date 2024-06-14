use crate::{
    natives::{delegate_play, delegate_stop},
    safe_call, Maybe,
};

use super::super::address::ON_AUDIOSYSTEM_SWITCH;
use audioware_bank::Banks;
use audioware_engine::Engine;
use audioware_mem::{load_native_func, Hook};
use red4ext_rs::types::{CName, EntityId};

pub struct HookAudioSystemSwitch;
mod __internals_func_hookaudiosystemswitch {
    fn storage() -> &'static ::std::sync::RwLock<::std::option::Option<::retour::RawDetour>> {
        static INSTANCE: ::once_cell::sync::OnceCell<
            ::std::sync::RwLock<::std::option::Option<::retour::RawDetour>>,
        > = ::once_cell::sync::OnceCell::new();
        return INSTANCE.get_or_init(::std::default::Default::default);
    }
    pub(super) fn store(detour: ::std::option::Option<::retour::RawDetour>) {
        if detour.is_some() {
            if let Ok(mut guard) = self::storage().try_write() {
                *guard = detour;
            } else {
                ::red4ext_rs::error!("lock contention (store HookAudioSystemSwitch)");
            }
        } else if let Ok(mut guard) = self::storage().try_write() {
            let _ = guard.take();
        } else {
            ::red4ext_rs::error!("lock contention (store HookAudioSystemSwitch)");
        }
    }
    pub(super) fn trampoline(closure: ::std::boxed::Box<dyn ::std::ops::Fn(&::retour::RawDetour)>) {
        if let Ok(Some(guard)) = self::storage().try_read().as_deref() {
            closure(guard);
        } else {
            ::red4ext_rs::error!("lock contention (trampoline HookAudioSystemSwitch)");
        }
    }
}
unsafe impl ::audioware_mem::DetourFunc for HookAudioSystemSwitch {
    const OFFSET: usize = ON_AUDIOSYSTEM_SWITCH;
    type Inputs = (CName, CName, EntityId, CName);
    unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs {
        let mut arg_0: CName = <CName>::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(
                frame,
                ::std::mem::transmute::<&mut red4ext_rs::prelude::CName, red4ext_rs::types::VoidPtr>(
                    &mut arg_0,
                ),
            )
        };
        let mut arg_1: CName = <CName>::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(
                frame,
                ::std::mem::transmute::<&mut red4ext_rs::prelude::CName, red4ext_rs::types::VoidPtr>(
                    &mut arg_1,
                ),
            )
        };
        let mut arg_2: EntityId = <EntityId>::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(
                frame,
                ::std::mem::transmute::<
                    &mut red4ext_rs::prelude::EntityId,
                    red4ext_rs::types::VoidPtr,
                >(&mut arg_2),
            )
        };
        let mut arg_3: CName = <CName>::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(
                frame,
                ::std::mem::transmute::<&mut red4ext_rs::prelude::CName, red4ext_rs::types::VoidPtr>(
                    &mut arg_3,
                ),
            )
        };
        (arg_0, arg_1, arg_2, arg_3)
    }
}
impl HookAudioSystemSwitch {
    unsafe fn hook(
        ctx: *mut red4ext_rs::ffi::IScriptable,
        frame: *mut red4ext_rs::ffi::CStackFrame,
        out: *mut std::ffi::c_void,
        a4: i64,
    ) {
        use audioware_mem::DetourFunc;
        let rewind = unsafe { (*frame.cast::<audioware_mem::frame::StackFrame>()).code };
        // read stack frame
        let inputs: (CName, CName, EntityId, CName) = unsafe { Self::from_frame(frame) };
        let (previous_sound_name, next_sound_name, entity_id, emitter_name) = inputs;
        match (
            Banks::exists(&previous_sound_name),
            Banks::exists(&next_sound_name),
        ) {
            (false, false) => {
                let trampoline = move |detour: &retour::RawDetour| {
                    // rewind the stack and call vanilla
                    unsafe {
                        (*frame.cast::<audioware_mem::frame::StackFrame>()).code = rewind;
                        (*frame.cast::<audioware_mem::frame::StackFrame>()).currentParam = 0;
                    }
                    let original: audioware_mem::ExternFnRedRegisteredFunc =
                        unsafe { ::std::mem::transmute(detour.trampoline()) };
                    unsafe { original(ctx, frame, out, a4) };
                };
                __internals_func_hookaudiosystemswitch::trampoline(Box::new(trampoline));
            }
            (p, n) => {
                if p {
                    Engine::stop_by_cname(&previous_sound_name, None);
                } else {
                    delegate_stop(previous_sound_name, entity_id.clone(), emitter_name.clone());
                }

                if n {
                    safe_call!(Engine::play(
                        &next_sound_name,
                        entity_id.maybe(),
                        emitter_name.maybe()
                    ));
                } else {
                    delegate_play(next_sound_name, entity_id, emitter_name);
                }
            }
        }
    }
}
impl Hook for HookAudioSystemSwitch {
    fn load()
    where
        Self: Sized,
    {
        match unsafe { load_native_func(ON_AUDIOSYSTEM_SWITCH, Self::hook) } {
            Ok(detour) => match unsafe { detour.enable() } {
                Ok(_) => {
                    __internals_func_hookaudiosystemswitch::store(Some(detour));
                }
                Err(e) => {
                    ::red4ext_rs::error!("could not enable native function detour ({e})");
                }
            },
            Err(e) => {
                ::red4ext_rs::error!("could not initialize native function detour ({e})");
            }
        }
    }

    fn unload()
    where
        Self: Sized,
    {
        __internals_func_hookaudiosystemswitch::store(None)
    }
}
