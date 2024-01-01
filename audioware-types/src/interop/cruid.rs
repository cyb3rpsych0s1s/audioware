use cxx::type_id;
use cxx::ExternType;
use red4ext_rs::conv::NativeRepr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Cruid {
    unk00: u64,
}

impl From<u64> for Cruid {
    fn from(unk00: u64) -> Self {
        Self { unk00 }
    }
}

impl From<Cruid> for u64 {
    fn from(value: Cruid) -> Self {
        value.unk00
    }
}

unsafe impl ExternType for Cruid {
    type Id = type_id!("RED4ext::CRUID");
    type Kind = cxx::kind::Trivial;
}

unsafe impl NativeRepr for Cruid {
    const NAME: &'static str = "CRUID";
}
