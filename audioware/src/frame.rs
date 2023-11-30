use cxx::type_id;
use cxx::ExternType;
// use red4ext_rs::conv::NativeRepr;

const PAD: usize = 0x62 - 0x08;

#[repr(C)]
#[allow(non_snake_case)]
pub(crate) struct StackFrame {
    pub code: i64,
    pad: [u8; PAD],
    pub currentParam: u8,
    useDirectData: bool,
}

unsafe impl ExternType for StackFrame {
    type Id = type_id!("RED4ext::CStackFrame");
    type Kind = cxx::kind::Opaque;
}
