use dashmap::DashMap;
use kira::sound::{
    static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError, PlaybackState,
};
use red4ext_rs::types::{CName, EntityId, Opt, Ref};
use snowflake::ProcessUniqueId;

use crate::{ToTween, Tween};

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
    ) {
        self.statics.insert(
            ProcessUniqueId::new(),
            Handle::new(handle, event_name, emitter),
        );
    }

    pub fn store_stream(
        &mut self,
        handle: StreamingSoundHandle<FromFileError>,
        event_name: CName,
        emitter: Option<Emitter>,
    ) {
        self.streams.insert(
            ProcessUniqueId::new(),
            Handle::new(handle, event_name, emitter),
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

    pub fn stop(&mut self, event_name: CName, emitter: Option<Emitter>, tween: Ref<Tween>) {
        for ref mut ref_multi in self.statics.iter_mut() {
            if ref_multi.value().event_name == event_name && ref_multi.value().emitter == emitter {
                ref_multi
                    .value_mut()
                    .handle
                    .stop(tween.clone().into_tween().unwrap_or_default());
            }
        }
        for ref mut ref_multi in self.streams.iter_mut() {
            if ref_multi.value().event_name == event_name && ref_multi.value().emitter == emitter {
                ref_multi
                    .value_mut()
                    .handle
                    .stop(tween.clone().into_tween().unwrap_or_default());
            }
        }
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
    pub fn new(id: Opt<EntityId>, name: Opt<CName>) -> Option<Self> {
        let id = id.into_option();
        let name = name.into_option();
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
}

impl<T> Handle<T> {
    pub fn new(handle: T, event_name: CName, emitter: Option<Emitter>) -> Self {
        Self {
            handle,
            event_name,
            emitter,
        }
    }
}
