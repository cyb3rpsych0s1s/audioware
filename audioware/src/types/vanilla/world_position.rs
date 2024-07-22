use std::fmt;

use red4ext_rs::NativeRepr;

use super::FixedPoint;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[repr(C, align(4))]
pub struct WorldPosition {
    pub x: FixedPoint, // 0x0
    pub y: FixedPoint, // 0x4
    pub z: FixedPoint, // 0x8
}

unsafe impl NativeRepr for WorldPosition {
    const NAME: &'static str = "WorldPosition";
}

impl fmt::Display for WorldPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[x: {}, y: {}, z: {}]", self.x, self.y, self.z)
    }
}
