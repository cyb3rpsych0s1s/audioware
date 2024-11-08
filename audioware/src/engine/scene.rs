use std::sync::{
    atomic::{AtomicBool, AtomicI32},
    LazyLock, Mutex, MutexGuard,
};

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
    types::{get_player, AsEntity, AsGameInstance, Entity, GameObject, Vector4},
    utils, AIActionHelper, Audioware,
};

use super::{effects::IMMEDIATELY, id::EmitterId, Tracks};

static SCENE_SYNC_ENABLED: AtomicBool = AtomicBool::new(false);
static NUM_EMITTERS: AtomicI32 = AtomicI32::new(-1);
static EMITTERS: LazyLock<RwLock<Vec<EmitterId>>> = LazyLock::new(Default::default);

/// Audio spatial scene.
pub struct Scene {
    pub scene: Mutex<SpatialSceneHandle>,
    pub v: Mutex<ListenerHandle>,
    pub active_entities: Mutex<DashMap<EmitterId, EmitterHandle>>,
    pub dead_entities: RwLock<Vec<EntityId>>,
    pub busy_entities: RwLock<Vec<EntityId>>,
}

impl Scene {
    pub fn try_new(manager: &mut AudioManager, tracks: &Tracks) -> Result<Self, Error> {
        let settings = SpatialSceneSettings::default();
        let capacity = settings.emitter_capacity as usize;
        let mut scene = manager.add_spatial_scene(settings)?;
        let listener = scene.add_listener(
            Vec3::ZERO,
            Quat::IDENTITY,
            ListenerSettings::default().track(tracks.sfx.as_ref()),
        )?;
        Ok(Self {
            scene: Mutex::new(scene),
            v: Mutex::new(listener),
            active_entities: Mutex::new(DashMap::with_capacity(capacity)),
            dead_entities: RwLock::new(Vec::with_capacity(capacity)),
            busy_entities: RwLock::new(Vec::with_capacity(capacity)),
        })
    }
    fn try_lock_scene(&self) -> Result<MutexGuard<'_, SpatialSceneHandle>, InternalError> {
        self.scene
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene handle",
            })
    }
    fn try_lock_listener(&self) -> Result<MutexGuard<'_, ListenerHandle>, InternalError> {
        self.v.try_lock().map_err(|_| InternalError::Contention {
            origin: "spatial scene listener handle",
        })
    }
    fn try_lock_active_emitters(
        &self,
    ) -> Result<MutexGuard<'_, DashMap<EmitterId, EmitterHandle>>, InternalError> {
        self.active_entities
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene emitters handles",
            })
    }
    fn try_write_dead_emitters(
        &self,
    ) -> Result<RwLockWriteGuard<'_, Vec<EntityId>>, InternalError> {
        self.dead_entities
            .try_write()
            .ok_or(InternalError::Contention {
                origin: "spatial scene dead emitters",
            })
    }
    fn try_write_busy_emitters(
        &self,
    ) -> Result<RwLockWriteGuard<'_, Vec<EntityId>>, InternalError> {
        self.busy_entities
            .try_write()
            .ok_or(InternalError::Contention {
                origin: "spatial scene busy emitters",
            })
    }
    pub fn toggle_sync_emitters(enable: bool) {
        SCENE_SYNC_ENABLED.store(enable, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn register_emitter(
        &mut self,
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
        let busy = if entity.is_a::<GameObject>() {
            AIActionHelper::is_in_workspot(entity.clone().cast::<GameObject>().unwrap())
        } else {
            false
        };
        let position = entity.get_world_position();
        let mut scene = self
            .scene
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene handle",
            })?;
        let emitters = self.try_lock_active_emitters()?;
        let emitter = scene.add_emitter(position, emitter_settings.unwrap_or_default())?;
        emitters.insert(EmitterId::new(entity_id, emitter_name), emitter);
        if busy {
            self.try_write_busy_emitters()?.push(entity_id);
        }
        utils::silly!("registered emitter: {:?} -> {:?}", entity_id, position);
        Ok(())
    }
    pub fn unregister_emitter(&mut self, entity_id: &EntityId) -> Result<(), Error> {
        utils::silly!("unregistering emitter: {:?}", entity_id);
        self.try_write_dead_emitters()?.push(*entity_id);
        utils::silly!("unregistered emitter: {:?}", entity_id);
        Ok(())
    }
    pub fn emitters_count() -> i32 {
        NUM_EMITTERS.load(std::sync::atomic::Ordering::Acquire)
    }
    pub fn clear_emitters(&mut self) -> Result<(), Error> {
        self.try_lock_active_emitters()?.clear();
        Ok(())
    }
    pub fn on_emitter_dies(&mut self, entity_id: EntityId) -> Result<(), Error> {
        self.try_write_dead_emitters()?.push(entity_id);
        Ok(())
    }
    pub fn should_sync_emitters() -> bool {
        SCENE_SYNC_ENABLED.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn sync_emitters(&self) -> Result<(), Error> {
        // utils::silly!("syncing emitters positions...");
        let mut entity: Ref<Entity>;
        let mut position: Vector4;
        if let (Ok(ref mut actives), Ok(mut deaths), Ok(mut busy)) = (
            self.try_lock_active_emitters(),
            self.try_write_dead_emitters(),
            self.try_write_busy_emitters(),
        ) {
            deaths.sort();
            deaths.dedup();
            let removals = deaths.drain(..).collect::<Vec<_>>();
            let occupied = busy.drain(..).collect::<Vec<_>>();
            std::mem::drop(deaths);
            actives.retain(|k, _| {
                !removals.as_slice().contains(k.entity_id()) && !occupied.contains(k.entity_id())
            });
            for mut entry in actives.iter_mut() {
                entity =
                    GameInstance::find_entity_by_id(GameInstance::new(), *entry.key().entity_id());
                if entity.is_null() {
                    continue;
                }
                if entity.is_a::<GameObject>()
                    && AIActionHelper::is_in_workspot(entity.clone().cast::<GameObject>().unwrap())
                {
                    busy.push(*entry.key().entity_id());
                }
                position = entity.get_world_position();
                entry.value_mut().set_position(position, IMMEDIATELY);
            }
        } else {
            log::warn!(Audioware::env(), "sync emitters contention");
        }
        // utils::silly!("synced emitters positions!");
        Ok(())
    }
    pub fn sync_listener(&self) -> Result<(), Error> {
        if let Ok(v) = self.try_lock_listener().as_deref_mut() {
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
    pub fn output_destination(&self, entity_id: &EntityId) -> Option<OutputDestination> {
        self.try_lock_active_emitters().ok().and_then(|x| {
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
        EMITTERS.read().iter().any(|x| x.entity_id() == entity_id)
    }
}
