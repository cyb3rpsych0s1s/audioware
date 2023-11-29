use red4ext_rs::types::CName;

use audioware_macros::FromMemory;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct SceneAudioEvent {
    serializable: [u8; 48],
    pub id: SceneEventId,
    pub r#type: EventType,
    pub start_time: u32,
    pub duration: u32,
    pub execution_tag_flags: u8,
    unk45: [u8; 3],
    pub scaling_data: [u8; 16],
    pub performer: PerformerId,
    #[allow(non_snake_case)]
    unk5C: [u8; 4],
    pub audio_event_name: CName,
    pub ambient_unique_name: CName,
    pub emitter_name: CName,
    pub fast_forward_support: AudioFastForwardSupport,
    unk79: [u8; 7],
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SceneEventId(u64);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct EventType(u32);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PerformerId(u32);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
pub enum AudioFastForwardSupport {
    MuteDuringFastForward = 1,
    DontMuteDuringFastForward = 2,
}

// unsafe impl FromMemory for AudioEvent {
//     fn from_memory(address: usize) -> Self {
//         let id: SceneEventId = unsafe {
//             core::slice::from_raw_parts::<SceneEventId>((address + 0x30) as *const SceneEventId, 1)
//                 .get_unchecked(0)
//                 .clone()
//         };
//         todo!()
//     }
// }
