use ambience::Ambience;
use car_radio::CarRadio;
use dialogue::Dialogue;
use holocall::Holocall;
use kira::{
    Tween,
    sound::{
        FromFileError, PlaybackState, static_sound::StaticSoundHandle,
        streaming::StreamingSoundHandle,
    },
    {AudioManager, backend::Backend},
};
use music::Music;
use radioport::Radioport;
use red4ext_rs::types::{CName, Cruid, EntityId};
use sfx::Sfx;
pub use spatial::Spatial;
use v::V;

use crate::error::Error;

use super::{DilationUpdate, modulators::Modulators, tweens::IMMEDIATELY};

pub mod ambience;
mod car_radio;
mod dialogue;
mod holocall;
mod music;
mod radioport;
mod sfx;
mod spatial;
mod v;

pub struct Handle<K, T> {
    pub event_name: K,
    pub entity_id: Option<EntityId>,
    pub emitter_name: Option<CName>,
    pub handle: T,
    pub affected_by_time_dilation: bool,
}

#[derive(Default)]
pub struct Handles<K> {
    pub statics: Vec<Handle<K, StaticSoundHandle>>,
    pub streams: Vec<Handle<K, StreamingSoundHandle<FromFileError>>>,
}

impl<K> Drop for Handles<K> {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.iter_mut().for_each(|x| {
            x.handle.stop(IMMEDIATELY);
        });
    }
}

pub struct Tracks {
    // vanilla tracks
    pub sfx: Sfx,
    pub radioport: Radioport,
    pub music: Music,
    pub dialogue: Dialogue,
    pub car_radio: CarRadio,
    // audioware tracks
    pub v: V,
    pub holocall: Holocall,
    // tracks affected by reverb mix + preset (e.g. underwater)
    pub ambience: Ambience,
    pub handles: Handles<CName>,
    pub scene_handles: Handles<Cruid>,
}

impl Tracks {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        modulators: &Modulators,
    ) -> Result<Self, Error> {
        let ambience = Ambience::try_new(manager, modulators)?;
        let v = V::try_new(manager, &ambience)?;
        let holocall = Holocall::try_new(manager, &ambience)?;
        let sfx = Sfx::try_new(manager, &ambience)?;
        let radioport = Radioport::try_new(manager, &ambience)?;
        let music = Music::try_new(manager, &ambience)?;
        let dialogue = Dialogue::try_new(manager, &ambience)?;
        let car_radio = CarRadio::try_new(manager, &ambience)?;
        Ok(Self {
            ambience,
            v,
            holocall,
            sfx,
            radioport,
            music,
            dialogue,
            car_radio,
            handles: Default::default(),
            scene_handles: Default::default(),
        })
    }
    pub fn pause(&mut self, tween: Tween) {
        self.handles.statics.iter_mut().for_each(|x| {
            x.handle.pause(tween);
        });
        self.handles.streams.iter_mut().for_each(|x| {
            x.handle.pause(tween);
        });
        self.scene_handles.statics.iter_mut().for_each(|x| {
            x.handle.pause(tween);
        });
        self.scene_handles.streams.iter_mut().for_each(|x| {
            x.handle.pause(tween);
        });
    }
    pub fn resume(&mut self, tween: Tween) {
        self.handles.statics.iter_mut().for_each(|x| {
            x.handle.resume(tween);
        });
        self.handles.streams.iter_mut().for_each(|x| {
            x.handle.resume(tween);
        });
        self.scene_handles.statics.iter_mut().for_each(|x| {
            x.handle.resume(tween);
        });
        self.scene_handles.streams.iter_mut().for_each(|x| {
            x.handle.resume(tween);
        });
    }
    pub fn reclaim(&mut self) {
        self.handles
            .statics
            .retain(|x| x.handle.state() != PlaybackState::Stopped);
        self.handles
            .streams
            .retain(|x| x.handle.state() != PlaybackState::Stopped);
        self.scene_handles
            .statics
            .retain(|x| x.handle.state() != PlaybackState::Stopped);
        self.scene_handles
            .streams
            .retain(|x| x.handle.state() != PlaybackState::Stopped);
    }
    pub fn any_handle(&self) -> bool {
        !self.handles.statics.is_empty()
            || !self.handles.streams.is_empty()
            || !self.scene_handles.statics.is_empty()
            || !self.scene_handles.streams.is_empty()
    }
    pub fn stop(&mut self, tween: Tween) {
        self.handles.statics.iter_mut().for_each(|x| {
            x.handle.stop(tween);
        });
        self.handles.streams.iter_mut().for_each(|x| {
            x.handle.stop(tween);
        });
        self.scene_handles.statics.iter_mut().for_each(|x| {
            x.handle.stop(tween);
        });
        self.scene_handles.streams.iter_mut().for_each(|x| {
            x.handle.stop(tween);
        });
    }
    pub fn stop_by(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        tween: Tween,
    ) {
        self.handles
            .statics
            .iter_mut()
            .filter(|x| {
                x.event_name == event_name
                    && x.entity_id == entity_id
                    && x.emitter_name == emitter_name
            })
            .for_each(|x| {
                x.handle.stop(tween);
            });
        self.handles
            .streams
            .iter_mut()
            .filter(|x| {
                x.event_name == event_name
                    && x.entity_id == entity_id
                    && x.emitter_name == emitter_name
            })
            .for_each(|x| {
                x.handle.stop(tween);
            });
    }
    pub fn stop_scene_by(&mut self, event_name: Cruid, tween: Tween) {
        self.scene_handles
            .statics
            .iter_mut()
            .filter(|x| x.event_name == event_name)
            .for_each(|x| {
                x.handle.stop(tween);
            });
        self.scene_handles
            .streams
            .iter_mut()
            .filter(|x| x.event_name == event_name)
            .for_each(|x| {
                x.handle.stop(tween);
            });
    }
    pub fn sync_dilation(&mut self, entity_id: EntityId, update: DilationUpdate) {
        self.handles
            .statics
            .iter_mut()
            .filter(|x| x.entity_id == Some(entity_id) && x.affected_by_time_dilation)
            .for_each(|x| {
                x.handle
                    .set_playback_rate(update.dilation(), update.tween_curve());
            });
        self.handles
            .streams
            .iter_mut()
            .filter(|x| x.entity_id == Some(entity_id) && x.affected_by_time_dilation)
            .for_each(|x| {
                x.handle
                    .set_playback_rate(update.dilation(), update.tween_curve());
            });
    }
    pub fn clear(&mut self) {
        self.stop(IMMEDIATELY);
        self.handles.statics.clear();
        self.handles.streams.clear();
        self.scene_handles.statics.clear();
        self.scene_handles.streams.clear();
    }
}
