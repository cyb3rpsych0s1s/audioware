use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use dashmap::DashMap;
use glam::{Quat, Vec3};
use kira::{
    manager::AudioManager,
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    OutputDestination,
};
use parking_lot::{RwLock, RwLockWriteGuard};
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance, Ref},
    PluginOps,
};

use crate::{
    error::{Error, InternalError, SceneError},
    types::{get_player, AsEntity, AsGameInstance, Entity, Vector4},
    Audioware,
};

use super::{effects::IMMEDIATELY, id::EmitterId, Tracks};

mod emitters;

static SCENE: OnceLock<Scene> = OnceLock::new();

pub struct Scene {
    pub scene: Arc<Mutex<SpatialSceneHandle>>,
    pub v: Arc<Mutex<ListenerHandle>>,
    pub active_entities: Arc<Mutex<DashMap<EmitterId, EmitterHandle>>>,
    pub dead_entities: Arc<RwLock<Vec<EntityId>>>,
}

impl Scene {
    pub fn setup(manager: &mut AudioManager, tracks: &Tracks) -> Result<(), Error> {
        let settings = SpatialSceneSettings::default();
        let capacity = settings.emitter_capacity as usize;
        let mut scene = manager.add_spatial_scene(settings)?;
        let listener = scene.add_listener(
            Vec3::ZERO,
            Quat::IDENTITY,
            ListenerSettings::default().track(tracks.sfx.as_ref()),
        )?;
        SCENE
            .set(Scene {
                scene: Arc::new(Mutex::new(scene)),
                v: Arc::new(Mutex::new(listener)),
                active_entities: Arc::new(Mutex::new(DashMap::with_capacity(capacity))),
                dead_entities: Arc::new(RwLock::new(Vec::with_capacity(capacity))),
            })
            .map_err(|_| Error::from(InternalError::Contention { origin: "scene" }))?;
        Ok(())
    }
    fn try_lock_scene<'a>() -> Result<MutexGuard<'a, SpatialSceneHandle>, InternalError> {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .scene
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene handle",
            })
    }
    fn try_lock_listener<'a>() -> Result<MutexGuard<'a, ListenerHandle>, InternalError> {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .v
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene listener handle",
            })
    }
    fn try_lock_active_emitters<'a>(
    ) -> Result<MutexGuard<'a, DashMap<EmitterId, EmitterHandle>>, InternalError> {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .active_entities
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene emitters handles",
            })
    }
    fn try_write_dead_emitters<'a>() -> Result<RwLockWriteGuard<'a, Vec<EntityId>>, InternalError>
    {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .dead_entities
            .try_write()
            .ok_or(InternalError::Contention {
                origin: "spatial scene dead emitters",
            })
    }
    pub fn register_emitter(
        entity_id: EntityId,
        emitter_name: Option<CName>,
        emitter_settings: Option<EmitterSettings>,
    ) -> Result<(), Error> {
        if !entity_id.is_defined() {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let position = entity.get_world_position();
        let mut scene = Self::try_lock_scene()?;
        let emitters = Self::try_lock_active_emitters()?;
        let emitter = scene.add_emitter(position, emitter_settings.unwrap_or_default())?;
        emitters.insert(EmitterId::new(entity_id, emitter_name), emitter);
        log::info!(
            Audioware::env(),
            "registered emitter: {:?} -> {:?}",
            entity_id,
            position
        );
        Ok(())
    }
    pub fn unregister_emitter(entity_id: &EntityId) -> Result<(), Error> {
        log::info!(Audioware::env(), "unregistering emitter: {:?}", entity_id);
        Self::try_write_dead_emitters()?.push(*entity_id);
        log::info!(Audioware::env(), "unregistered emitter: {:?}", entity_id);
        Ok(())
    }
    pub fn emitters_count() -> Result<usize, Error> {
        Ok(Self::try_lock_active_emitters()?.len())
    }
    pub fn clear_emitters() -> Result<(), Error> {
        Self::try_lock_active_emitters()?.clear();
        Ok(())
    }
    pub fn on_emitter_dies(entity_id: EntityId) -> Result<(), Error> {
        Self::try_write_dead_emitters()?.push(entity_id);
        Ok(())
    }
    pub fn sync_emitters() -> Result<(), Error> {
        // log::info!(Audioware::env(), "syncing emitters positions...");
        let mut entity: Ref<Entity>;
        let mut position: Vector4;
        if let (Ok(ref mut actives), Ok(mut updates)) = (
            Self::try_lock_active_emitters(),
            Self::try_write_dead_emitters(),
        ) {
            updates.sort();
            updates.dedup();
            let removals = updates.drain(..).collect::<Vec<_>>();
            std::mem::drop(updates);
            actives.retain(|k, _| !removals.as_slice().contains(k.entity_id()));
            for mut entry in actives.iter_mut() {
                entity =
                    GameInstance::find_entity_by_id(GameInstance::new(), *entry.key().entity_id());
                if entity.is_null() {
                    continue;
                }
                position = entity.get_world_position();
                entry.value_mut().set_position(position, IMMEDIATELY);
            }
        } else {
            log::warn!(Audioware::env(), "sync emitters contention");
        }
        // log::info!(Audioware::env(), "synced emitters positions!");
        Ok(())
    }
    pub fn sync_listener() -> Result<(), Error> {
        if let Ok(v) = Self::try_lock_listener().as_deref_mut() {
            let player = get_player(GameInstance::new());
            if player.is_null() {
                return Ok(());
            }
            let entity = player.cast::<Entity>().unwrap();
            let position = entity.get_world_position();
            let orientation = entity.get_world_orientation();
            v.set_position(position, IMMEDIATELY);
            v.set_orientation(orientation, IMMEDIATELY);
        }
        Ok(())
    }
    pub fn output_destination(entity_id: &EntityId) -> Option<OutputDestination> {
        Self::try_lock_active_emitters().ok().and_then(|x| {
            x.iter().find_map(|entry| {
                if entry.key().entity_id() == entity_id {
                    Some(entry.value().into())
                } else {
                    None
                }
            })
        })
    }
    pub fn is_registered_emitter(entity_id: &EntityId) -> bool {
        if let Ok(emitters) = Self::try_lock_active_emitters() {
            for entry in emitters.iter() {
                if entry.key().entity_id() == entity_id {
                    return true;
                }
            }
        }
        false
    }
}
