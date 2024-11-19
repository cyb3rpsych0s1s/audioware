use std::{collections::HashSet, ops::DerefMut, sync::LazyLock};

use dashmap::{DashMap, DashSet};
use kira::{
    manager::{backend::Backend, AudioManager},
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    tween::Tween,
};
use parking_lot::RwLock;
use red4ext_rs::types::{CName, EntityId, GameInstance};
use smallvec::SmallVec;

use crate::{
    error::{Error, SceneError},
    get_player, AIActionHelper, AsEntity, AsGameInstance, Entity, GameObject, Vector4,
};

use super::{lifecycle, tracks::Tracks, tweens::IMMEDIATELY};

#[derive(Debug)]
pub struct Emitter {
    handle: EmitterHandle,
    handles: Handles,
    last_known_position: Vector4,
    busy: bool,
    pub names: DashSet<Option<CName>>,
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

static EMITTERS: LazyLock<RwLock<HashSet<(EntityId, Option<CName>)>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

/// Audio spatial scene.
pub struct Scene {
    pub emitters: DashMap<EntityId, Emitter>,
    pub v: ListenerHandle,
    pub listener_id: EntityId,
    pub scene: SpatialSceneHandle,
    synced: SmallVec<[EntityId; 32]>,
}

impl Scene {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        tracks: &Tracks,
    ) -> Result<Self, Error> {
        let settings = SpatialSceneSettings::default();
        let capacity = settings.emitter_capacity as usize;
        let mut scene = manager.add_spatial_scene(settings)?;
        let (listener_id, position, orientation) = {
            let v = get_player(GameInstance::new()).cast::<Entity>().unwrap();
            (
                v.get_entity_id(),
                v.get_world_position(),
                v.get_world_orientation(),
            )
        };
        let v = scene.add_listener(
            position,
            orientation,
            ListenerSettings::default().track(tracks.sfx.as_ref()),
        )?;
        *EMITTERS.write().deref_mut() = HashSet::with_capacity(capacity);
        Ok(Self {
            v,
            listener_id,
            scene,
            emitters: DashMap::with_capacity(capacity),
            synced: SmallVec::new(),
        })
    }

    fn emitter_infos(&self, entity_id: EntityId) -> Result<(Vector4, bool), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let busy = if entity.is_a::<GameObject>() {
            AIActionHelper::is_in_workspot(entity.clone().cast::<GameObject>().unwrap())
        } else {
            false
        };
        let position = entity.get_world_position();
        Ok((position, busy))
    }

    pub fn add_emitter(
        &mut self,
        entity_id: EntityId,
        emitter_name: Option<CName>,
        settings: Option<EmitterSettings>,
    ) -> Result<(), Error> {
        if entity_id == self.listener_id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        if let Some(emitter) = self.emitters.get_mut(&entity_id) {
            emitter.names.insert(emitter_name);
            return Ok(());
        }
        let (position, busy) = self.emitter_infos(entity_id)?;
        let emitter = self
            .scene
            .add_emitter(position, settings.unwrap_or_default())?;
        let names = DashSet::with_capacity(3);
        names.insert(emitter_name);
        let handle = Emitter {
            handle: emitter,
            handles: Default::default(),
            last_known_position: position,
            busy,
            names,
        };
        self.emitters.insert(entity_id, handle);
        EMITTERS.write().insert((entity_id, emitter_name));
        lifecycle!("added emitter {entity_id:?}");
        Ok(())
    }

    pub fn remove_emitter(&mut self, entity_id: EntityId) -> Result<bool, Error> {
        let removal: Vec<_> = self
            .emitters
            .iter()
            .filter_map(|x| {
                if *x.key() == entity_id {
                    Some(*x.key())
                } else {
                    None
                }
            })
            .collect();
        if removal.is_empty() {
            return Ok(false);
        }
        self.emitters.retain(|k, _| !removal.contains(k));
        EMITTERS.write().retain(|(id, _)| *id != entity_id);
        lifecycle!("removed emitter {entity_id:?}");
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
        self.v.set_position(position, IMMEDIATELY);
        self.v.set_orientation(orientation, IMMEDIATELY);
        Ok(())
    }

    fn sync_emitters(&mut self) -> Result<(), Error> {
        if self.emitters.is_empty() {
            return Ok(());
        }
        let mut synced: SmallVec<[EntityId; 32]> = SmallVec::with_capacity(self.emitters.len());
        let remove = |entity_id: EntityId| {
            EMITTERS.write().retain(|(id, _)| *id != entity_id);
            false
        };
        self.emitters.retain(|k, v| {
            if synced.contains(k) {
                return true;
            }
            let Ok((position, busy)) = self.emitter_infos(*k) else {
                return remove(*k);
            };
            v.busy = busy;
            v.last_known_position = position;
            // weirdly enough if emitter is not updated, sound(s) won't update as expected.
            // e.g. when listener moves but emitter stands still.
            v.handle.set_position(position, IMMEDIATELY);
            synced.push(*k);
            true
        });
        self.synced.clear();
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
        EMITTERS.write().clear();
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

    pub fn sync_dilation(&mut self, listener: f32, emitters: &[(EntityId, f32)]) {
        self.emitters
            .iter_mut()
            .filter(|x| emitters.iter().any(|(id, _)| id == x.key()))
            .for_each(|mut x| {
                x.value_mut().handles.statics.iter_mut().for_each(|x| {
                    x.handle.set_playback_rate(listener as f64, IMMEDIATELY);
                });
                x.value_mut().handles.streams.iter_mut().for_each(|x| {
                    x.handle.set_playback_rate(listener as f64, IMMEDIATELY);
                });
            });
    }
}
