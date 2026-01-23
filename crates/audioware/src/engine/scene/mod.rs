use std::num::NonZero;

use audioware_core::SpatialTrackSettings;
use audioware_manifest::PlayerGender;
use dilation::Dilation;
use emitters::{Emitter, Emitters};
use kira::{AudioManager, Easing, Tween, backend::Backend, track::SpatialTrackDistances};
use listener::Listener;
use red4ext_rs::types::{CName, EntityId, GameInstance, Ref};

use crate::{
    AsEntity, AsScriptedPuppet, AsTimeDilatable, AvObject, BikeObject, CarObject, ControlId,
    Device, Entity, GamedataNpcType, ScriptedPuppet, TankObject, TimeDilatable, VehicleObject,
    engine::{
        scene::actors::{Actors, slot::ActorSlot},
        tracks::Spatial,
        traits::{
            clear::Clear,
            pause::{Pause, PauseControlled},
            playback::SetControlledPlaybackRate,
            position::PositionControlled,
            reclaim::Reclaim,
            resume::{Resume, ResumeControlled, ResumeControlledAt},
            seek::{SeekControlledBy, SeekControlledTo},
            stop::{Stop, StopControlled},
            terminate::Terminate,
            volume::SetControlledVolume,
        },
    },
    error::{Error, SceneError},
    get_player, resolve_any_entity,
};

use super::{lifecycle, tracks::ambience::Ambience, tweens::IMMEDIATELY};

mod actors;
mod dilation;
mod emitters;
mod listener;

pub use dilation::{AffectedByTimeDilation, DilationUpdate};

/// Audio spatial scene.
pub struct Scene {
    pub emitters: Emitters,
    pub actors: Actors,
    pub v: Listener,
}

impl Scene {
    pub fn try_new<B: Backend>(manager: &mut AudioManager<B>) -> Result<Self, Error> {
        const RESERVED_FOR_ACTORS: usize = 8;
        let capacity = manager.sub_track_capacity();
        let (id, position, orientation, dilation) = {
            let v = get_player(GameInstance::new()).cast::<Entity>().unwrap();
            let d = get_player(GameInstance::new())
                .cast::<TimeDilatable>()
                .unwrap();
            (
                v.get_entity_id(),
                v.get_world_position(),
                v.get_world_orientation(),
                d.get_time_dilation_value(),
            )
        };
        let handle = manager.add_listener(position, orientation)?;
        Ok(Self {
            v: Listener {
                id,
                handle,
                dilation: Dilation::new(dilation),
            },
            emitters: Emitters::with_capacity(capacity - RESERVED_FOR_ACTORS),
            actors: Actors::with_capacity(RESERVED_FOR_ACTORS),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_emitter<B: Backend>(
        &mut self,
        manager: &mut AudioManager<B>,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Option<CName>,
        settings: Option<&(SpatialTrackSettings, NonZero<u64>)>,
        ambience: &Ambience,
    ) -> Result<(), Error> {
        if entity_id == self.v.id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let (position, busy, dilation, distances) = Emitter::full_infos(entity_id)?;

        lifecycle!("emitter settings before {:?} [{entity_id}]", settings);
        let mapped = match settings {
            Some((settings, _))
                if settings.distances.min_distance == 0.
                    && settings.distances.max_distance == 0. =>
            {
                let mut settings = settings.clone();
                settings.distances = distances.unwrap_or_default();
                settings
            }
            Some((settings, _)) => settings.clone(),
            None => SpatialTrackSettings {
                distances: distances.unwrap_or_default(),
                ..Default::default()
            },
        };
        lifecycle!("emitter settings after {:?} [{entity_id}]", mapped);
        let handle = Spatial::try_new(manager, self.v.handle.id(), position, mapped, ambience)?;
        self.emitters.add_emitter(
            handle,
            entity_id,
            tag_name,
            emitter_name,
            dilation,
            Some(0.),
            position,
            busy,
            settings
                .map(|x| x.0.persist_until_sounds_finish)
                .unwrap_or(false),
        )
    }

    pub fn exists_actor(&self, entity_id: &EntityId) -> bool {
        self.actors.exists(entity_id)
    }

    pub fn add_actor<B: Backend>(
        &mut self,
        manager: &mut AudioManager<B>,
        entity_id: EntityId,
        ambience: &Ambience,
    ) -> Result<(), Error> {
        if self.actors.exists(&entity_id) {
            return Ok(());
        }
        if entity_id == self.v.id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let (position, distances) = Emitter::actor_infos(entity_id)?;
        let settings = SpatialTrackSettings {
            distances: distances.unwrap_or_default(),
            affected_by_reverb_mix: true,
            affected_by_environmental_preset: true,
            attenuation_function: Some(Easing::Linear),
            ..Default::default()
        };
        let handle = Spatial::try_new(
            manager,
            self.v.handle.id(),
            position,
            settings.clone(),
            ambience,
        )?;
        let slot = ActorSlot::new(handle, position);
        lifecycle!(
            "slot about to be inserted: {slot} with settings {:?}",
            settings
        );
        self.actors.emitters.insert(entity_id, slot);
        Ok(())
    }

    pub fn stop_on_emitter(
        &mut self,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        tween: Tween,
    ) {
        self.emitters
            .stop_on_emitter(event_name, entity_id, tag_name, tween);
    }

    pub fn stop_emitters_and_actors(&mut self, tween: Tween) {
        self.emitters.stop(tween);
        self.actors.stop(tween);
    }

    fn sync_listener(&mut self) -> Result<(), Error> {
        let player = get_player(GameInstance::new());
        if player.is_null() {
            return Ok(());
        }
        let (position, orientation) = {
            let entity = player.cast::<Entity>().unwrap();
            (entity.get_world_position(), entity.get_world_orientation())
        };
        self.v.handle.set_position(position, IMMEDIATELY);
        self.v.handle.set_orientation(orientation, IMMEDIATELY);
        Ok(())
    }

    fn sync_emitters(&mut self) -> Result<(), Error> {
        self.emitters.sync_emitters()?;
        Ok(())
    }

    fn sync_actors(&mut self) -> Result<(), Error> {
        self.actors.sync_emitters()?;
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_listener()?;
        self.sync_emitters()?;
        self.sync_actors()?;
        Ok(())
    }

    pub fn is_registered_emitter(entity_id: EntityId, tag_name: Option<CName>) -> bool {
        Emitters::is_registered_emitter(entity_id, tag_name)
    }

    pub fn unregister_emitter(&mut self, entity_id: &EntityId, tag_name: &CName) -> bool {
        self.emitters.unregister_emitter(entity_id, tag_name)
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        self.emitters.on_emitter_dies(&entity_id);
    }

    pub fn on_emitter_incapacitated(&mut self, entity_id: EntityId, tween: Tween) {
        self.emitters.on_emitter_incapacitated(entity_id, tween);
    }

    pub fn any_emitter(&self) -> bool {
        !self.emitters.is_empty()
    }

    pub fn any_actor(&self) -> bool {
        !self.actors.is_empty()
    }

    pub fn clear(&mut self) {
        self.stop_emitters_and_actors(IMMEDIATELY);
        self.emitters.clear();
        self.actors.clear();
    }

    pub fn pause(&mut self, tween: Tween) {
        self.emitters.pause(tween);
        self.actors.pause(tween);
    }

    pub fn resume(&mut self, tween: Tween) {
        self.emitters.resume(tween);
        self.actors.resume(tween);
    }

    pub fn reclaim(&mut self) {
        self.emitters.reclaim();
        self.actors.reclaim();
    }

    pub fn set_listener_dilation(&mut self, dilation: &DilationUpdate) -> bool {
        if self.v.dilation.last.as_ref() != Some(dilation) {
            self.v.dilation.last = Some(dilation.clone());
            self.sync_dilation();
            return true;
        }
        false
    }

    pub fn unset_listener_dilation(&mut self, dilation: &DilationUpdate) -> bool {
        if self.v.dilation.last.as_ref() != Some(dilation) {
            self.v.dilation.last = Some(dilation.clone());
            self.sync_dilation();
            return true;
        }
        false
    }

    pub fn set_emitter_dilation(&mut self, entity_id: EntityId, dilation: &DilationUpdate) -> bool {
        let mut updated = false;
        if let Some(mut slots) = self.emitters.get_mut(&entity_id)
            && slots.value().dilation.last.as_ref() != Some(dilation)
        {
            slots.value_mut().dilation.last = Some(dilation.clone());
            updated = true;
        }
        if updated {
            self.sync_dilation();
        }
        updated
    }

    pub fn unset_emitter_dilation(
        &mut self,
        entity_id: EntityId,
        dilation: &DilationUpdate,
    ) -> bool {
        let mut updated = false;
        if let Some(mut slots) = self.emitters.get_mut(&entity_id)
            && slots.value().dilation.last.as_ref() != Some(dilation)
        {
            slots.value_mut().dilation.last = Some(dilation.clone());
            updated = true;
        }
        if updated {
            self.sync_dilation();
        }
        updated
    }

    fn sync_dilation(&mut self) {
        let listener = self.v.dilation.dilation(); // e.g. 0.7
        let mut tween = self.v.dilation.tween();
        for ref mut slots in self.emitters.iter_mut() {
            let rate: f64 = 1. - (1. - listener)
                + (
                    // e.g. 5 or 7
                    slots
                        .dilation
                        .last
                        .as_ref()
                        .filter(|x| x.dilation() != 1.)
                        .map(|x| x.dilation() / 10.)
                        .unwrap_or(0.)
                );
            if tween.is_none() {
                tween = slots.dilation.tween();
            }
            lifecycle!("sync emitter handle dilation: {rate} {tween:?}");
            slots.sync_dilation(rate, tween.unwrap_or(IMMEDIATELY));
        }
    }

    pub fn listener_id(&self) -> EntityId {
        self.v.id
    }

    pub fn emitters_count() -> i32 {
        Emitters::emitters_count()
    }

    pub fn update_pending_occlusions(&mut self, entity_id: EntityId, factor: f32) {
        self.emitters.update_pending_occlusions(entity_id, factor);
    }
}

pub trait AsEntityExt {
    fn get_emitter_distances(&self) -> Option<SpatialTrackDistances>;
    fn get_gender(&self) -> Option<PlayerGender>;
}

impl AsEntityExt for Ref<Entity> {
    fn get_emitter_distances(&self) -> Option<SpatialTrackDistances> {
        if self.is_null() {
            return None;
        }
        if let Some(puppet) = self.clone().cast::<ScriptedPuppet>().as_ref() {
            let s = match puppet.get_npc_type() {
                GamedataNpcType::Device | GamedataNpcType::Drone | GamedataNpcType::Spiderbot => {
                    SpatialTrackDistances {
                        min_distance: 3.,
                        max_distance: 30.,
                    }
                }
                GamedataNpcType::Android | GamedataNpcType::Human => SpatialTrackDistances {
                    min_distance: 5.,
                    max_distance: 65.,
                },
                GamedataNpcType::Cerberus | GamedataNpcType::Chimera | GamedataNpcType::Mech => {
                    SpatialTrackDistances {
                        min_distance: 10.,
                        max_distance: 130.,
                    }
                }
                GamedataNpcType::Invalid | GamedataNpcType::Count | GamedataNpcType::Any => {
                    return None;
                }
            };
            return Some(s);
        }
        if let Some(vehicle) = self.clone().cast::<VehicleObject>().as_ref() {
            if vehicle.is_a::<CarObject>() {
                return Some(SpatialTrackDistances {
                    min_distance: 8.,
                    max_distance: 100.,
                });
            }
            if vehicle.is_a::<BikeObject>() {
                return Some(SpatialTrackDistances {
                    min_distance: 8.,
                    max_distance: 120.,
                });
            }
            if vehicle.is_a::<TankObject>() || vehicle.is_a::<AvObject>() {
                return Some(SpatialTrackDistances {
                    min_distance: 20.,
                    max_distance: 200.,
                });
            }
        }
        if self.clone().cast::<Device>().as_ref().is_some() {
            return Some(SpatialTrackDistances {
                min_distance: 3.,
                max_distance: 30.,
            });
        }
        None
    }

    fn get_gender(&self) -> Option<PlayerGender> {
        unsafe { self.fields() }.and_then(|x| {
            x.visual_tags.tags.iter().find_map(|x| match x {
                t if *t == CName::new("Female") => Some(PlayerGender::Female),
                t if *t == CName::new("Male") => Some(PlayerGender::Male),
                _ => None,
            })
        })
    }
}

pub trait ToDistances {
    fn to_distances(&self) -> Option<SpatialTrackDistances>;
}

impl ToDistances for EntityId {
    fn to_distances(&self) -> Option<SpatialTrackDistances> {
        let entity = resolve_any_entity(*self);
        entity.get_emitter_distances()
    }
}

impl SetControlledVolume for Scene {
    fn set_controlled_volume(
        &mut self,
        id: crate::ControlId,
        amplitude: audioware_core::Amplitude,
        tween: Tween,
    ) {
        self.emitters.set_controlled_volume(id, amplitude, tween);
    }
}

impl SetControlledPlaybackRate for Scene {
    fn set_controlled_playback_rate(&mut self, id: ControlId, rate: f64, tween: Tween) {
        self.emitters.set_controlled_playback_rate(id, rate, tween);
    }
}

impl PositionControlled for Scene {
    fn position_controlled(&mut self, id: ControlId, sender: crossbeam::channel::Sender<f32>) {
        self.emitters.position_controlled(id, sender);
    }
}

impl StopControlled for Scene {
    fn stop_controlled(&mut self, id: ControlId, tween: Tween) {
        self.emitters.stop_controlled(id, tween);
    }
}

impl PauseControlled for Scene {
    fn pause_controlled(&mut self, id: ControlId, tween: Tween) {
        self.emitters.pause_controlled(id, tween);
    }
}

impl ResumeControlled for Scene {
    fn resume_controlled(&mut self, id: ControlId, tween: Tween) {
        self.emitters.resume_controlled(id, tween);
    }
}

impl ResumeControlledAt for Scene {
    fn resume_controlled_at(&mut self, id: ControlId, delay: f64, tween: Tween) {
        self.emitters.resume_controlled_at(id, delay, tween);
    }
}

impl SeekControlledTo for Scene {
    fn seek_controlled_to(&mut self, id: ControlId, position: f64) {
        self.emitters.seek_controlled_to(id, position);
    }
}

impl SeekControlledBy for Scene {
    fn seek_controlled_by(&mut self, id: ControlId, amount: f64) {
        self.emitters.seek_controlled_by(id, amount);
    }
}

impl Terminate for Scene {
    fn terminate(&mut self) {
        self.actors.terminate();
        self.emitters.terminate();
    }
}
