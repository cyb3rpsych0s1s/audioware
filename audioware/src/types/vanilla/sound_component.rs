use red4ext_rs::{
    class_kind::Native,
    types::{CName, Cruid, IScriptable, Ref},
    ScriptClass,
};

use super::WorldTransform;

#[repr(C)]
pub struct SoundComponentBase {
    base: IScriptable,
    pub name: CName, // 0x40
    _padding0: [u8; 0x18],
    pub id: Cruid, // 0x60
    _padding1: [u8; 0x23],
    pub is_enabled: bool,                   // 0x8B
    pub is_replicable: bool,                // 0x8C
    pub parent_transform: Ref<IScriptable>, // 0x90
    _padding2: [u8; 0x20],
    pub local_transform: WorldTransform, // 0xC0
    _padding3: [u8; 0x40],
    pub audio_name: CName, // 0x120
    _padding4: [u8; 0x9],
    pub apply_obstruction: bool,            // 0x131
    pub apply_acoustic_occlusion: bool,     // 0x132
    pub apply_acoustic_repositioning: bool, // 0x133
    _padding5: [u8; 0x4],
    pub obstruction_change_time: f32, // 0x138
    _padding6: [u8; 0x1C],
    pub max_play_distance: f32, // 0x158
    _padding7: [u8; 0x24],
}

unsafe impl ScriptClass for SoundComponentBase {
    const NAME: &'static str = "gameaudioSoundComponentBase";
    type Kind = Native;
}

impl std::fmt::Debug for SoundComponentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundComponentBase")
            .field("name", &self.name.as_str())
            .field("id", &self.id)
            .field("is_enabled", &self.is_enabled)
            .field("is_replicable", &self.is_replicable)
            .field("audio_name", &self.audio_name.as_str())
            .field("apply_obstruction", &self.apply_obstruction)
            .field("apply_acoustic_occlusion", &self.apply_acoustic_occlusion)
            .field(
                "apply_acoustic_repositioning",
                &self.apply_acoustic_repositioning,
            )
            .field("obstruction_change_time", &self.obstruction_change_time)
            .field("max_play_distance", &self.max_play_distance)
            .finish()
    }
}
