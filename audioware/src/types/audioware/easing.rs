use red4ext_rs::NativeRepr;

/// Interop type for [kira::tween::Easing].
#[allow(clippy::enum_variant_names, dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum Easing {
    #[default]
    InPowf = 0,
    OutPowf = 1,
    InOutPowf = 2,
}
unsafe impl NativeRepr for Easing {
    const NAME: &'static str = "Audioware.Easing";
}
