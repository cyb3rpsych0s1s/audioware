use red4ext_rs::NativeRepr;

use crate::{Quaternion, Vector4, WorldPosition, WorldTransform};

#[derive(Debug, Default, Clone)]
#[repr(C, align(16))]
pub struct Transform {
    pub position: Vector4,       // 00
    pub orientation: Quaternion, // 10
}

unsafe impl NativeRepr for Transform {
    const NAME: &'static str = "Transform";
}

impl From<Transform> for WorldTransform {
    fn from(value: Transform) -> Self {
        Self {
            position: WorldPosition {
                x: value.position.x.into(),
                y: value.position.y.into(),
                z: value.position.z.into(),
            },
            orientation: value.orientation,
        }
    }
}
