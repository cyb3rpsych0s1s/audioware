use std::{fmt, ops::Not};

use red4ext_rs::{
    class_kind::Native,
    types::{CName, Cruid, IScriptable, Ref},
    ScriptClass,
};

use super::Event;

const PADDING_44: usize = 0x48 - 0x44;
const PADDING_58: usize = 0x68 - 0x58;
const PADDING_83: usize = 0x98 - 0x83;

#[repr(C)]
pub struct SpawnEffectEvent {
    base: Event,                           // 40
    pub e3hack_defer_count: u32,           // 40
    unk44: [u8; PADDING_44],               // 44
    pub blackboard: Ref<EffectBlackboard>, // 48
    unk58: [u8; PADDING_58],               // 58
    pub effect_name: CName,                // 68
    pub id_for_randomized_effect: Cruid,   // 70
    pub effect_instance_name: CName,       // 78
    pub persist_on_detach: bool,           // 80
    pub break_all_loops: bool,             // 81
    pub break_all_on_destroy: bool,        // 82
    unk83: [u8; PADDING_83],               // 83
}

unsafe impl ScriptClass for SpawnEffectEvent {
    type Kind = Native;
    const NAME: &'static str = "entSpawnEffectEvent";
}

impl fmt::Debug for SpawnEffectEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpawnEffectEvent")
            .field("base", &self.base)
            .field("e3hack_defer_count", &self.e3hack_defer_count)
            .field("blackboard", &self.blackboard.is_null().not())
            .field("effect_name", &self.effect_name)
            .field("id_for_randomized_effect", &self.id_for_randomized_effect)
            .field("effect_instance_name", &self.effect_instance_name)
            .field("persist_on_detach", &self.persist_on_detach)
            .field("break_all_loops", &self.break_all_loops)
            .field("break_all_on_destroy", &self.break_all_on_destroy)
            .finish()
    }
}

const PADDING_40: usize = 0x70 - 0x40;

#[repr(C)]
pub struct EffectBlackboard {
    base: IScriptable,       // 0
    unk40: [u8; PADDING_40], // 40
}

unsafe impl ScriptClass for EffectBlackboard {
    type Kind = Native;
    const NAME: &'static str = "worldEffectBlackboard";
}

impl AsRef<IScriptable> for EffectBlackboard {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}
