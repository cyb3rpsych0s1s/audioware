use red4ext_rs::SdkEnv;

mod entity;
mod events;
mod save_handling_controller;
mod time_dilatable;
mod time_system;

pub fn attach(env: &SdkEnv) {
    save_handling_controller::attach_hook(env);
    entity::attach_hook(env);
    time_dilatable::attach_hooks(env);
    time_system::attach_hook(env);

    // #[cfg(debug_assertions)]
    // {
    //     events::dialog_line::Handler::attach(env);
    //     events::dialog_line_end::Handler::attach(env);
    // }
}

#[rustfmt::skip]
#[doc(hidden)]
mod offsets {
    pub const ENTITY_DISPOSE: u32                               = 0x3221A80;    // 0x14232C744 (2.13)
    pub const TIMEDILATABLE_SETINDIVIDUALTIMEDILATION: u32      = 0x80102488;   // 0x1423AF554 (2.13)
    pub const TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION: u32    = 0xDA20256B;   // 0x14147B424 (2.13)
    pub const TIMESYSTEM_SETTIMEDILATION: u32                   = 0xA1DC1F92;   // 0x140A46EE4 (2.13)
    // gameuiSaveHandlingController
    // note: LoadSaveInGame and LoadModdedSave share same underlying address
    pub const SAVEHANDLINGCONTROLLER_LOAD_SAVE_IN_GAME: u32     = 0x9AB824D9;   // 0x14083FB6C (2.13)

    pub const EVENT_DIALOGLINE: u32                             = 0x10E71E89;   // 0x1409C12A8 (2.12a)
    pub const EVENT_DIALOGLINEEND: u32                          = 0x6F24331;    // 0x141188BF4 (2.12a)
}

#[macro_export]
macro_rules! attach_hook {
    ($name:literal, $offset:path, $hook: ident, $me:ident, $to:ident $(, $v:vis)?) => {
        ::red4ext_rs::hooks! {
            static $hook: fn(
                i: *mut ::red4ext_rs::types::IScriptable,
                f: *mut ::red4ext_rs::types::StackFrame,
                a3: ::red4ext_rs::VoidPtr,
                a4: ::red4ext_rs::VoidPtr) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        $($v)? fn $me(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve($offset);
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook($hook, addr, $to) };
            $crate::utils::intercept!("attached hook for {}", $name);
        }
    };
    ($name:literal, $offset:path) => {
        attach_hook!($name, $offset, HOOK, attach_hook, detour, pub);
    };
}
