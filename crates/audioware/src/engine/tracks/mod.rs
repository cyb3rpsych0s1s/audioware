use ambience::Ambience;
use car_radio::CarRadio;
use dialogue::Dialogue;
use holocall::Holocall;
use kira::{
    Tween,
    sound::FromFileError,
    {AudioManager, backend::Backend},
};
use music::Music;
use radioport::Radioport;
use red4ext_rs::types::{CName, Cruid, EntityId};
use sfx::Sfx;
pub use spatial::Spatial;
use v::V;

use crate::{
    engine::{
        AffectedByTimeDilation,
        traits::{
            DualHandles,
            clear::Clear,
            dilation::{Comparable, SyncDilationBy},
            pause::Pause,
            reclaim::Reclaim,
            resume::Resume,
            stop::{Stop, StopBy},
        },
    },
    error::Error,
};

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

pub struct TrackEntryOptions {
    pub entity_id: Option<EntityId>,
    pub emitter_name: Option<CName>,
    pub affected_by_time_dilation: bool,
}

impl AffectedByTimeDilation for TrackEntryOptions {
    fn affected_by_time_dilation(&self) -> bool {
        self.affected_by_time_dilation
    }
}

impl Comparable<EntityId> for TrackEntryOptions {
    fn compare(&self, rhs: &EntityId) -> bool {
        self.entity_id.map(|x| x == *rhs).unwrap_or(false)
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
    pub handles: DualHandles<CName, TrackEntryOptions, FromFileError>,
    pub scene_handles: DualHandles<Cruid, (), FromFileError>,
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
        self.handles.pause(tween);
        self.scene_handles.pause(tween);
    }
    pub fn resume(&mut self, tween: Tween) {
        self.handles.resume(tween);
        self.scene_handles.resume(tween);
    }
    pub fn reclaim(&mut self) {
        self.handles.reclaim();
        self.scene_handles.reclaim();
    }
    pub fn any_handle(&self) -> bool {
        self.handles.any_handle() || self.scene_handles.any_handle()
    }
    pub fn stop(&mut self, tween: Tween) {
        self.handles.stop(tween);
        self.scene_handles.stop(tween);
    }
    pub fn stop_by(
        &mut self,
        event_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        tween: Tween,
    ) {
        self.handles
            .stop_by(&(event_name, entity_id, emitter_name), tween);
    }
    pub fn stop_scene_by(&mut self, event_name: Cruid, tween: Tween) {
        self.scene_handles.stop_by(&event_name, tween);
    }
    pub fn sync_dilation(&mut self, entity_id: EntityId, update: DilationUpdate) {
        self.handles.sync_dilation_by(&entity_id, &update);
    }
    pub fn clear(&mut self) {
        self.stop(IMMEDIATELY);
        self.handles.clear();
        self.scene_handles.clear();
    }
}
