//! Interop types for [kira].

use core::fmt;
use std::{hash::Hash, ops::Not};

use red4ext_rs::{
    class_kind::Scripted,
    types::{CName, Opt, Ref},
    ScriptClass,
};

use super::{ElasticTween, LinearTween, ToEasing, Tween};

/// Interop type for [kira::spatial::emitter::EmitterSettings].
#[repr(C)]
pub struct EmitterSettings {
    pub distances: Ref<EmitterDistances>,
    pub attenuation_function: Ref<Tween>,
    pub enable_spatialization: bool,
    pub persist_until_sounds_finish: bool,
    pub override_emitter_name: Opt<CName>,
}

impl Default for EmitterSettings {
    fn default() -> Self {
        Self {
            enable_spatialization: true,
            attenuation_function: Default::default(),
            distances: Default::default(),
            persist_until_sounds_finish: false,
            override_emitter_name: Default::default(),
        }
    }
}

unsafe impl ScriptClass for EmitterSettings {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.EmitterSettings";
}

impl Hash for EmitterSettings {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let distance = self
            .distances
            .is_null()
            .not()
            .then_some(unsafe { self.distances.fields() }.unwrap().clone());
        distance.hash(state);
        match self
            .attenuation_function
            .is_null()
            .not()
            .then_some(self.attenuation_function.clone())
        {
            None => {
                None::<crate::Tween>.hash(state);
            }
            Some(x) if x.is_a::<LinearTween>() => {
                let x = unsafe { std::mem::transmute::<&Ref<Tween>, &Ref<LinearTween>>(&x) };
                unsafe { x.fields() }.unwrap().hash(state);
            }
            Some(x) if x.is_a::<ElasticTween>() => {
                let x = unsafe { std::mem::transmute::<&Ref<Tween>, &Ref<ElasticTween>>(&x) };
                unsafe { x.fields() }.unwrap().hash(state);
            }
            _ => {
                unreachable!("unknown attenuation function");
            }
        };

        self.enable_spatialization.hash(state);
        self.persist_until_sounds_finish.hash(state);
        // self.override_emitter_name is ignored on purpose
    }
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
            persist_until_sounds_finish: self.persist_until_sounds_finish,
            override_emitter_name: self.override_emitter_name,
        }
    }
}

impl fmt::Debug for EmitterSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudiowareEmitterSettings")
            .field("distances", &self.attenuation_function.is_null())
            .field("attenuation_function", &self.attenuation_function.is_null())
            .field("enable_spatialization", &self.enable_spatialization)
            .field(
                "persist_until_sounds_finish",
                &self.persist_until_sounds_finish,
            )
            .field(
                "override_emitter_name",
                &self
                    .override_emitter_name
                    .into_option()
                    .map(|x| x.as_str())
                    .unwrap_or("None"),
            )
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

impl Hash for EmitterDistances {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ((self.min_distance * 100.) as u64).hash(state);
        ((self.max_distance * 100.) as u64).hash(state);
    }
}
