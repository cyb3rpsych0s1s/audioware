use std::{collections::HashSet, sync::LazyLock};

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
    get_player, AIActionHelper, AsEntity, AsGameInstance, AsScriptedPuppet, AsTimeDilatable,
    Entity, GameObject, ScriptedPuppet, TimeDilatable, ToTween, Vector4,
};

use super::{lifecycle, tracks::Tracks, tweens::IMMEDIATELY, Dilation};

#[derive(Debug)]
pub struct Listener {
    id: EntityId,
    handle: ListenerHandle,
    last_dilation: Option<Dilation>,
}

#[derive(Debug)]
pub struct Emitter {
    handle: EmitterHandle,
    handles: Handles,
    last_known_position: Vector4,
    busy: bool,
    pub names: DashSet<Option<CName>>,
    last_dilation: Option<Dilation>,
}

impl Emitter {
    pub fn store_static(&mut self, event_name: CName, handle: StaticSoundHandle) {
        self.handles.statics.push(Handle { event_name, handle });
    }
    pub fn store_stream(&mut self, event_name: CName, handle: StreamingSoundHandle<FromFileError>) {
        self.handles.streams.push(Handle { event_name, handle });
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

static EMITTERS: LazyLock<RwLock<HashSet<(EntityId, Option<CName>)>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

/// Audio spatial scene.
pub struct Scene {
    pub emitters: Emitters,
    pub v: Listener,
    pub scene: SpatialSceneHandle,
    dilation_changed: bool,
}

pub struct Emitters(DashMap<EntityId, Emitter>);

impl Emitters {
    fn with_capacity(capacity: usize) -> Self {
        *EMITTERS.write() = HashSet::with_capacity(capacity);
        Self(DashMap::with_capacity(capacity))
    }
    fn get_mut(&mut self, entity_id: &EntityId) -> Option<RefMut<'_, EntityId, Emitter>> {
        self.0.get_mut(entity_id)
    }
    pub fn get_mut_with_name(
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
        self.0.clear();
        EMITTERS.write().clear();
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
        let (listener_id, position, orientation, dilation) = {
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
                id: listener_id,
                handle,
                last_dilation: if dilation == 1. {
                    None
                } else {
                    Some(Dilation {
                        value: dilation,
                        curve: CName::default(),
                    })
                },
            },
            scene,
            emitters: Emitters::with_capacity(capacity),
            dilation_changed: true,
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
        let (position, busy, dilation, distances) = Emitter::full_infos(entity_id)?;
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
        let handle = Emitter {
            handle: emitter,
            handles: Default::default(),
            last_known_position: position,
            busy,
            names,
            last_dilation: dilation.map(|x| Dilation {
                value: x,
                curve: CName::default(),
            }),
        };
        self.emitters.insert(entity_id, emitter_name, handle);
        lifecycle!("added emitter {entity_id:?}");
        Ok(())
    }

    pub fn remove_emitter(&mut self, entity_id: EntityId) -> Result<bool, Error> {
        if self.emitters.remove(entity_id) {
            lifecycle!("removed emitter {entity_id:?}");
        }
        Ok(true)
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
            let Ok((position, busy)) = Emitter::infos(*k) else {
                EMITTERS.write().retain(|(id, _)| id != k);
                return false;
            };
            v.busy = busy;
            v.last_known_position = position;
            // weirdly enough if emitter is not updated, sound(s) won't update as expected.
            // e.g. when listener moves but emitter stands still.
            v.handle.set_position(position, IMMEDIATELY);
            // TODO:
            // if dilation != v.last_dilation.as_ref().map(|x| x.value) {
            //     v.last_dilation = dilation.map(|x| Dilation {
            //         value: x,
            //         curve: CName::default(),
            //     });
            //     dilation_changed = true;
            // }
            true
        });
        // if dilation_changed {
        //     self.dilation_changed = true;
        // }
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_listener()?;
        self.sync_emitters()?;
        if self.dilation_changed {
            self.sync_dilation();
        }
        Ok(())
    }

    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        EMITTERS.read().iter().any(|(id, _)| *id == entity_id)
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        self.emitters.retain(|k, v| {
            if *k == entity_id {
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
                true
            }
        });
    }

    pub fn any_emitter(&self) -> bool {
        !self.emitters.is_empty()
    }

    pub fn clear(&mut self) {
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

    pub fn stop_for(&mut self, entity_id: EntityId, tween: Tween) {
        self.emitters.iter_mut().for_each(|mut x| {
            if *x.key() == entity_id {
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
            }
        });
    }

    pub fn set_listener_dilation(&mut self, dilation: Option<Dilation>) {
        if self.v.last_dilation.as_ref().map(|x| x.value).unwrap_or(1.)
            != dilation.as_ref().map(|x| x.value).unwrap_or(1.)
        {
            self.v.last_dilation = dilation;
            self.dilation_changed = true;
        }
    }

    pub fn unset_listener_dilation(&mut self, dilation: Option<Dilation>) {
        self.v.last_dilation = dilation;
        self.dilation_changed = true;
    }

    pub fn set_emitter_dilation(&mut self, entity_id: EntityId, dilation: Option<Dilation>) {
        if let Some(mut emitter) = self.emitters.get_mut(&entity_id) {
            if dilation.as_ref().map(|x| x.value).unwrap_or(1.)
                != emitter
                    .last_dilation
                    .as_ref()
                    .map(|x| x.value)
                    .unwrap_or(1.)
            {
                emitter.last_dilation = dilation;
                self.dilation_changed = true;
            }
        }
    }

    pub fn unset_emitter_dilation(&mut self, entity_id: EntityId) {
        todo!()
    }

    fn sync_dilation(&mut self) {
        let listener = self
            .v
            .last_dilation
            .as_ref()
            .map(|x| x.value as f64)
            .unwrap_or(1.); // e.g. 0.7
        let mut tween = self
            .v
            .last_dilation
            .as_ref()
            .and_then(|x| x.curve.into_tween());
        let mut rate: f64 = 1.;
        self.emitters.iter_mut().for_each(|mut x| {
            rate = 1. - (1. - listener)
                + (
                    // e.g. 5 or 7
                    1. - x
                        .last_dilation
                        .as_ref()
                        .filter(|x| x.value != 1.)
                        .map(|x| x.value as f64 / 10.)
                        .unwrap_or(1.)
                );
            if tween.is_none() {
                tween = x.last_dilation.as_ref().and_then(|x| x.curve.into_tween());
            }
            x.value_mut().handles.statics.iter_mut().for_each(|x| {
                x.handle
                    .set_playback_rate(rate, tween.unwrap_or(IMMEDIATELY));
            });
            x.value_mut().handles.streams.iter_mut().for_each(|x| {
                x.handle
                    .set_playback_rate(rate, tween.unwrap_or(IMMEDIATELY));
            });
        });
        self.dilation_changed = false;
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

    fn full_infos(
        entity_id: EntityId,
    ) -> Result<(Vector4, bool, Option<f32>, Option<EmitterDistances>), Error> {
        let (position, busy) = Self::infos(entity_id)?;
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let distances = entity.get_emitter_distances();
        if !entity.is_a::<TimeDilatable>() {
            return Ok((position, busy, None, distances));
        }
        let dilation = entity
            .clone()
            .cast::<TimeDilatable>()
            .as_ref()
            .map(AsTimeDilatable::get_time_dilation_value);
        Ok((position, busy, dilation, distances))
    }
}

pub trait AsEntityExt {
    fn get_emitter_distances(&self) -> Option<EmitterDistances>;
}

impl AsEntityExt for Ref<Entity> {
    fn get_emitter_distances(&self) -> Option<EmitterDistances> {
        self.clone()
            .cast::<ScriptedPuppet>()
            .as_ref()
            .map(AsScriptedPuppet::get_npc_type)
            .map(|x| match x {
                crate::GamedataNpcType::Device
                | crate::GamedataNpcType::Drone
                | crate::GamedataNpcType::Spiderbot => EmitterDistances {
                    min_distance: 1.,
                    max_distance: 10.,
                },
                crate::GamedataNpcType::Android | crate::GamedataNpcType::Human => {
                    EmitterDistances {
                        min_distance: 2.,
                        max_distance: 16.,
                    }
                }
                crate::GamedataNpcType::Cerberus
                | crate::GamedataNpcType::Chimera
                | crate::GamedataNpcType::Mech => EmitterDistances {
                    min_distance: 5.,
                    max_distance: 40.,
                },
                crate::GamedataNpcType::Invalid
                | crate::GamedataNpcType::Count
                | crate::GamedataNpcType::Any => EmitterDistances::default(),
            })
    }
}
