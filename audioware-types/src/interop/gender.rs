use red4ext_rs::conv::NativeRepr;

#[derive(Default, Clone, Copy, strum_macros::Display)]
#[repr(i64)]
pub enum PlayerGender {
    #[default]
    Female = 0,
    Male = 1,
}

unsafe impl NativeRepr for PlayerGender {
    const NAME: &'static str = "Codeware.Localization.PlayerGender";
}
