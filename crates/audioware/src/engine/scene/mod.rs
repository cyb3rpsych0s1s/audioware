use std::num::NonZero;

use audioware_manifest::PlayerGender;
use dilation::Dilation;
use emitters::{Emitter, Emitters, EMITTERS};
use kira::{
    manager::{backend::Backend, AudioManager},
    spatial::{
        emitter::{EmitterDistances, EmitterSettings},
        listener::ListenerSettings,
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    tween::Tween,
};
use listener::Listener;
use red4ext_rs::types::{CName, EntityId, GameInstance, Ref};

use crate::{
    error::{Error, SceneError},
    get_player, AsEntity, AsScriptedPuppet, AsScriptedPuppetExt, AsTimeDilatable, AvObject,
    BikeObject, CarObject, Device, Entity, GamedataNpcType, ScriptedPuppet, TankObject,
    TimeDilatable, VehicleObject,
};

use super::{lifecycle, tracks::Tracks, tweens::IMMEDIATELY};

mod dilation;
mod emitters;
mod listener;

pub use dilation::{AffectedByTimeDilation, DilationUpdate};
pub use emitters::Store;

/// Audio spatial scene.
pub struct Scene {
    pub emitters: Emitters,
    pub v: Listener,
    pub scene: SpatialSceneHandle,
}

impl Scene {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        tracks: &Tracks,
    ) -> Result<Self, Error> {
        let settings = SpatialSceneSettings::default();
        let capacity = settings.emitter_capacity as usize;
        let mut scene = manager.add_spatial_scene(settings)?;
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
        let handle = scene.add_listener(
            position,
            orientation,
            ListenerSettings::default().track(tracks.sfx.as_ref()),
        )?;
        Ok(Self {
            v: Listener {
                id,
                handle,
                dilation: Dilation::new(dilation),
            },
            scene,
            emitters: Emitters::with_capacity(capacity),
        })
    }

    pub fn add_emitter(
        &mut self,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Option<CName>,
        settings: Option<(EmitterSettings, NonZero<u64>)>,
    ) -> Result<(), Error> {
        if entity_id == self.v.id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let paired = self
            .emitters
            .pair_emitter(entity_id, tag_name, emitter_name, settings)?;
        if !paired {
            let (position, busy, dilation, distances) = Emitter::full_infos(entity_id)?;

            lifecycle!("emitter settings before {:?} [{entity_id}]", settings);
            let mapped = match settings {
                Some((settings, _))
                    if settings.distances.min_distance == 0.
                        && settings.distances.max_distance == 0. =>
                {
                    settings.distances(distances.unwrap_or_default())
                }
                Some((settings, _)) => settings,
                None => EmitterSettings::default().distances(distances.unwrap_or_default()),
            };
            lifecycle!("emitter settings after {:?} [{entity_id}]", mapped);
            let handle = self.scene.add_emitter(position, mapped)?;
            self.emitters.add_emitter(
                entity_id,
                tag_name,
                emitter_name,
                settings,
                handle,
                dilation,
                position,
                busy,
            )?;
        }
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

    pub fn stop_emitters(&mut self, tween: Tween) {
        self.emitters.stop_emitters(tween);
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

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_listener()?;
        self.sync_emitters()?;
        Ok(())
    }

    pub fn is_registered_emitter(entity_id: EntityId, tag_name: Option<CName>) -> bool {
        EMITTERS
            .read()
            .iter()
            .any(|(id, tag)| *id == entity_id && tag_name.map(|x| x == *tag).unwrap_or(true))
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

    pub fn clear(&mut self) {
        self.stop_emitters(IMMEDIATELY);
        self.emitters.clear();
    }

    pub fn pause(&mut self, tween: Tween) {
        self.emitters.pause(tween);
    }

    pub fn resume(&mut self, tween: Tween) {
        self.emitters.resume(tween);
    }

    pub fn reclaim(&mut self) {
        self.emitters.reclaim();
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
        if let Some(mut slots) = self.emitters.get_mut(&entity_id) {
            if slots.value().dilation.last.as_ref() != Some(dilation) {
                slots.value_mut().dilation.last = Some(dilation.clone());
                updated = true;
            }
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
        if let Some(mut slots) = self.emitters.get_mut(&entity_id) {
            if slots.value().dilation.last.as_ref() != Some(dilation) {
                slots.value_mut().dilation.last = Some(dilation.clone());
                updated = true;
            }
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
        EMITTERS.read().len() as i32
    }
}

pub trait AsEntityExt {
    fn get_emitter_distances(&self) -> Option<EmitterDistances>;
    fn get_gender(&self) -> Option<PlayerGender>;
}

impl AsEntityExt for Ref<Entity> {
    fn get_emitter_distances(&self) -> Option<EmitterDistances> {
        if self.is_null() {
            return None;
        }
        if let Some(puppet) = self.clone().cast::<ScriptedPuppet>().as_ref() {
            let s = match puppet.get_npc_type() {
                GamedataNpcType::Device | GamedataNpcType::Drone | GamedataNpcType::Spiderbot => {
                    EmitterDistances {
                        min_distance: 3.,
                        max_distance: 30.,
                    }
                }
                GamedataNpcType::Android | GamedataNpcType::Human => EmitterDistances {
                    min_distance: 5.,
                    max_distance: 65.,
                },
                GamedataNpcType::Cerberus | GamedataNpcType::Chimera | GamedataNpcType::Mech => {
                    EmitterDistances {
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
                return Some(EmitterDistances {
                    min_distance: 8.,
                    max_distance: 100.,
                });
            }
            if vehicle.is_a::<BikeObject>() {
                return Some(EmitterDistances {
                    min_distance: 8.,
                    max_distance: 120.,
                });
            }
            if vehicle.is_a::<TankObject>() || vehicle.is_a::<AvObject>() {
                return Some(EmitterDistances {
                    min_distance: 20.,
                    max_distance: 200.,
                });
            }
        }
        if self.clone().cast::<Device>().as_ref().is_some() {
            return Some(EmitterDistances {
                min_distance: 3.,
                max_distance: 30.,
            });
        }
        None
    }

    fn get_gender(&self) -> Option<PlayerGender> {
        if self.is_a::<ScriptedPuppet>() {
            let puppet = self.clone().cast::<ScriptedPuppet>().unwrap();
            let gender = puppet.get_template_gender();
            return Some(gender);
        }
        None
    }
}

pub trait ToDistances {
    fn to_distances(&self) -> Option<EmitterDistances>;
}

impl ToDistances for EntityId {
    fn to_distances(&self) -> Option<EmitterDistances> {
        use crate::types::AsGameInstance;
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, *self);
        entity.get_emitter_distances()
    }
}
