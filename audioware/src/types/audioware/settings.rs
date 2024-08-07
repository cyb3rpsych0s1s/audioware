use core::fmt;

use red4ext_rs::{class_kind::Native, types::Ref, NativeRepr, ScriptClass};

use super::{ToEasing, Tween};

#[derive(Default)]
#[repr(C)]
pub struct EmitterSettings {
    pub distances: EmitterDistances,
    pub attenuation_function: Ref<Tween>,
    pub enable_spatialization: bool,
    pub persist_until_sounds_finish: bool,
}

unsafe impl NativeRepr for EmitterSettings {
    const NAME: &'static str = "Audioware.EmitterSettings";
}

unsafe impl ScriptClass for EmitterSettings {
    type Kind = Native;
    const NAME: &'static str = <Self as NativeRepr>::NAME;
}

impl PartialEq for EmitterSettings {
    fn eq(&self, other: &Self) -> bool {
        match (
            self.attenuation_function.clone().into_easing(),
            other.attenuation_function.clone().into_easing(),
        ) {
            (Some(_), None) | (None, Some(_)) => return false,
            (Some(x), Some(y)) if x != y => return false,
            _ => {}
        };
        self.distances == other.distances
            && self.enable_spatialization == other.enable_spatialization
            && self.persist_until_sounds_finish == other.persist_until_sounds_finish
    }
}

impl Clone for EmitterSettings {
    fn clone(&self) -> Self {
        Self {
            distances: self.distances.clone(),
            attenuation_function: self.attenuation_function.clone(),
            enable_spatialization: self.enable_spatialization,
            persist_until_sounds_finish: self.persist_until_sounds_finish,
        }
    }
}

impl fmt::Debug for EmitterSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudiowareEmitterSettings")
            .field("distances", &self.distances)
            .field("attenuation_function", &self.attenuation_function.is_null())
            .field("enable_spatialization", &self.enable_spatialization)
            .field(
                "persist_until_sounds_finish",
                &self.persist_until_sounds_finish,
            )
            .finish()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
#[repr(C)]
pub struct EmitterDistances {
    pub min_distance: f32,
    pub max_distance: f32,
}

unsafe impl NativeRepr for EmitterDistances {
    const NAME: &'static str = "Audioware.EmitterDistances";
}

unsafe impl ScriptClass for EmitterDistances {
    type Kind = Native;
    const NAME: &'static str = <Self as NativeRepr>::NAME;
}

impl From<EmitterSettings> for kira::spatial::emitter::EmitterSettings {
    fn from(
        EmitterSettings {
            distances,
            attenuation_function,
            enable_spatialization,
            persist_until_sounds_finish,
        }: EmitterSettings,
    ) -> Self {
        Self {
            distances: distances.into(),
            attenuation_function: attenuation_function.into_easing(),
            enable_spatialization,
            persist_until_sounds_finish,
        }
    }
}

impl From<EmitterDistances> for kira::spatial::emitter::EmitterDistances {
    fn from(
        EmitterDistances {
            min_distance,
            max_distance,
        }: EmitterDistances,
    ) -> Self {
        Self {
            min_distance,
            max_distance,
        }
    }
}
