mod frame;
mod module;
pub use module::*;

/// Read a struct directly from memory at given offset.
///
/// # Safety
/// this is only safe as long as it matches memory representation specified in [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).
///
/// It usually implies that:
/// - it must be annotated with `#[repr(C)]` to guarantee that the order of its fields will be preserved
/// - fields are defined in correct order
/// - padding is preserved
/// - fields type match underlying memory representation
pub unsafe trait FromMemory {
    fn from_memory(address: usize) -> Self;
}

/// Read native function parameters from `C` stack-frame
///
/// # Safety
/// this is only safe as long as it matches function parameters specified in [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK).
///
/// It usually implies that:
/// - parameters must be read in order
/// - parameters type match underlying memory representation
pub unsafe trait FromFrame {
    fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self;
}
