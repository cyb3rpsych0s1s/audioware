use red4ext_rs::types::CName;

use audioware_types::FromMemory;

/// see [RED4ext::ent::SoundEvent](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/ent/SoundEvent.hpp).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SoundEvent {
    pub(crate) event_name: CName,
    // TODO ...
}

unsafe impl FromMemory for SoundEvent {
    fn from_memory(address: usize) -> Self {
        let event_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        Self { event_name }
    }
}
