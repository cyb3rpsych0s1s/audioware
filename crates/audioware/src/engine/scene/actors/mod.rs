use std::{collections::HashSet, sync::LazyLock};

use dashmap::DashMap;
use kira::sound::FromFileError;
use parking_lot::RwLock;
use red4ext_rs::types::{Cruid, EntityId};

use crate::{
    engine::{
        scene::{actors::slot::ActorSlot, emitters::Emitter},
        traits::{
            DualHandles, clear::Clear, pause::Pause, reclaim::Reclaim, resume::Resume, stop::Stop,
        },
    },
    error::Error,
};

pub mod slot;

#[allow(clippy::type_complexity)]
pub static ACTORS: LazyLock<RwLock<HashSet<EntityId>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub struct Actors {
    pub v: DualHandles<Cruid, (), FromFileError>,
    pub holocall: DualHandles<Cruid, (), FromFileError>,
    pub emitters: DashMap<EntityId, ActorSlot>,
}

impl Actors {
    pub fn with_capacity(capacity: usize) -> Self {
        *ACTORS.write() = HashSet::with_capacity(capacity);
        Self {
            v: Default::default(),
            holocall: Default::default(),
            emitters: Default::default(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.emitters.is_empty()
    }
    pub fn exists(&self, entity_id: &EntityId) -> bool {
        self.emitters.contains_key(entity_id)
    }
    pub fn sync_emitters(&mut self) -> Result<(), Error> {
        if self.emitters.is_empty() {
            return Ok(());
        }
        self.emitters.retain(|k, v| {
            if !v.any_playing_handle() {
                return false;
            }
            let Ok(position) = Emitter::position(*k) else {
                return false;
            };
            v.last_known_position = position;
            v.set_emitter_position(position);
            true
        });
        Ok(())
    }
}

impl Stop for Actors {
    fn stop(&mut self, tween: kira::Tween) {
        self.v.stop(tween);
        self.emitters.iter_mut().for_each(|mut x| x.stop(tween));
        self.holocall.stop(tween);
    }
}

impl Pause for Actors {
    fn pause(&mut self, tween: kira::Tween) {
        self.v.pause(tween);
        self.emitters.pause(tween);
        self.holocall.pause(tween);
    }
}

impl Resume for Actors {
    fn resume(&mut self, tween: kira::Tween) {
        self.v.resume(tween);
        self.emitters.resume(tween);
        self.holocall.resume(tween);
    }
}

impl Clear for Actors {
    fn clear(&mut self) {
        self.v.clear();
        self.emitters.clear();
        self.holocall.clear();
    }
}

impl Reclaim for Actors {
    fn reclaim(&mut self) {
        self.v.reclaim();
        self.emitters.iter_mut().for_each(|mut x| x.reclaim());
        self.holocall.reclaim();
    }
}
