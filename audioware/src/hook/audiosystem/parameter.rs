use super::super::address::ON_AUDIOSYSTEM_PARAMETER;
use audioware_mem::ExternFnRedRegisteredFunc;
use red4ext_rs::types::{CName, EntityId};

pub fn audioware_parameter(params: (CName, f32, EntityId, CName)) {
    // red4ext_rs::info!(
    //     "AudioSystem::Parameter({}, {}, {:?}, {})",
    //     params.0,
    //     params.1,
    //     params.2,
    //     params.3
    // );
}

pub fn audioware_always_allow(_: &(CName, f32, EntityId, CName)) -> bool {
    true
}

pub struct HookAudioSystemParameter;

mod __internals_func_audiosystem_parameter {
    fn storage() -> &'static ::std::sync::Mutex<::std::option::Option<::retour::RawDetour>> {
        static INSTANCE: ::once_cell::sync::OnceCell<
            ::std::sync::Mutex<::std::option::Option<::retour::RawDetour>>,
        > = ::once_cell::sync::OnceCell::new();
        return INSTANCE.get_or_init(::std::default::Default::default);
    }
    pub(super) fn store(detour: ::std::option::Option<::retour::RawDetour>) {
        if let Ok(mut guard) = self::storage().try_lock() {
            *guard = detour;
        } else {
            ::red4ext_rs::error!("lock contention (store {})", stringify!(#name));
        }
    }
    pub(super) fn trampoline(closure: ::std::boxed::Box<dyn ::std::ops::Fn(&::retour::RawDetour)>) {
        if let Ok(Some(guard)) = self::storage().try_lock().as_deref() {
            closure(guard);
        } else {
            ::red4ext_rs::error!("lock contention (trampoline {})", stringify!(#name));
        }
    }
}
unsafe impl ::audioware_mem::DetourFunc for HookAudioSystemParameter {
    const OFFSET: usize = ON_AUDIOSYSTEM_PARAMETER;
    type Inputs = (CName, f32, EntityId, CName);
    unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs {
        let mut parameter_name: CName = CName::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut parameter_name))
        };
        let mut parameter_value: f32 = f32::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut parameter_value))
        };
        let mut entity_id: EntityId = EntityId::default();
        unsafe { ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut entity_id)) };
        let mut emitter_name: CName = CName::default();
        unsafe {
            ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut emitter_name))
        };
        (parameter_name, parameter_value, entity_id, emitter_name)
    }
}
impl ::audioware_mem::NativeFunc for HookAudioSystemParameter {
    const HOOK: fn(Self::Inputs) -> () = audioware_parameter;
    const CONDITION: fn(&Self::Inputs) -> bool = audioware_always_allow;
    const TRAMPOLINE: fn(Box<dyn Fn(&::retour::RawDetour)>) =
        __internals_func_audiosystem_parameter::trampoline;
    const STORE: fn(Option<::retour::RawDetour>) = __internals_func_audiosystem_parameter::store;

    unsafe fn hook(
        ctx: *mut red4ext_rs::ffi::IScriptable,
        frame: *mut red4ext_rs::ffi::CStackFrame,
        out: *mut std::ffi::c_void,
        a4: i64,
    ) {
        use audioware_mem::DetourFunc;
        let rewind = unsafe { (*frame.cast::<audioware_mem::frame::StackFrame>()).code };
        // read stack frame
        let inputs: Self::Inputs = unsafe { Self::from_frame(frame) };
        Self::HOOK(inputs);
        let trampoline = move |detour: &retour::RawDetour| {
            // rewind the stack and call vanilla
            unsafe {
                (*frame.cast::<audioware_mem::frame::StackFrame>()).code = rewind;
                (*frame.cast::<audioware_mem::frame::StackFrame>()).currentParam = 0;
            }
            let original: ExternFnRedRegisteredFunc =
                unsafe { ::std::mem::transmute(detour.trampoline()) };
            unsafe { original(ctx, frame, out, a4) };
        };
        Self::TRAMPOLINE(Box::new(trampoline));
    }
}
