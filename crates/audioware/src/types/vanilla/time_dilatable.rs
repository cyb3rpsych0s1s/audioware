use red4ext_rs::{
    RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, IScriptable, Ref},
};

use super::GameObject;

#[repr(C)]
pub struct TimeDilatable {
    pub base: GameObject,
    unk240: [u8; 0x260 - 0x240], // 0x240
}

unsafe impl ScriptClass for TimeDilatable {
    const NAME: &'static str = "gameTimeDilatable";
    type Kind = Native;
}

impl AsRef<IScriptable> for TimeDilatable {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

pub trait AsTimeDilatable {
    fn get_time_dilation_value(&self) -> f32;
    fn has_individual_time_dilation(&self, reason: CName) -> bool;
    fn is_ignoring_global_time_dilation(&self) -> bool;
    fn is_ignoring_time_dilation(&self) -> bool;
}

impl AsTimeDilatable for Ref<TimeDilatable> {
    fn get_time_dilation_value(&self) -> f32 {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(TimeDilatable::NAME)).unwrap();
        let method = cls
            .get_method(CName::new("GetTimeDilationValue"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, f32>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }

    fn has_individual_time_dilation(&self, reason: CName) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(TimeDilatable::NAME)).unwrap();
        let method = cls
            .get_method(CName::new("HasIndividualTimeDilation"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }.map(AsRef::as_ref), (reason,))
            .unwrap()
    }

    fn is_ignoring_global_time_dilation(&self) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(TimeDilatable::NAME)).unwrap();
        let method = cls
            .get_method(CName::new("IsIgnoringGlobalTimeDilation"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }

    fn is_ignoring_time_dilation(&self) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(TimeDilatable::NAME)).unwrap();
        let method = cls
            .get_method(CName::new("IsIgnoringTimeDilation"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}
