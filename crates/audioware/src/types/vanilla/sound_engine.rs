use std::ops::Not;

use red4ext_rs::{
    VoidPtr,
    types::{CName, EntityId, RedHashMap},
};

#[cfg(windows)]
use crate::utils::get_base_offset;
use crate::{SoundObject, SoundObjectId, WwiseId};

/// macOS only: the id of the SoundEngine global in the Audioware block of the SDK address table (`Relocation-inl.hpp`, branch mac-rs).
/// It is a Mac-only id (the 0xC0DExxxx range the other plugins use for rows that have no Windows hash): Windows reads the cell at a fixed
/// address.
#[cfg(target_os = "macos")]
const SOUNDENGINE_GLOBAL: u32 = 0xC0DEA002;

#[repr(C)]
pub struct SoundEngine(VoidPtr);

#[repr(C)]
pub struct MetadataManager(VoidPtr);

#[repr(C)]
pub struct SoundObjectManager(VoidPtr);

#[repr(C)]
pub struct EventApplicationInterface(VoidPtr);

impl SoundEngine {
    pub fn as_ptr(&self) -> VoidPtr {
        self.0
    }
    /// # Safety
    /// can only be obtained after SoundEngine initialization.
    #[cfg(windows)]
    pub unsafe fn get() -> Self {
        const OFFSET: usize = 0x1434389F0 - 0x140000000;
        let base = get_base_offset();
        // lifecycle!("base: {:#X}", base.0 as usize);
        let qword = base.0 as usize + OFFSET;
        // lifecycle!("qword: {:#X}", qword as usize);
        let addr = unsafe { *(qword as *const u64) };
        // lifecycle!("*qword: {:#X}", addr);
        Self(addr as VoidPtr)
    }

    /// # Safety
    /// can only be obtained after SoundEngine initialization.
    ///
    /// macOS: the global that holds the `SoundEngine*` is a row of the Audioware block of the SDK address table (Mac-only id
    /// [SOUNDENGINE_GLOBAL], RVA 0x8DCC9E0; the Windows cell is .data 0x34389F0, which the code above reads through the module base).
    /// Evidence: two independent Windows/Mac function pairs load the same global, the `DefaultListener` init function
    /// (Windows 0xA2D45C = Mac 0xDD4010) and the `player_fired` function (Windows 0x9D524C = Mac 0x3E3E1D4); 118 Mac functions reference it.
    #[cfg(target_os = "macos")]
    pub unsafe fn get() -> Self {
        let cell = ::red4ext_rs::addr_hashes::resolve(SOUNDENGINE_GLOBAL);
        if cell == 0 {
            return Self(std::ptr::null_mut());
        }
        let addr = unsafe { *(cell as *const u64) };
        Self(addr as VoidPtr)
    }

    pub fn metadata_manager(&self) -> MetadataManager {
        // macOS: the global can still be null when a hook runs before Wwise created its engine; answer with a null manager, whose lookups
        // answer "invalid id", instead of reading near address 0.
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return MetadataManager(std::ptr::null_mut());
        }
        const OFFSET: usize = 0x58;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        let addr = unsafe { *(field as *const u64) };
        // lifecycle!("*field: {:#X}", addr as usize);
        MetadataManager(addr as VoidPtr)
    }

    pub fn sound_object_manager(&self) -> SoundObjectManager {
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return SoundObjectManager(std::ptr::null_mut());
        }
        const OFFSET: usize = 0x60;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        let addr = unsafe { *(field as *const u64) };
        // lifecycle!("*field: {:#X}", addr as usize);
        SoundObjectManager(addr as VoidPtr)
    }
}

impl MetadataManager {
    pub fn event_wwise_id(&self, event_name: CName) -> WwiseId {
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return WwiseId::default();
        }
        const HASH: u32 = 0x3B1D13A1;
        let get_event_wwise_id = ::red4ext_rs::addr_hashes::resolve(HASH);
        let get_event_wwise_id = unsafe {
            ::std::mem::transmute::<usize, unsafe extern "C" fn(VoidPtr, CName) -> WwiseId>(
                get_event_wwise_id,
            )
        };
        unsafe { get_event_wwise_id(self.0, event_name) }
    }
    pub fn switch_group_id(&self, switch_name: CName) -> WwiseId {
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return WwiseId::default();
        }
        const OFFSET: usize = 0x30;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        unsafe { self.wwise_lookup()(self.0, field as VoidPtr, switch_name) }
    }
    pub fn switch_id(&self, switch_value: CName) -> WwiseId {
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return WwiseId::default();
        }
        const OFFSET: usize = 0x58;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        unsafe { self.wwise_lookup()(self.0, field as VoidPtr, switch_value) }
    }
    pub fn game_parameter_id(&self, parameter_name: CName) -> WwiseId {
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return WwiseId::default();
        }
        const OFFSET: usize = 0xD0;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        unsafe { self.wwise_lookup()(self.0, field as VoidPtr, parameter_name) }
    }
    fn wwise_lookup(&self) -> unsafe extern "C" fn(VoidPtr, VoidPtr, CName) -> WwiseId {
        const HASH: u32 = 0x6AA2209F;
        let wwise_lookup = ::red4ext_rs::addr_hashes::resolve(HASH);
        unsafe {
            ::std::mem::transmute::<usize, unsafe extern "C" fn(VoidPtr, VoidPtr, CName) -> WwiseId>(
                wwise_lookup,
            )
        }
    }
}

impl SoundObjectManager {
    pub fn sound_object(&self, id: SoundObjectId) -> Option<&SoundObject> {
        #[cfg(target_os = "macos")]
        if self.0.is_null() {
            return None;
        }
        const OFFSET: usize = 0x7010;
        let map = unsafe {
            std::mem::transmute::<VoidPtr, *const RedHashMap<SoundObjectId, *const SoundObject>>(
                (self.0 as usize + OFFSET) as VoidPtr,
            )
        };
        if map.is_null() {
            return None;
        }
        let map = unsafe { &*map };
        map.iter().find_map(|(k, v)| {
            if *k == id && v.is_null().not() {
                Some(unsafe { &**v })
            } else {
                None
            }
        })
    }
}

impl EventApplicationInterface {
    /// # Safety
    ///
    /// the caller must guarantee that the pointer is valid, initialized, of correct type and non-null.
    pub unsafe fn new(inner: VoidPtr) -> Self {
        Self(inner)
    }

    pub fn entity_id(&self) -> EntityId {
        // macOS: slot 0x80. Windows `ApplyAction` calls `[vtable + 0x78]` five times and the Mac `ApplyAction` (0x3E34FD0) calls
        // `[vtable + 0x80]` five times; its other interface calls sit at the Windows slot plus 8 as well (Windows +0x70 x2 = Mac +0x78 x2,
        // Windows 0x88/0x90/0x98/0xA0 = Mac 0x90/0x98/0xA0/0xA8), which is the Itanium destructor shift. Inferred by call counts,
        // not yet confirmed from a live vtable.
        #[cfg(target_os = "macos")]
        const OFFSET: usize = 0x80;
        #[cfg(not(target_os = "macos"))]
        const OFFSET: usize = 0x78;
        let me = unsafe { *(self.0 as *const usize) };
        let getter = me + OFFSET;
        let getter =
            std::ptr::with_exposed_provenance::<unsafe extern "C" fn(VoidPtr) -> EntityId>(getter);
        unsafe { (*getter)(self.0) }
    }
}
