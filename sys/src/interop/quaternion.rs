use kira::tween::Value;
use red4ext_rs::conv::NativeRepr;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Quaternion {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub r: f32,
}

unsafe impl NativeRepr for Quaternion {
    const NAME: &'static str = "Quaternion";
}

impl From<self::Quaternion> for Value<mint::Quaternion<f32>> {
    fn from(val: self::Quaternion) -> Self {
        Value::Fixed(mint::Quaternion {
            v: mint::Vector3 {
                x: val.i,
                y: val.j,
                z: val.k,
            },
            s: val.r,
        })
    }
}

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(std::mem::size_of::<super::Quaternion>(), 0x10);
    }
}
