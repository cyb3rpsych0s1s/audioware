use red4ext_rs::types::CName;

use crate::FromMemory;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MusicEvent {
    pub(crate) event_name: CName,
}

unsafe impl FromMemory for MusicEvent {
    fn from_memory(address: usize) -> Self {
        let event_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        Self { event_name }
    }
}
