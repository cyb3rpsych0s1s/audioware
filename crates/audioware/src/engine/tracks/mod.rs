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
use red4ext_rs::types::{CName, EntityId};
use sfx::Sfx;
pub use spatial::Spatial;
use v::V;

use crate::{
    ControlId,
    engine::{
        AffectedByTimeDilation,
        traits::{
            DualHandles,
            clear::Clear,
            dilation::{Comparable, SyncDilationBy},
            panning::SetControlledPanning,
            pause::{Pause, PauseControlled},
            playback::SetControlledPlaybackRate,
            position::PositionControlled,
            reclaim::Reclaim,
            resume::{Resume, ResumeControlled, ResumeControlledAt},
            seek::{SeekControlledBy, SeekControlledTo},
            stop::{Stop, StopBy, StopControlled},
            terminate::Terminate,
            volume::SetControlledVolume,
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
        })
    }
    pub fn pause(&mut self, tween: Tween) {
        self.handles.pause(tween);
    }
    pub fn resume(&mut self, tween: Tween) {
        self.handles.resume(tween);
    }
    pub fn reclaim(&mut self) {
        self.handles.reclaim();
    }
    pub fn any_handle(&self) -> bool {
        self.handles.any_handle()
    }
    pub fn stop(&mut self, tween: Tween) {
        self.handles.stop(tween);
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
    pub fn sync_dilation(&mut self, entity_id: EntityId, update: DilationUpdate) {
        self.handles.sync_dilation_by(&entity_id, &update);
    }
    pub fn clear(&mut self) {
        self.stop(IMMEDIATELY);
        self.handles.clear();
    }
}

impl SetControlledVolume for Tracks {
    fn set_controlled_volume(
        &mut self,
        id: crate::ControlId,
        amplitude: audioware_core::Amplitude,
        tween: Tween,
    ) {
        self.handles.set_controlled_volume(id, amplitude, tween);
    }
}

impl SetControlledPlaybackRate for Tracks {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.handles.set_controlled_playback_rate(id, rate, tween);
    }
}

impl SetControlledPanning for Tracks {
    fn set_controlled_panning(&mut self, id: ControlId, panning: kira::Panning, tween: Tween) {
        self.handles.set_controlled_panning(id, panning, tween);
    }
}

impl PositionControlled for Tracks {
    fn position_controlled(&mut self, id: ControlId, sender: crossbeam::channel::Sender<f32>) {
        self.handles.position_controlled(id, sender);
    }
}

impl StopControlled for Tracks {
    fn stop_controlled(&mut self, id: ControlId, tween: Tween) {
        self.handles.stop_controlled(id, tween);
    }
}

impl PauseControlled for Tracks {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.handles.pause_controlled(id, tween);
    }
}

impl ResumeControlled for Tracks {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.handles.resume_controlled(id, tween);
    }
}

impl ResumeControlledAt for Tracks {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.handles.resume_controlled_at(id, delay, tween);
    }
}

impl SeekControlledTo for Tracks {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.handles.seek_controlled_to(id, position);
    }
}

impl SeekControlledBy for Tracks {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.handles.seek_controlled_by(id, amount);
    }
}

impl Terminate for Tracks {
    fn terminate(&mut self) {
        self.handles.terminate();
    }
}
