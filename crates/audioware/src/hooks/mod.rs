use red4ext_rs::SdkEnv;

mod audio_system;
mod time_dilatable;
mod time_system;

#[cfg(debug_assertions)]
mod entity;
#[cfg(debug_assertions)]
mod save_handling_controller;

#[cfg(feature = "research")]
mod events;

pub fn attach(env: &SdkEnv) {
    audio_system::attach_hooks(env);
    time_dilatable::attach_hooks(env);
    time_system::attach_hooks(env);

    save_handling_controller::attach_hook(env);
    #[cfg(debug_assertions)]
    entity::attach_hook(env);

    #[cfg(feature = "research")]
    {
        // events::audio::attach_hook(env); // 🌊
        events::vehicle_audio::attach_hook(env);
        events::dialog_line_end::attach_hook(env);
        events::dialog_line::attach_hook(env);
        events::weapon::attach_hook(env);
        events::trigger::attach_hooks(env);
    }
}

#[rustfmt::skip]
#[doc(hidden)]
mod offsets {
    pub const AUDIOSYSTEM_PLAY: u32                             = 0xCDB11D0E;   // 0x140974F58 (2.12a)
    pub const AUDIOSYSTEM_STOP: u32                             = 0xD2781D1E;   // 0x1424503F8 (2.12a)
    pub const AUDIOSYSTEM_SWITCH: u32                           = 0x15081DEA;   // 0x140291688 (2.12a)
    #[cfg(debug_assertions)]
    pub const ENTITY_DISPOSE: u32                               = 0x3221A80;    // 0x14232C744 (2.13)
    pub const TIMEDILATABLE_SETINDIVIDUALTIMEDILATION: u32      = 0x80102488;   // 0x1423AF554 (2.13)
    pub const TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION: u32    = 0xDA20256B;   // 0x14147B424 (2.13)
    pub const TIMESYSTEM_SETTIMEDILATION: u32                   = 0xA1DC1F92;   // 0x140A46EE4 (2.13)
    pub const TIMESYSTEM_UNSETTIMEDILATION: u32                 = 0xF0652075;   // 0x1409BAD34 (2.13)
    // gameuiSaveHandlingController
    // note: LoadSaveInGame and LoadModdedSave share same underlying address
    #[cfg(debug_assertions)]
    pub const SAVEHANDLINGCONTROLLER_LOAD_SAVE_IN_GAME: u32     = 0x9AB824D9;   // 0x14083FB6C (2.13)

    #[cfg(feature = "research")]
    mod events {
        pub const EVENT_DIALOGLINE: u32                             = 0x10E71E89;   // 0x1409C12A8 (2.12a)
        pub const EVENT_DIALOGLINEEND: u32                          = 0x6F24331;    // 0x141188BF4 (2.12a)
        pub const VEHICLE_AUDIO_EVENT: u32                          = 0x69EF1461;   // 0x1418D4C44 (2.13)
        pub const AUDIO_EVENT: u32                                  = 0x10C412FD;   // 0x14065816C (2.13)
        pub const WEAPON_PRE_FIRE_EVENT: u32                        = 0x7BC51906;   // 0x140652AB4 (2.13)
        // note: gameaudioeventsStopWeaponFire and gameweaponeventsStopFiringEvent share same underlying address
        pub const WEAPON_STOP_FIRING_EVENT: u32                     = 0xA83C1996;   // 0x140652AF8 (2.13)
        pub const AREA_ENTERED_EVENT: u32                           = 0x252517CB;   // 0x142863744 (2.21)
        pub const AREA_EXITED_EVENT: u32                            = 0xF3E11703;   // 0x142863818 (2.21)
    }
    #[cfg(feature = "research")]
    pub use events::*;
}

#[macro_export]
macro_rules! attach_native_func {
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
            $crate::utils::intercept!("attached native func hook for {}", $name);
        }
    };
    ($name:literal, $offset:path) => {
        attach_native_func!($name, $offset, HOOK, attach_hook, detour, pub);
    };
}

#[macro_export]
macro_rules! attach_native_event {
    ($offset:path, $class:path, $to:ident $(, $v:vis)?) => {
        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: *mut ::red4ext_rs::types::IScriptable,
                a2: *mut $class) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        $($v)? fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve($offset);
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, $to) };
            $crate::utils::intercept!("attached native event hook for {}", <$class as ::red4ext_rs::ScriptClass>::NAME);
        }
    };
    ($offset:path, $class:path) => {
        attach_native_event!($offset, $class, detour, pub);
    };
}
