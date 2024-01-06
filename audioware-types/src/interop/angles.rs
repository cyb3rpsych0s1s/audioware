use red4ext_rs::conv::NativeRepr;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct EulerAngles {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

unsafe impl NativeRepr for EulerAngles {
    const NAME: &'static str = "EulerAngles";
}

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(std::mem::size_of::<super::EulerAngles>(), 0xC);
    }
}
