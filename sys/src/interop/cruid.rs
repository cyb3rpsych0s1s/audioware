use cxx::type_id;
use cxx::ExternType;
use red4ext_rs::conv::NativeRepr;
use red4ext_rs::types::CName;

use super::hash::fnv1a32;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Cruid {
    unk00: u64,
}

impl From<&str> for Cruid {
    /// following psiberx recommendations, see [Discord](https://discord.com/channels/717692382849663036/717720094196760760/1208391892119719946)
    fn from(value: &str) -> Self {
        0xF000000000000000 | (fnv1a32(value) << 2)
    }
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
