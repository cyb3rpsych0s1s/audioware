use cxx::type_id;
use cxx::ExternType;

/// padding between [`StackFrame::code`] and [`StackFrame::currentParam`]
const PAD: usize = 0x62 - 0x08;

/// see [RED4ext::CStackFrame](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Stack.hpp#L111)
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
