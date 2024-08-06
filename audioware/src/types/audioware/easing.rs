use red4ext_rs::NativeRepr;

#[allow(clippy::enum_variant_names, dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum AudiowareEasing {
    #[default]
    InPowf = 0,
    OutPowf = 1,
    InOutPowf = 2,
}
unsafe impl NativeRepr for AudiowareEasing {
    const NAME: &'static str = "AudiowareEasing";
}
