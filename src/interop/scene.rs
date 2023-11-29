use red4ext_rs::types::CName;

use audioware_macros::FromMemory;

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct SceneAudioEvent {
    serializable: [u8; 0x30], // 48 bits
    pub id: SceneEventId,
    pub r#type: EventType,
    pub start_time: u32,
    pub duration: u32,
    pub execution_tag_flags: u8,
    unk45: [u8; 0x48 - 0x45], // 3 bits
    pub scaling_data: [u8; 0x10], // 16 bits
    pub performer: PerformerId,
    unk5C: [u8; 0x60 - 0x5C], // 4 bits
    pub audio_event_name: CName,
    pub ambient_unique_name: CName,
    pub emitter_name: CName,
    pub fast_forward_support: AudioFastForwardSupport,
    unk79: [u8; 0x80 - 0x79], // 7 bits
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
