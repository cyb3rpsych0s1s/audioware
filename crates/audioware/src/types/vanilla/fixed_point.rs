use std::fmt;

use red4ext_rs::NativeRepr;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[repr(C, align(4))]
pub struct FixedPoint {
    pub bits: i32, // 0x0
}

unsafe impl NativeRepr for FixedPoint {
    const NAME: &'static str = "FixedPoint";
}

impl fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

impl From<f32> for FixedPoint {
    fn from(value: f32) -> Self {
        Self {
            bits: value.round() as i32,
        }
    }
}
