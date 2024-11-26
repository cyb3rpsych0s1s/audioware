use std::fmt;

use red4ext_rs::NativeRepr;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
#[repr(C, align(16))]
pub struct Quaternion {
    pub i: f32, // 0x0
    pub j: f32, // 0x4
    pub k: f32, // 0x8
    pub r: f32, // 0xC
}

unsafe impl NativeRepr for Quaternion {
    const NAME: &'static str = "Quaternion";
}

impl From<glam::Quat> for Quaternion {
    fn from(value: glam::Quat) -> Self {
        Self {
            i: value.x,
            j: value.y,
            k: value.z,
            r: value.w,
        }
    }
}

impl From<mint::Quaternion<f32>> for Quaternion {
    fn from(value: mint::Quaternion<f32>) -> Self {
        Self {
            i: value.v.x,
            j: value.v.y,
            k: value.v.z,
            r: value.s,
        }
    }
}

impl From<Quaternion> for mint::Quaternion<f32> {
    fn from(value: Quaternion) -> Self {
        Self {
            v: mint::Vector3 {
                x: value.i,
                y: value.j,
                z: value.k,
            },
            s: value.r,
        }
    }
}

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ i: {}, j: {}, k: {}, r: {} ]",
            self.i, self.j, self.k, self.r
        )
    }
}
