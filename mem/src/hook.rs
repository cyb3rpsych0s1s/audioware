//! deprecated, only kept around for learning purpose

#[macro_export]
macro_rules! hook {
    ($name:ident, $address:ident, $fn_ty:ty, $hook:ident, $storage:ident) => {
        fn $storage() -> &'static ::std::sync::Mutex<::std::option::Option<::retour::RawDetour>> {
            static INSTANCE: ::once_cell::sync::OnceCell<
                ::std::sync::Mutex<::std::option::Option<::retour::RawDetour>>,
            > = ::once_cell::sync::OnceCell::new();
            return INSTANCE.get_or_init(::std::default::Default::default)
        }
        pub struct $name;
        impl $crate::Hook for $name {
            fn load() {
                #[cfg(debug_assertions)]
                ::red4ext_rs::info!("[{}] load", ::std::stringify! {$name});
                match unsafe { $crate::load_native_func($address, $hook) } {
                    Ok(detour) => match unsafe { detour.enable() } {
                        Ok(_) => {
                            if let Ok(mut guard) = $storage().try_lock() {
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
            fn unload() {
                #[cfg(debug_assertions)]
                ::red4ext_rs::info!("[{}] unload", ::std::stringify! {$name});
                let _ = $storage().try_lock().map(Option::take);
            }
        }
    };
}

#[macro_export]
macro_rules! native_func {
    ($name:ident, $address:ident, $storage:ident, $hook:ident, ($($param:ident : $ty:ty),*) -> $ret:ty, $should:ident, $detour:ident) => {
        $crate::hook!($name, $address, $crate::ExternFnRedRegisteredFunc, $hook, $storage);
        fn $hook(
            ctx: *mut red4ext_rs::ffi::IScriptable,
            frame: *mut red4ext_rs::ffi::CStackFrame,
            out: *mut std::ffi::c_void,
            a4: i64,
        ) {
            let rewind = unsafe { (*frame.cast::<$crate::frame::StackFrame>()).code };
            // read stack frame
            $(
                let mut $param: $ty = <$ty>::default();
                unsafe { ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut $param)) };
            )*
            if !$should($($param.clone(),)*) {
                if let Ok(ref guard) = $storage().try_lock() {
                    if let Some(detour) = guard.as_ref() {
                        // rewind the stack and call vanilla
                        unsafe {
                            (*frame.cast::<$crate::frame::StackFrame>()).code = rewind;
                            (*frame.cast::<$crate::frame::StackFrame>()).currentParam = 0;
                        }
                        let original: $crate::ExternFnRedRegisteredFunc =
                            unsafe { ::std::mem::transmute(detour.trampoline()) };
                        unsafe { original(ctx, frame, out, a4) };
                    }
                }
            } else {
                $detour($($param,)*);
            }
        }
    };
}
