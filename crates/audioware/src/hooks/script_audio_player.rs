use red4ext_rs::SdkEnv;

pub fn attach_hooks(env: &SdkEnv) {
    play::single::attach_hook(env);
    play::three::attach_hook(env);
}

mod play {
    pub mod single {
        use red4ext_rs::{VoidPtr, types::CName};

        ::red4ext_rs::hooks! {
            static PLAY: fn(
                a1: VoidPtr,
                a2: CName) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SCRIPTAUDIOPLAYER_PLAY_SINGLE,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(PLAY, addr, detour) };
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
            static PLAY: fn(
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
            unsafe { env.attach_hook(PLAY, addr, detour) };
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
}
