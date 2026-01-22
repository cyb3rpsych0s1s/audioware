use red4ext_rs::SdkEnv;

pub(super) fn attach_hooks(env: &SdkEnv) {
    override_audio_listeners::attach_hook(env);
    activate::attach_hook(env);
    deactivate::attach_hook(env);
}

mod override_audio_listeners {
    use red4ext_rs::VoidPtr;
    ::red4ext_rs::hooks! {
        static HOOK: fn(a1: VoidPtr,
        a2: bool) -> ();
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr = ::red4ext_rs::addr_hashes::resolve(
            super::super::offsets::CAMERACOMPONENT_OVERRIDE_AUDIO_LISTENERS,
        );
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for gameCameraComponent::OverrideAudioListeners( Bool )"
        );
    }

    unsafe extern "C" fn detour(
        a1: VoidPtr,
        a2: bool,
        cb: unsafe extern "C" fn(a1: VoidPtr, a2: bool) -> (),
    ) {
        unsafe {
            crate::utils::inspect!("gameCameraComponent::OverrideAudioListeners( {a2} )");
            cb(a1, a2);
        }
    }
}

mod activate {
    use std::ops::Not;

    use red4ext_rs::VoidPtr;
    use red4ext_rs::types::{IScriptable, Opt, StackFrame};

    use crate::abi::lifecycle::Lifecycle;
    use crate::attach_native_func;
    use crate::engine::queue;
    use crate::utils::warns;

    attach_native_func! {
        "gameCameraComponent::Activate",
        super::super::offsets::CAMERACOMPONENT_ACTIVATE,
        HOOK,
        attach_hook,
        detour,
        pub(super)
    }
    unsafe extern "C" fn detour(
        i: *mut IScriptable,
        f: *mut StackFrame,
        a3: VoidPtr,
        a4: VoidPtr,
        cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
    ) {
        unsafe {
            let frame = &mut *f;
            let state = frame.args_state();

            let blend_time: Opt<f32> = StackFrame::get_arg(frame);
            let should_override_listeners: Opt<bool> = StackFrame::get_arg(frame);

            let blend_time = blend_time.unwrap_or_default();
            let should_override_listeners = should_override_listeners.unwrap_or_default();
            let me = i
                .is_null()
                .not()
                .then(|| &*i)
                .and_then(|x| x.as_serializable().inner_ref());

            frame.restore_args(state);
            cb(i, f, a3, a4);

            if should_override_listeners {
                if let Some(me) = me {
                    queue::notify(Lifecycle::ActivateCamera {
                        blend_time,
                        triggered_by: me.into(),
                    });
                } else {
                    warns!("gameCameraComponent::Activate: unable to get camera");
                }
            }
        }
    }
}

mod deactivate {
    use red4ext_rs::VoidPtr;
    use red4ext_rs::types::{IScriptable, Opt, StackFrame};

    use crate::abi::lifecycle::Lifecycle;
    use crate::attach_native_func;
    use crate::engine::queue;

    attach_native_func! {
        "gameCameraComponent::Deactivate",
        super::super::offsets::CAMERACOMPONENT_DEACTIVATE,
        HOOK,
        attach_hook,
        detour,
        pub(super)
    }
    unsafe extern "C" fn detour(
        i: *mut IScriptable,
        f: *mut StackFrame,
        a3: VoidPtr,
        a4: VoidPtr,
        cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
    ) {
        unsafe {
            let frame = &mut *f;
            let state = frame.args_state();

            let blend_time: Opt<f32> = StackFrame::get_arg(frame);
            let should_override_listeners: Opt<bool> = StackFrame::get_arg(frame);

            let blend_time = blend_time.unwrap_or_default();
            let should_override_listeners = should_override_listeners.unwrap_or_default();

            frame.restore_args(state);
            cb(i, f, a3, a4);

            if should_override_listeners {
                queue::notify(Lifecycle::DeactivateCamera { blend_time });
            }
        }
    }
}
