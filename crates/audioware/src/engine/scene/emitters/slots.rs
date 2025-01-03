use std::num::NonZero;

use dashmap::{mapref::multiple::RefMulti, DashMap};
use kira::{
    spatial::emitter::{EmitterHandle, EmitterId, EmitterSettings},
    tween::Tween,
};
use red4ext_rs::types::{CName, EntityId};

use crate::{
    engine::{scene::dilation::Dilation, tweens::IMMEDIATELY},
    utils::fails,
    Vector4,
};

use super::mods::EmitterMod;

/// Identify active [EmitterHandle]s.
/// These handles can be shared by multiple mods.
pub struct EmitterSlots {
    pub slots: DashMap<EmitterFootprint, EmitterSlot, ahash::RandomState>,
    pub marked_for_death: bool,
    pub busy: bool,
    pub last_known_position: Vector4,
    pub dilation: Dilation,
}

impl EmitterSlots {
    pub fn new(dilation: Option<f32>, busy: bool, last_known_position: Vector4) -> Self {
        Self {
            slots: DashMap::with_hasher(ahash::RandomState::new()),
            marked_for_death: false,
            busy,
            last_known_position,
            dilation: Dilation::new(dilation.unwrap_or(1.0)),
        }
    }
    pub fn get(
        &self,
        settings_hash: Option<NonZero<u64>>,
    ) -> Option<RefMulti<EmitterFootprint, EmitterSlot>> {
        self.slots
            .iter()
            .find(|x| x.key().settings_hash == settings_hash)
    }
    pub fn exists_tag(&self, tag_name: &CName) -> bool {
        self.slots
            .iter()
            .any(|x| x.value().mods.contains_key(tag_name))
    }
    pub fn insert(&mut self, key: EmitterFootprint, value: EmitterSlot) -> Option<EmitterSlot> {
        self.slots.insert(key, value)
    }
    pub fn reclaim(&mut self) {
        self.slots.iter_mut().for_each(|x| {
            x.mods.iter_mut().for_each(|mut x| {
                x.reclaim();
            })
        });
    }
    pub fn set_emitter_position(&mut self, position: Vector4) {
        self.slots.iter_mut().for_each(|mut x| {
            x.value_mut().handle.set_position(position, IMMEDIATELY);
        });
    }
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn any_playing_handle(&self) -> bool {
        self.slots.iter().any(|x| x.value().any_playing_handle())
    }

    pub fn stop_on_emitter(&mut self, event_name: CName, tag_name: CName, tween: Tween) {
        self.slots.iter_mut().for_each(|mut x| {
            if let Some(mut r#mod) = x.value_mut().mods.get_mut(&tag_name) {
                r#mod.stop_by_event_name(event_name, tween);
            }
        });
    }

    pub fn stop_emitters(&mut self, tween: Tween) {
        self.slots.iter_mut().for_each(|mut x| {
            x.value_mut().stop_emitters(tween);
        });
    }

    pub fn pause(&mut self, tween: Tween) {
        self.slots.iter_mut().for_each(|mut x| {
            x.value_mut().pause(tween);
        });
    }

    pub fn resume(&mut self, tween: Tween) {
        self.slots.iter_mut().for_each(|mut x| {
            x.value_mut().resume(tween);
        });
    }

    pub fn unregister_emitter(&mut self, tag_name: &CName) {
        self.slots.iter_mut().for_each(|mut x| {
            x.value_mut().mods.remove(tag_name);
        });
    }
    pub fn emitter_destination(&self, tag_name: &CName) -> Option<(EmitterId, Option<CName>)> {
        self.slots.iter().find_map(|x| {
            x.value()
                .mods
                .get(tag_name)
                .map(|y| (x.value().handle.id(), y.value().name))
        })
    }
    pub fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.slots.iter_mut().for_each(|mut x| {
            x.value_mut().sync_dilation(rate, tween);
        });
    }
}

/// Emitter footprint which identify a single [EmitterHandle] by its unique set of settings for a given [EntityId].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmitterFootprint {
    settings_hash: Option<NonZero<u64>>,
    pub persist_until_sounds_finish: bool,
}

impl EmitterFootprint {
    pub fn new(settings: Option<(EmitterSettings, NonZero<u64>)>) -> Self {
        Self {
            settings_hash: settings.map(|(_, x)| x),
            persist_until_sounds_finish: settings
                .map(|(x, _)| x.persist_until_sounds_finish)
                .unwrap_or(false),
        }
    }
}

impl std::hash::Hash for EmitterFootprint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.settings_hash.hash(state);
        // persist_until_sounds_finish is ignored on purpose
    }
}

/// Underlying handle to the emitter,
/// and whether sound(s) should persist until they finish playing.
#[derive(Debug)]
pub struct EmitterSlot {
    pub handle: EmitterHandle,
    pub mods: DashMap<CName, EmitterMod, ahash::RandomState>,
}

impl EmitterSlot {
    pub fn any_playing_handle(&self) -> bool {
        self.mods.iter().any(|x| x.value().any_playing_handle())
    }
    pub fn new(handle: EmitterHandle, tag_name: CName, emitter_name: Option<CName>) -> Self {
        let mods = DashMap::with_hasher(ahash::RandomState::new());
        mods.insert(tag_name, EmitterMod::new(emitter_name));
        Self { handle, mods }
    }
    pub fn stop_emitters(&mut self, tween: Tween) {
        self.mods.iter_mut().for_each(|mut x| {
            x.value_mut().stop_emitters(tween);
        });
    }

    pub fn pause(&mut self, tween: Tween) {
        self.mods.iter_mut().for_each(|mut x| {
            x.value_mut().pause(tween);
        });
    }

    pub fn resume(&mut self, tween: Tween) {
        self.mods.iter_mut().for_each(|mut x| {
            x.value_mut().resume(tween);
        });
    }

    pub fn sync_dilation(&mut self, rate: f64, tween: Tween) {
        self.mods.iter_mut().for_each(|mut x| {
            x.value_mut().sync_dilation(rate, tween);
        });
    }
}

impl From<(&EntityId, &red4ext_rs::types::Ref<crate::EmitterSettings>)> for EmitterFootprint {
    fn from(value: (&EntityId, &red4ext_rs::types::Ref<crate::EmitterSettings>)) -> Self {
        let settings_hash = if value.1.is_null() {
            None::<NonZero<u64>>
        } else {
            match ahash::RandomState::new().hash_one(unsafe { value.1.fields() }) {
                0 => {
                    fails!("emitter settings hash should not be 0");
                    None
                }
                hash => Some(NonZero::new(hash).unwrap()),
            }
        };
        let persist_until_sounds_finish = if value.1.is_null() {
            false
        } else {
            unsafe { value.1.fields() }
                .unwrap()
                .persist_until_sounds_finish
        };
        Self {
            settings_hash,
            persist_until_sounds_finish,
        }
    }
}
