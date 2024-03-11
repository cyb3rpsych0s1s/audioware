use cxx::type_id;
use cxx::ExternType;
use red4ext_rs::conv::NativeRepr;

use super::hash::fnv1a32;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Cruid {
    unk00: u64,
}

impl From<&str> for Cruid {
    /// following psiberx recommendations, see [Discord](https://discord.com/channels/717692382849663036/717720094196760760/1208391892119719946)
    fn from(value: &str) -> Self {
        Self {
            unk00: 0xF000000000000000 | ((fnv1a32(value) as u64) << 2_u64),
        }
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

#[cfg(test)]
mod tests {
    use super::Cruid;

    #[test]
    fn conversion() {
        let id: &str = "ono_hhuh";
        let cruid: Cruid = Cruid::from(id);
        assert_eq!(cruid.unk00, 17293822576383693964);

        let id: &str = "get_this_damn_UI_out_of_my_face";
        let cruid: Cruid = Cruid::from(id);
        assert_eq!(cruid.unk00, 17293822579811696368);
    }
}
