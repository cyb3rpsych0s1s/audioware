#![feature(arbitrary_self_types)]

pub mod interop;

/// # Safety
/// this is only safe as long as it matches memory representation specified in [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).
pub unsafe trait FromMemory {
    fn from_memory(address: usize) -> Self;
}
