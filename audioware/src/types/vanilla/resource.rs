use std::fmt;

use red4ext_rs::{
    class_kind::Native,
    types::{ISerializable, Ref, ResRef},
    NativeRepr, ScriptClass,
};

#[derive(Debug)]
#[repr(C)]
pub struct CResource {
    base: ISerializable,
    pub path: ResRef,
    pub cooking_platform: ECookingPlatform,
}

unsafe impl ScriptClass for CResource {
    type Kind = Native;
    const NAME: &'static str = "CResource";
}

const UO: usize = 0x50 - 0x40;

#[repr(C)]
pub struct JsonResource {
    base: CResource,
    pub root: Ref<ISerializable>,
}

impl fmt::Debug for JsonResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JsonResource")
            .field("base", &self.base)
            .finish_non_exhaustive()
    }
}

unsafe impl ScriptClass for JsonResource {
    type Kind = Native;
    const NAME: &'static str = "JsonResource";
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum ECookingPlatform {
    PLATFORM_None = 0,
    PLATFORM_PC = 1,
    PLATFORM_XboxOne = 2,
    PLATFORM_PS4 = 3,
    PLATFORM_PS5 = 4,
    PLATFORM_XSX = 5,
    PLATFORM_WindowsServer = 6,
    PLATFORM_LinuxServer = 7,
    PLATFORM_GGP = 8,
}

unsafe impl NativeRepr for ECookingPlatform {
    const NAME: &'static str = "ECookingPlatform";
}
