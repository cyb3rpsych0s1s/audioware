use audioware_bank::error::registry::ErrorDisplay;
use red4ext_rs::VoidPtr;
use red4ext_rs::types::{Cruid, RaRef};

::red4ext_rs::hooks! {
    static HOOK: fn(
    a1: VoidPtr,
    a2: VoType,
    a3: *const Cruid,
    a4: VoidPtr,
    a5: *mut RaRef<()>) -> VoFileResult;
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr = ::red4ext_rs::addr_hashes::resolve(super::offsets::VO_STORAGE_GET_VO_FILE);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for vo::GetVoFile( VoType, &CRUID, x, &RaRef ) -> VoFileResult"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: VoType,
    a3: *const Cruid,
    a4: VoidPtr,
    a5: *mut RaRef<()>,
    cb: unsafe extern "C" fn(
        a1: VoidPtr,
        a2: VoType,
        a3: *const Cruid,
        a4: VoidPtr,
        a5: *mut RaRef<()>,
    ) -> VoFileResult,
) -> VoFileResult {
    unsafe {
        let outcome = cb(a1, a2, a3, a4, a5);
        crate::utils::lifecycle!(
            "vo::GetVoFile( {a2}, {}, x, x) -> {outcome}",
            (&*a3).error_display()
        );
        outcome
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[allow(clippy::enum_variant_names, dead_code)]
pub enum VoFileResult {
    NoFileFound = 0,
    RecordedFileFound = 1,
    GeneratedFileFound = 2,
    GeneratedFallbackVariantFileFound = 3,
    CommonStorageFileFound = 4,
}

impl std::fmt::Display for VoFileResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoFileFound => "NoFileFound",
                Self::RecordedFileFound => "RecordedFileFound",
                Self::GeneratedFileFound => "GeneratedFileFound",
                Self::GeneratedFallbackVariantFileFound => "GeneratedFallbackVariantFileFound",
                Self::CommonStorageFileFound => "CommonStorageFileFound",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(dead_code)]
pub enum VoType {
    Standard = 0,
    Rewind = 1,
    Helmet = 2,
    Holocall = 3,
}

impl std::fmt::Display for VoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Standard => "Standard",
                Self::Rewind => "Rewind",
                Self::Helmet => "Helmet",
                Self::Holocall => "Holocall",
            }
        )
    }
}
