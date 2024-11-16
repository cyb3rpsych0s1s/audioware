use dashmap::DashMap;
use kira::{
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    tween::Tween,
};
use red4ext_rs::types::{CName, EntityId};
use snowflake::ProcessUniqueId;

use super::tweens::IMMEDIATELY;

pub struct Handles {
    statics: DashMap<ProcessUniqueId, Handle<StaticSoundHandle>>,
    streams: DashMap<ProcessUniqueId, Handle<StreamingSoundHandle<FromFileError>>>,
}

impl Handles {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            statics: DashMap::with_capacity(capacity),
            streams: DashMap::with_capacity(capacity),
        }
    }

    pub fn store_static(
        &mut self,
        handle: StaticSoundHandle,
        event_name: CName,
        emitter: Option<Emitter>,
        spatial: bool,
    ) {
        self.statics.insert(
            ProcessUniqueId::new(),
            Handle::new(handle, event_name, emitter, spatial),
        );
    }

    pub fn store_stream(
        &mut self,
        handle: StreamingSoundHandle<FromFileError>,
        event_name: CName,
        emitter: Option<Emitter>,
        spatial: bool,
    ) {
        self.streams.insert(
            ProcessUniqueId::new(),
            Handle::new(handle, event_name, emitter, spatial),
        );
    }

    pub fn reclaim(&mut self) {
        self.statics
            .retain(|_, v| v.handle.state() != PlaybackState::Stopped);
        self.streams
            .retain(|_, v| v.handle.state() != PlaybackState::Stopped);
    }

    pub fn pause(&mut self) {
        for ref mut ref_multi in self.statics.iter_mut() {
            if ref_multi.value_mut().handle.state() == PlaybackState::Playing {
                ref_multi.value_mut().handle.pause(Default::default());
            }
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            if ref_multi.value_mut().handle.state() == PlaybackState::Playing {
                ref_multi.value_mut().handle.pause(Default::default());
            }
        }
    }

    pub fn resume(&mut self) {
        for ref mut ref_multi in self.statics.iter_mut() {
            if ref_multi.value_mut().handle.state() == PlaybackState::Paused
                || ref_multi.value_mut().handle.state() == PlaybackState::Pausing
            {
                ref_multi.value_mut().handle.resume(Default::default());
            }
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            if ref_multi.value_mut().handle.state() == PlaybackState::Paused
                || ref_multi.value_mut().handle.state() == PlaybackState::Pausing
            {
                ref_multi.value_mut().handle.resume(Default::default());
            }
        }
    }

    pub fn stop_all(&mut self, tween: Tween) {
        for ref mut ref_multi in self.statics.iter_mut() {
            ref_multi.value_mut().handle.stop(tween);
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            ref_multi.value_mut().handle.stop(tween);
        }
    }

    pub fn stop_by(&mut self, event_name: CName, emitter: Option<Emitter>, tween: Tween) {
        for ref mut ref_multi in self.statics.iter_mut() {
            if ref_multi.value().event_name == event_name && ref_multi.value().emitter == emitter {
                ref_multi.value_mut().handle.stop(tween);
            }
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            if ref_multi.value().event_name == event_name && ref_multi.value().emitter == emitter {
                ref_multi.value_mut().handle.stop(tween);
            }
        }
    }

    pub fn stop_emitters(&mut self, tween: Tween) {
        for ref mut ref_multi in self.statics.iter_mut() {
            if ref_multi.value().emitter.is_some() && ref_multi.value().spatial {
                ref_multi.value_mut().handle.stop(tween);
            }
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            if ref_multi.value().emitter.is_some() && ref_multi.value().spatial {
                ref_multi.value_mut().handle.stop(tween);
            }
        }
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        self.statics.retain(|_, v| {
            if v.emitter
                .as_ref()
                .map(|x| x.id != entity_id)
                .unwrap_or(true)
            {
                v.handle.stop(IMMEDIATELY);
                return true;
            }
            false
        });
        self.streams.retain(|_, v| {
            if v.emitter
                .as_ref()
                .map(|x| x.id != entity_id)
                .unwrap_or(true)
            {
                v.handle.stop(IMMEDIATELY);
                return true;
            }
            false
        });
    }

    pub fn stop_for(&mut self, entity_id: EntityId, tween: Tween) {
        for ref mut ref_multi in self.statics.iter_mut() {
            if ref_multi
                .value()
                .emitter
                .as_ref()
                .map(|x| x.id == entity_id)
                .unwrap_or(false)
            {
                ref_multi.value_mut().handle.stop(tween);
            }
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            if ref_multi
                .value()
                .emitter
                .as_ref()
                .map(|x| x.id == entity_id)
                .unwrap_or(false)
            {
                ref_multi.value_mut().handle.stop(tween);
            }
        }
    }

    pub fn clear(&mut self) {
        self.statics.clear();
        self.streams.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.statics.is_empty() && self.streams.is_empty()
    }
}

#[derive(PartialEq)]
pub struct Emitter {
    pub id: EntityId,
    pub name: CName,
}

impl Emitter {
    pub fn new(id: Option<EntityId>, name: Option<CName>) -> Option<Self> {
        match (id, name) {
            (Some(id), Some(name)) => Some(Emitter { id, name }),
            _ => None,
        }
    }
}

pub struct Handle<T> {
    pub handle: T,
    pub event_name: CName,
    pub emitter: Option<Emitter>,
    pub spatial: bool,
}

impl<T> Handle<T> {
    pub fn new(handle: T, event_name: CName, emitter: Option<Emitter>, spatial: bool) -> Self {
        Self {
            handle,
            event_name,
            emitter,
            spatial,
        }
    }
}
