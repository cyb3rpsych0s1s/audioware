use red4ext_rs::{
    RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable, Method, Ref},
};

use crate::{IGameSystem, Transform};

pub trait AsCameraSystem {
    fn get_active_camera_world_transform(&self, transform: Transform) -> bool;
}

const PADDING_48: usize = 0x730 - 0x48;

#[derive(Debug)]
#[repr(C)]
pub struct CameraSystem {
    base: ICameraSystem,
    unk48: [u8; PADDING_48], // 48
}

unsafe impl ScriptClass for CameraSystem {
    type Kind = Native;
    const NAME: &'static str = "gameCameraSystem";
}
impl AsRef<IScriptable> for CameraSystem {
    fn as_ref(&self) -> &IScriptable {
        self.base.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ICameraSystem {
    base: IGameSystem,
}

unsafe impl ScriptClass for ICameraSystem {
    type Kind = Native;
    const NAME: &'static str = "gameICameraSystem";
}

impl AsCameraSystem for Ref<CameraSystem> {
    #[allow(unused_mut, reason = "transform is an out parameter")]
    fn get_active_camera_world_transform(&self, mut transform: Transform) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(CameraSystem::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetActiveCameraWorldTransform"))
            .ok()
            .unwrap();
        match unsafe { self.instance() } {
            Some(x) => method
                .as_function()
                .execute::<_, bool>(Some(x.as_ref()), (transform,))
                .unwrap_or(false),
            None => false,
        }
    }
}
