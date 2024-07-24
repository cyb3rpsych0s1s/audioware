use red4ext_rs::{
    class_kind::Scripted,
    types::{CName, Ref},
    RttiSystem, ScriptClass,
};

#[repr(C)]
pub struct AudiowareService;

unsafe impl ScriptClass for AudiowareService {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.AudiowareService";
}

pub trait AsAudiowareService {
    /// `public static func GetInstance() -> ref<AudiowareService>`
    fn get_instance() -> Ref<AudiowareService>;
}

impl AsAudiowareService for AudiowareService {
    fn get_instance() -> Ref<AudiowareService> {
        let rtti = RttiSystem::get();
        let methods = rtti.get_global_functions();
        let method = methods
            .iter()
            .find(|x| x.name() == CName::new("GetInstance;"))
            .unwrap();
        method
            .execute::<_, Ref<AudiowareService>>(None, ())
            .unwrap()
    }
}
