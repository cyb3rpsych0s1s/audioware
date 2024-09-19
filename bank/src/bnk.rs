use red4ext_rs::{
    class_kind::Native, types::{CName, ResRef}, NativeRepr, ScriptClass
};

use crate::Error;

#[derive(Debug, Default, Clone)]
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

unsafe impl NativeRepr for SoundBankInfo {
    const NAME: &'static str = "SoundBankInfo";
}

impl TryFrom<(CName, audioware_manifest::SoundBankInfo)> for self::SoundBankInfo {
    type Error = Error;

    fn try_from(
        (key, value): (CName, audioware_manifest::SoundBankInfo),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            name: key,
            is_resident: value.is_resident,
            path: ResRef::new(value.path.clone()).map_err(|_| {
                Error::from(crate::error::validation::Error::InvalidResourcePath {
                    path: value.path,
                })
            })?,
        })
    }
}
