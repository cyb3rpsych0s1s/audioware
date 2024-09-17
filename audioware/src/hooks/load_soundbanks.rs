#![allow(unused_imports)]

use audioware_bank::BNKS;
use audioware_manifest::SoundBankInfo;
use red4ext_rs::{
    addr_hashes, hooks,
    types::{CName, CNamePool, RedHashMap, Ref},
    SdkEnv,
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
    crate::utils::lifecycle!("LoadSoundBanks: before transmuting");
    let map = a1 + 104;
    let map = map as *mut red4ext_rs::types::RedHashMap<
        red4ext_rs::types::CName,
        red4ext_rs::types::Ref<audioware_bank::SoundBankInfo>,
    >;
    let map: &mut RedHashMap<CName, Ref<audioware_bank::SoundBankInfo>> = unsafe { &mut *map };
    crate::utils::lifecycle!("LoadSoundBanks: after transmuting");
    for (key, value) in BNKS.iter() {
        let reference: Ref<audioware_bank::SoundBankInfo> =
            Ref::new_with(|x: &mut audioware_bank::SoundBankInfo| {
                x.name = value.name;
                x.is_resident = value.is_resident;
                x.path = value.path.clone();
            })
            .unwrap();
        crate::utils::lifecycle!("LoadSoundBanks: before insertion");
        let _ = map.insert(*key, reference);
        crate::utils::lifecycle!("LoadSoundBanks: after insertion");
    }
    res
}
