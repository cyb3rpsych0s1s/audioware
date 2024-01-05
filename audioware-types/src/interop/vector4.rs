use red4ext_rs::conv::NativeRepr;

#[derive(Debug, Default)]
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

impl From<Vector4> for glam::Vec3 {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
