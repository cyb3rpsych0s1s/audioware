//! Interop types for [kira].

use core::fmt;
use std::ops::Not;

use red4ext_rs::{class_kind::Scripted, types::Ref, ScriptClass};

use super::{ToEasing, Tween};

/// Interop type for [kira::spatial::emitter::EmitterSettings].
#[repr(C)]
pub struct EmitterSettings {
    pub distances: Ref<EmitterDistances>,
    pub attenuation_function: Ref<Tween>,
    pub enable_spatialization: bool,
    pub persist_until_sound_finish: bool,
}

impl Default for EmitterSettings {
    fn default() -> Self {
        Self {
            enable_spatialization: true,
            attenuation_function: Default::default(),
            distances: Default::default(),
            persist_until_sound_finish: false,
        }
    }
}

unsafe impl ScriptClass for EmitterSettings {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.EmitterSettings";
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
        match (
            self.distances
                .clone()
                .is_null()
                .not()
                .then_some(unsafe { self.distances.fields() })
                .flatten(),
            other
                .distances
                .clone()
                .is_null()
                .not()
                .then_some(unsafe { self.distances.fields() })
                .flatten(),
        ) {
            (None, Some(_)) | (Some(_), None) => return false,
            (Some(x), Some(y)) if x != y => return false,
            _ => {}
        };
        self.enable_spatialization == other.enable_spatialization
    }
}

impl Clone for EmitterSettings {
    fn clone(&self) -> Self {
        Self {
            distances: self.distances.clone(),
            attenuation_function: self.attenuation_function.clone(),
            enable_spatialization: self.enable_spatialization,
            persist_until_sound_finish: self.persist_until_sound_finish,
        }
    }
}

impl fmt::Debug for EmitterSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudiowareEmitterSettings")
            .field("attenuation_function", &self.attenuation_function.is_null())
            .field("enable_spatialization", &self.enable_spatialization)
            .finish()
    }
}

/// Interop type for [kira::spatial::emitter::EmitterDistances].
#[derive(Debug, Default, Clone, PartialEq)]
#[repr(C)]
pub struct EmitterDistances {
    pub min_distance: f32,
    pub max_distance: f32,
}

unsafe impl ScriptClass for EmitterDistances {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.EmitterDistances";
}
