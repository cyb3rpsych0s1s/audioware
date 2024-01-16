use kira::tween::Value;
use mint::Vector3;
use red4ext_rs::conv::NativeRepr;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

unsafe impl NativeRepr for Vector4 {
    const NAME: &'static str = "Vector4";
}

impl From<Vector4> for Value<Vector3<f32>> {
    fn from(val: Vector4) -> Self {
        Value::Fixed(Vector3 {
            x: val.x,
            y: val.y,
            z: val.z,
        })
    }
}
