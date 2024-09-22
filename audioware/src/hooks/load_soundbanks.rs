#![allow(unused_imports)]

use audioware_bank::BNKS;
use audioware_manifest::SoundBankInfo;
use red4ext_rs::{
    addr_hashes::{self, resolve},
    hooks,
    types::{CName, CNamePool, IScriptable, ISerializable, RedHashMap, Ref, SharedPtr},
    RttiSystemMut, SdkEnv,
};

hooks! {
   static HOOK: fn(a1: i64) -> bool;
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::LOAD_SOUNDBANKS);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for LoadSoundBanks");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(a1: i64, cb: unsafe extern "C" fn(a1: i64) -> bool) -> bool {
    crate::utils::lifecycle!("LoadSoundBanks called");
    let res = cb(a1);

    let map = a1 + 104;
    let map = map as *mut RedHashMap<CName, SharedPtr<audioware_bank::SoundBankInfo>>;
    let map = unsafe { &mut *map };
    for (key, value) in BNKS.iter() {
        let reference = SharedPtr::new_with(value.info.clone());
        let _ = map.insert(*key, reference);
    }

    res
}
