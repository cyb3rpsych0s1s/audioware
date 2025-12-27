use std::ops::Not;

use red4ext_rs::{
    VoidPtr,
    types::{CName, EntityId, RedHashMap},
};

use crate::{SoundObject, SoundObjectId, WwiseId, utils::get_base_offset};

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

    pub fn metadata_manager(&self) -> MetadataManager {
        const OFFSET: usize = 0x58;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        let addr = unsafe { *(field as *const u64) };
        // lifecycle!("*field: {:#X}", addr as usize);
        MetadataManager(addr as VoidPtr)
    }

    pub fn sound_object_manager(&self) -> SoundObjectManager {
        const OFFSET: usize = 0x60;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        let addr = unsafe { *(field as *const u64) };
        // lifecycle!("*field: {:#X}", addr as usize);
        SoundObjectManager(addr as VoidPtr)
    }
}

impl MetadataManager {
    pub fn wwise_id(&self, event_name: CName) -> WwiseId {
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
        const OFFSET: usize = 0x30;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        unsafe { self.wwise_lookup()(self.0, field as VoidPtr, switch_name) }
    }
    pub fn switch_id(&self, switch_value: CName) -> WwiseId {
        const OFFSET: usize = 0x58;
        let field = self.0 as usize + OFFSET;
        // lifecycle!("field: {:#X}", field);
        unsafe { self.wwise_lookup()(self.0, field as VoidPtr, switch_value) }
    }
    pub fn game_parameter_id(&self, parameter_name: CName) -> WwiseId {
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
        const OFFSET: usize = 0x78;
        let me = unsafe { *(self.0 as *const usize) };
        let getter = me + OFFSET;
        let getter =
            std::ptr::with_exposed_provenance::<unsafe extern "C" fn(VoidPtr) -> EntityId>(getter);
        unsafe { (*getter)(self.0) }
    }

    pub fn emitter_name(&self) -> CName {
        const OFFSET: usize = 0x70;
        let me = unsafe { *(self.0 as *const usize) };
        let getter = me + OFFSET;
        let getter =
            std::ptr::with_exposed_provenance::<unsafe extern "C" fn(VoidPtr) -> CName>(getter);
        unsafe { (*getter)(self.0) }
    }

    pub fn metadata_name(&self) -> CName {
        const OFFSET: usize = 0x88;
        let me = unsafe { *(self.0 as *const usize) };
        let getter = me + OFFSET;
        let getter =
            std::ptr::with_exposed_provenance::<unsafe extern "C" fn(VoidPtr) -> CName>(getter);
        unsafe { (*getter)(self.0) }
    }
}
