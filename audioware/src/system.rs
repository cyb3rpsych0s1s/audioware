use red4rs::{
    invocable::InvokeError,
    systems::RttiSystem,
    types::{CName, ScriptClass, Scripted},
};
use red4rs_bindings::ScriptableSystem;

#[repr(C)]
pub struct AudiowareSystem {
    base: ScriptableSystem,
}

unsafe impl ScriptClass for AudiowareSystem {
    type Kind = Scripted;
    const CLASS_NAME: &'static str = "Audioware.AudiowareSystem";
}

// pub trait Yolo {
//     fn yolo(&self);
// }

// impl Yolo for AudiowareSystem {
//     fn yolo(&self) {
//         let _ = red4rs::call!(self, "Yolo"() -> ());
//     }
// }

impl AudiowareSystem {
    pub fn yolo(&self) {
        // let _ = red4rs::call!(self, "Yolo"() -> ());
        {
            RttiSystem::get()
                .get_function(CName::new("Yolo"))
                .ok_or(InvokeError::FunctionNotFound("Yolo"))
                .unwrap()
                .execute::<(), ()>(None, ());
        };
    }
}

// impl AsRef<IScriptable> for AudiowareSystem {
//     fn as_ref(&self) -> &IScriptable {
//         unsafe {
//             std::mem::transmute::<&[u8; 0x40], &IScriptable>(
//                 &self.base._padding0[0..0x40].try_into().expect("yolo"),
//             )
//         }
//     }
// }
