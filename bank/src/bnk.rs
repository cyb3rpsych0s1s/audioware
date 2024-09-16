use red4ext_rs::{
    class_kind::Native,
    types::{CName, ResRef},
    ScriptClass,
};

use crate::Error;

#[derive(Debug)]
#[repr(C)]
pub struct SoundBankInfo {
    pub name: CName,
    pub is_resident: bool,
    pub path: ResRef,
}

unsafe impl ScriptClass for SoundBankInfo {
    type Kind = Native;
    const NAME: &'static str = "SoundBankInfo";
}

impl TryFrom<audioware_manifest::SoundBankInfo> for self::SoundBankInfo {
    type Error = Error;

    fn try_from(value: audioware_manifest::SoundBankInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            name: CName::new(&value.name),
            is_resident: value.is_resident,
            path: ResRef::new(value.path.clone()).map_err(|_| {
                Error::from(crate::error::validation::Error::InvalidResourcePath {
                    path: value.path,
                })
            })?,
        })
    }
}
