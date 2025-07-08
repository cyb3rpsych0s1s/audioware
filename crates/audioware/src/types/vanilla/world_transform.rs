use std::fmt;

use red4ext_rs::{NativeRepr, RttiSystem, types::CName};

use super::{Quaternion, WorldPosition};

#[derive(Debug, Default, Clone)]
#[repr(C, align(16))]
pub struct WorldTransform {
    pub position: WorldPosition, // 0x0
    pub orientation: Quaternion, // 0x10
}

unsafe impl NativeRepr for WorldTransform {
    const NAME: &'static str = "WorldTransform";
}

impl WorldTransform {
    pub fn get_inverse(&self) -> Self {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(WorldTransform::NAME)).unwrap();
        let methods = cls.static_methods();
        let method = methods
            .iter()
            .find(|x| x.as_function().name() == CName::new("GetInverse"))
            .unwrap();
        method
            .as_function()
            .execute::<(WorldTransform,), Self>(None, (self.clone(),))
            .unwrap()
    }
}

impl fmt::Display for WorldTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "position: {}, orientation: {}",
            self.position, self.orientation
        )
    }
}
