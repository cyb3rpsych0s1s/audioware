use red4ext_rs::{
    types::{CName, RedArray},
    NativeRepr,
};

#[repr(C, align(8))]
pub struct WorldRuntimeScene {
    pub _padding0: [u8; 0x4B8],
}

unsafe impl NativeRepr for WorldRuntimeScene {
    const NAME: &'static str = "worldRuntimeScene";
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ECustomCameraTarget {
    EcctvAll = 0,
    EcctvOnlyOffscreen = 1,
    EcctvOnlyOnscreen = 2,
}

unsafe impl NativeRepr for ECustomCameraTarget {
    const NAME: &'static str = "ECustomCameraTarget";
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RenderSceneLayerMask(pub [u8; 0x1]);

#[repr(C, align(8))]
pub struct GamePlayerSocket {
    pub _padding0: [u8; 0x28],
}

unsafe impl NativeRepr for GamePlayerSocket {
    const NAME: &'static str = "gamePlayerSocket";
}

#[repr(C, align(8))]
pub struct RedTagList {
    pub tags: RedArray<CName>, // 0x0
}

unsafe impl NativeRepr for RedTagList {
    const NAME: &'static str = "redTagList";
}
