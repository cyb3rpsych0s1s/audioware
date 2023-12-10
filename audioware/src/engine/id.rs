use red4ext_rs::types::CName;

#[repr(transparent)]
pub struct SoundId(CName);

impl std::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", red4ext_rs::ffi::resolve_cname(&self.0))
    }
}
