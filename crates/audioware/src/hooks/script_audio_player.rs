#![allow(dead_code)]

use red4ext_rs::SdkEnv;

pub fn attach_hooks(env: &SdkEnv) {
    play::single::attach_hook(env);
    play::three::attach_hook(env);
    play::unique_with_seek::attach_hook(env);
    stop::attach_hook(env);
    set::switch::attach_hook(env);
    set::parameter::attach_hook(env);
}

mod play {
    pub mod single {
        use red4ext_rs::{VoidPtr, types::CName};

        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: VoidPtr,
                a2: CName) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SCRIPTAUDIOPLAYER_PLAY_SINGLE,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for ScriptAudioPlayer::Play( CName )"
            );
        }

        unsafe extern "C" fn detour(
            a1: VoidPtr,
            a2: CName,
            cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName),
        ) {
            unsafe {
                crate::utils::intercept!("ScriptAudioPlayer::Play( {} )", a2.as_str());
                cb(a1, a2);
            }
        }
    }
    pub mod three {
        use red4ext_rs::{VoidPtr, types::CName};

        use crate::RedTagList;

        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: VoidPtr,
                a2: CName,
                a3: *const RedTagList,
                a4: bool) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SCRIPTAUDIOPLAYER_PLAY_THREE,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for ScriptAudioPlayer::Play( CName, RedTagList, Bool )"
            );
        }

        unsafe extern "C" fn detour(
            a1: VoidPtr,
            a2: CName,
            a3: *const RedTagList,
            a4: bool,
            cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a3: *const RedTagList, a4: bool),
        ) {
            unsafe {
                crate::utils::intercept!(
                    "ScriptAudioPlayer::Play( {}, {}, {a4} )",
                    a2.as_str(),
                    &*a3
                );
                cb(a1, a2, a3, a4);
            }
        }
    }
    pub mod unique_with_seek {
        use red4ext_rs::{VoidPtr, types::CName};

        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: VoidPtr,
                a2: CName,
                a3: f32) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SCRIPTAUDIOPLAYER_PLAY_UNIQUE_WITH_SEEK,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for ScriptAudioPlayer::PlayUniqueWithSeek( CName, Float )"
            );
        }

        unsafe extern "C" fn detour(
            a1: VoidPtr,
            a2: CName,
            a3: f32,
            cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a4: f32),
        ) {
            unsafe {
                crate::utils::intercept!(
                    "ScriptAudioPlayer::PlayUniqueWithSeek( {}, {a3} )",
                    a2.as_str(),
                );
                cb(a1, a2, a3);
            }
        }
    }
}

mod stop {
    use red4ext_rs::{VoidPtr, types::CName};

    ::red4ext_rs::hooks! {
        static HOOK: fn(
            a1: VoidPtr,
            a2: CName) -> ();
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr =
            ::red4ext_rs::addr_hashes::resolve(super::super::offsets::SCRIPTAUDIOPLAYER_STOP);
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for ScriptAudioPlayer::Stop( CName )"
        );
    }

    unsafe extern "C" fn detour(
        a1: VoidPtr,
        a2: CName,
        cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName),
    ) {
        unsafe {
            crate::utils::intercept!("ScriptAudioPlayer::Stop( {} )", a2.as_str());
            cb(a1, a2);
        }
    }
}
mod set {
    pub mod switch {
        use red4ext_rs::{VoidPtr, types::CName};

        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: VoidPtr,
                a2: CName,
                a3: CName) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SCRIPTAUDIOPLAYER_SET_SWITCH,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for ScriptAudioPlayer::SetSwitch( CName, CName )"
            );
        }

        unsafe extern "C" fn detour(
            a1: VoidPtr,
            a2: CName,
            a3: CName,
            cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a4: CName),
        ) {
            unsafe {
                crate::utils::intercept!(
                    "ScriptAudioPlayer::SetSwitch( {}, {} )",
                    a2.as_str(),
                    a3.as_str()
                );
                cb(a1, a2, a3);
            }
        }
    }
    pub mod parameter {
        use red4ext_rs::{VoidPtr, types::CName};

        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: VoidPtr,
                a2: CName,
                a3: f32) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SCRIPTAUDIOPLAYER_SET_PARAMETER,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for ScriptAudioPlayer::SetParameter( CName, Float )"
            );
        }

        unsafe extern "C" fn detour(
            a1: VoidPtr,
            a2: CName,
            a3: f32,
            cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a4: f32),
        ) {
            unsafe {
                crate::utils::intercept!(
                    "ScriptAudioPlayer::SetParameter( {}, {a3} )",
                    a2.as_str(),
                );
                cb(a1, a2, a3);
            }
        }
    }
}
