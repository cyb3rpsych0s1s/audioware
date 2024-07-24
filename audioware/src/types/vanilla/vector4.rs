use std::{fmt, ops::Add};

use red4ext_rs::NativeRepr;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
#[repr(C, align(16))]
pub struct Vector4 {
    pub x: f32, // 0x0
    pub y: f32, // 0x4
    pub z: f32, // 0x8
    pub w: f32, // 0xC
}

unsafe impl NativeRepr for Vector4 {
    const NAME: &'static str = "Vector4";
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl From<Vector4> for mint::Vector3<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector4> for glam::Vec3 {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<glam::Vec3> for Vector4 {
    fn from(value: glam::Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: 1.,
        }
    }
}

impl From<mint::Vector4<f32>> for Vector4 {
    fn from(value: mint::Vector4<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<Vector4> for mint::Vector4<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ x: {}, y: {}, z: {}, w: {} ]",
            self.x, self.y, self.z, self.w
        )
    }
}
