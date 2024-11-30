use std::{collections::HashSet, sync::LazyLock};

use audioware_manifest::PlayerGender;
use dashmap::{iter::IterMut, mapref::one::RefMut, DashMap, DashSet};
use kira::{
    manager::{backend::Backend, AudioManager},
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    spatial::{
        emitter::{EmitterDistances, EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    tween::Tween,
};
use parking_lot::RwLock;
use red4ext_rs::types::{CName, EntityId, GameInstance, Ref};

use crate::{
    error::{Error, SceneError},
    get_player, AIActionHelper, AsEntity, AsGameInstance, AsScriptedPuppet, AsScriptedPuppetExt,
    AsTimeDilatable, AvObject, BikeObject, CarObject, Device, Entity, GameObject, GamedataNpcType,
    ScriptedPuppet, TankObject, TimeDilatable, Vector4, VehicleObject,
};

use super::{lifecycle, tracks::Tracks, tweens::IMMEDIATELY, DilationUpdate};

#[derive(Debug)]
pub struct Listener {
    id: EntityId,
    handle: ListenerHandle,
    dilation: Dilation,
}

#[derive(Debug)]
pub struct Emitter {
    handles: Handles,
    handle: EmitterHandle,
    names: DashSet<Option<CName>>,
    dilation: Dilation,
    last_known_position: Vector4,
    busy: bool,
    persist_until_sounds_finishes: bool,
    marked_for_death: bool,
    #[allow(dead_code, reason = "todo")]
    gender: Option<PlayerGender>,
}

impl Emitter {
    pub fn store_static(
        &mut self,
        event_name: CName,
        handle: StaticSoundHandle,
        affected_by_time_dilation: bool,
    ) {
        self.handles.statics.push(Handle {
            event_name,
            handle,
            affected_by_time_dilation,
        });
    }
    pub fn store_stream(
        &mut self,
        event_name: CName,
        handle: StreamingSoundHandle<FromFileError>,
        affected_by_time_dilation: bool,
    ) {
        self.handles.streams.push(Handle {
            event_name,
            handle,
            affected_by_time_dilation,
        });
    }
    pub fn any_playing_handle(&self) -> bool {
        self.handles
            .statics
            .iter()
            .any(|x| x.handle.state() == PlaybackState::Playing)
            || self
                .handles
                .streams
                .iter()
                .any(|x| x.handle.state() == PlaybackState::Playing)
    }
}

impl AsRef<EmitterHandle> for Emitter {
    fn as_ref(&self) -> &EmitterHandle {
        &self.handle
    }
}

#[derive(Debug)]
pub struct Handle<T> {
    event_name: CName,
    handle: T,
    affected_by_time_dilation: bool,
}

#[derive(Debug, Default)]
pub struct Handles {
    pub statics: Vec<Handle<StaticSoundHandle>>,
    pub streams: Vec<Handle<StreamingSoundHandle<FromFileError>>>,
}

impl Drop for Handles {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.iter_mut().for_each(|x| {
            x.handle.stop(IMMEDIATELY);
        });
    }
}

#[allow(clippy::type_complexity)]
static EMITTERS: LazyLock<RwLock<HashSet<(EntityId, Option<CName>)>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

/// Audio spatial scene.
pub struct Scene {
    pub emitters: Emitters,
    pub v: Listener,
    pub scene: SpatialSceneHandle,
}

pub struct Emitters(DashMap<EntityId, Emitter>);

impl Emitters {
    fn with_capacity(capacity: usize) -> Self {
        *EMITTERS.write() = HashSet::with_capacity(capacity);
        Self(DashMap::with_capacity(capacity))
    }
    pub fn exists(&self, entity_id: &EntityId) -> bool {
        self.0.contains_key(entity_id)
    }
    fn get_mut(&mut self, entity_id: &EntityId) -> Option<RefMut<'_, EntityId, Emitter>> {
        self.0.get_mut(entity_id)
    }
    pub fn get_mut_by_name(
        &mut self,
        entity_id: &EntityId,
        emitter_name: &Option<CName>,
    ) -> Option<RefMut<'_, EntityId, Emitter>> {
        if let Some(emitter) = self.0.get_mut(entity_id) {
            if emitter.names.contains(emitter_name) {
                return Some(emitter);
            }
        }
        None
    }
    fn insert(
        &mut self,
        entity_id: EntityId,
        emitter_name: Option<CName>,
        value: Emitter,
    ) -> Option<Emitter> {
        let inserted = self.0.insert(entity_id, value);
        if inserted.is_none() {
            EMITTERS.write().insert((entity_id, emitter_name));
        }
        inserted
    }
    fn remove(&mut self, entity_id: EntityId) -> bool {
        let mut removed = false;
        self.0.retain(|k, _| {
            if *k == entity_id {
                removed = true;
                false
            } else {
                true
            }
        });
        if !removed {
            return false;
        }
        EMITTERS.write().retain(|(id, _)| *id != entity_id);
        true
    }
    fn iter_mut(&mut self) -> IterMut<EntityId, Emitter> {
        self.0.iter_mut()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&EntityId, &mut Emitter) -> bool,
    {
        self.0.retain(f)
    }
    fn clear(&mut self) {
        EMITTERS.write().clear();
        self.0.clear();
    }
}

impl Drop for Emitters {
    fn drop(&mut self) {
        EMITTERS.write().clear();
    }
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
        emitter_name: Option<CName>,
        settings: Option<EmitterSettings>,
    ) -> Result<(), Error> {
        if entity_id == self.v.id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        if let Some(emitter) = self.emitters.get_mut(&entity_id) {
            emitter.names.insert(emitter_name);
            return Ok(());
        }
        let (gender, position, busy, dilation, distances) = Emitter::full_infos(entity_id)?;
        lifecycle!("emitter settings before {:?} [{entity_id}]", settings);
        let settings = match settings {
            Some(settings)
                if settings.distances.min_distance == 0.
                    && settings.distances.max_distance == 0. =>
            {
                settings.distances(distances.unwrap_or_default())
            }
            Some(settings) => settings,
            None => EmitterSettings::default().distances(distances.unwrap_or_default()),
        };
        let emitter = self.scene.add_emitter(position, settings)?;
        let names = DashSet::with_capacity(3);
        names.insert(emitter_name);
        lifecycle!("emitter settings after {:?} [{entity_id}]", settings);
        let handle = Emitter {
            handle: emitter,
            handles: Default::default(),
            gender,
            last_known_position: position,
            busy,
            names,
            dilation: Dilation::new(dilation.unwrap_or(1.)),
            persist_until_sounds_finishes: settings.persist_until_sounds_finish,
            marked_for_death: false,
        };
        self.emitters.insert(entity_id, emitter_name, handle);
        lifecycle!(
            "added emitter {entity_id} with name {}",
            emitter_name.map(|x| x.as_str()).unwrap_or("None")
        );
        Ok(())
    }

    pub fn remove_emitter(&mut self, entity_id: EntityId) -> bool {
        if self.emitters.remove(entity_id) {
            lifecycle!("removed emitter {entity_id}");
            return true;
        }
        false
    }

    pub fn stop_on_emitter(
        &mut self,
        event_name: CName,
        entity_id: EntityId,
        emitter_name: Option<CName>,
        tween: Tween,
    ) {
        self.emitters
            .iter_mut()
            .filter(|x| *x.key() == entity_id && x.value().names.contains(&emitter_name))
            .for_each(|mut x| {
                x.value_mut()
                    .handles
                    .statics
                    .iter_mut()
                    .filter(|x| x.event_name == event_name)
                    .for_each(|x| {
                        x.handle.stop(tween);
                    });
                x.value_mut()
                    .handles
                    .streams
                    .iter_mut()
                    .filter(|x| x.event_name == event_name)
                    .for_each(|x| {
                        x.handle.stop(tween);
                    });
            });
    }

    pub fn stop_emitters(&mut self, tween: Tween) {
        self.emitters.iter_mut().for_each(|mut x| {
            x.value_mut().handles.statics.iter_mut().for_each(|x| {
                x.handle.stop(tween);
            });
            x.value_mut().handles.streams.iter_mut().for_each(|x| {
                x.handle.stop(tween);
            });
        });
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
        if self.emitters.is_empty() {
            return Ok(());
        }
        self.emitters.retain(|k, v| {
            if v.marked_for_death && !v.any_playing_handle() {
                EMITTERS.write().retain(|(id, _)| id != k);
                return false;
            }
            let Ok((position, busy)) = Emitter::infos(*k) else {
                EMITTERS.write().retain(|(id, _)| id != k);
                return false;
            };
            v.busy = busy;
            v.last_known_position = position;
            // weirdly enough if emitter is not updated, sound(s) won't update as expected.
            // e.g. when listener moves but emitter stands still.
            v.handle.set_position(position, IMMEDIATELY);
            true
        });
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_listener()?;
        self.sync_emitters()?;
        Ok(())
    }

    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        EMITTERS.read().iter().any(|(id, _)| *id == entity_id)
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        self.emitters.retain(|k, v| {
            if *k == entity_id {
                if !v.persist_until_sounds_finishes {
                    v.handles
                        .statics
                        .iter_mut()
                        .for_each(|x| x.handle.stop(IMMEDIATELY));
                    v.handles
                        .streams
                        .iter_mut()
                        .for_each(|x| x.handle.stop(IMMEDIATELY));
                    EMITTERS.write().retain(|(id, _)| id != k);
                    false
                } else {
                    v.marked_for_death = true;
                    true
                }
            } else {
                true
            }
        });
    }

    pub fn on_emitter_incapacitated(&mut self, entity_id: EntityId, tween: Tween) {
        self.emitters
            .iter_mut()
            .filter(|x| *x.key() == entity_id && !x.value().persist_until_sounds_finishes)
            .for_each(|mut x| {
                x.value_mut()
                    .handles
                    .statics
                    .iter_mut()
                    .for_each(|x| x.handle.stop(tween));
                x.value_mut()
                    .handles
                    .streams
                    .iter_mut()
                    .for_each(|x| x.handle.stop(tween));
            });
    }

    pub fn any_emitter(&self) -> bool {
        !self.emitters.is_empty()
    }

    pub fn clear(&mut self) {
        self.stop_emitters(IMMEDIATELY);
        self.emitters.clear();
    }

    pub fn pause(&mut self, tween: Tween) {
        self.emitters.iter_mut().for_each(|mut x| {
            x.value_mut()
                .handles
                .statics
                .iter_mut()
                .for_each(|x| x.handle.pause(tween));
            x.value_mut()
                .handles
                .streams
                .iter_mut()
                .for_each(|x| x.handle.pause(tween));
        });
    }

    pub fn resume(&mut self, tween: Tween) {
        self.emitters.iter_mut().for_each(|mut x| {
            x.value_mut()
                .handles
                .statics
                .iter_mut()
                .for_each(|x| x.handle.resume(tween));
            x.value_mut()
                .handles
                .streams
                .iter_mut()
                .for_each(|x| x.handle.resume(tween));
        });
    }

    pub fn reclaim(&mut self) {
        self.emitters.iter_mut().for_each(|mut x| {
            x.value_mut()
                .handles
                .statics
                .retain(|x| x.handle.state() != PlaybackState::Stopped);
            x.value_mut()
                .handles
                .streams
                .retain(|x| x.handle.state() != PlaybackState::Stopped);
        });
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
        if let Some(mut emitter) = self.emitters.get_mut(&entity_id) {
            if emitter.dilation.last.as_ref() != Some(dilation) {
                emitter.dilation.last = Some(dilation.clone());
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
        if let Some(mut emitter) = self.emitters.get_mut(&entity_id) {
            if emitter.dilation.last.as_ref() != Some(dilation) {
                emitter.dilation.last = Some(dilation.clone());
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
        let mut rate: f64 = 1.;
        self.emitters.iter_mut().for_each(|mut x| {
            rate = 1. - (1. - listener)
                + (
                    // e.g. 5 or 7
                    x.dilation
                        .last
                        .as_ref()
                        .filter(|x| x.dilation() != 1.)
                        .map(|x| x.dilation() / 10.)
                        .unwrap_or(0.)
                );
            if tween.is_none() {
                tween = x.dilation.tween();
            }
            lifecycle!("sync emitter handle dilation: {rate} {tween:?}");
            x.value_mut()
                .handles
                .statics
                .iter_mut()
                .filter(|x| x.affected_by_time_dilation)
                .for_each(|x| {
                    x.handle
                        .set_playback_rate(rate, tween.unwrap_or(IMMEDIATELY));
                });
            x.value_mut()
                .handles
                .streams
                .iter_mut()
                .filter(|x| x.affected_by_time_dilation)
                .for_each(|x| {
                    x.handle
                        .set_playback_rate(rate, tween.unwrap_or(IMMEDIATELY));
                });
        });
    }

    pub fn listener_id(&self) -> EntityId {
        self.v.id
    }

    pub fn emitters_count() -> i32 {
        EMITTERS.read().len() as i32
    }
}

#[derive(Debug)]
pub struct Dilation {
    value: f32,
    last: Option<DilationUpdate>,
}

impl Dilation {
    fn new(value: f32) -> Self {
        Self { value, last: None }
    }
    pub fn dilation(&self) -> f64 {
        match self.last {
            Some(ref update) => update.dilation(),
            None => self.value as f64,
        }
    }
    pub fn tween(&self) -> Option<Tween> {
        self.last.as_ref().map(|x| x.tween_curve())
    }
}

impl Emitter {
    fn infos(entity_id: EntityId) -> Result<(Vector4, bool), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let busy = entity
            .clone()
            .cast::<GameObject>()
            .map(AIActionHelper::is_in_workspot)
            .unwrap_or(false);
        let position = entity.get_world_position();
        Ok((position, busy))
    }

    #[allow(clippy::type_complexity)]
    fn full_infos(
        entity_id: EntityId,
    ) -> Result<
        (
            Option<PlayerGender>,
            Vector4,
            bool,
            Option<f32>,
            Option<EmitterDistances>,
        ),
        Error,
    > {
        let (position, busy) = Self::infos(entity_id)?;
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let gender = entity.get_gender();
        let distances = entity.get_emitter_distances();
        if !entity.is_a::<TimeDilatable>() {
            return Ok((gender, position, busy, None, distances));
        }
        let dilation = entity
            .clone()
            .cast::<TimeDilatable>()
            .as_ref()
            .map(AsTimeDilatable::get_time_dilation_value);
        Ok((gender, position, busy, dilation, distances))
    }
}

pub trait AsEntityExt {
    fn get_emitter_distances(&self) -> Option<EmitterDistances>;
    fn get_gender(&self) -> Option<PlayerGender>;
}

impl AsEntityExt for Ref<Entity> {
    fn get_emitter_distances(&self) -> Option<EmitterDistances> {
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
                    EmitterDistances::default()
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
