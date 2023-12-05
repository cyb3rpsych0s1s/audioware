use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    spatial::scene::{SpatialSceneHandle, SpatialSceneSettings},
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle, TrackId, TrackRoutes},
};
use lazy_static::lazy_static;

use crate::Audioware;

lazy_static! {
    static ref AUDIO: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
    static ref TRACKS: Arc<Mutex<HashMap<NamedTrackId, TrackHandle>>> =
        Arc::new(Mutex::new(HashMap::default()));
    static ref SCENE: Arc<Mutex<Option<SpatialSceneHandle>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug)]
pub struct NamedTrackId(&'static str, TrackId);

impl PartialEq for NamedTrackId {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
impl Eq for NamedTrackId {}
impl std::hash::Hash for NamedTrackId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl Audioware {
    fn setup() -> anyhow::Result<AudioManager> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().mix(1.0));
            builder
        })?;
        let ambience = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.5)),
        )?;
        let player = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.25)),
        )?;
        let vocal = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&player, 1.)),
        )?;
        let mental = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&player, 1.)),
        )?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&player, 1.)),
        )?;
        let environment = manager.add_spatial_scene(SpatialSceneSettings::default())?;

        if let Ok(mut guard) = TRACKS.clone().borrow_mut().try_lock() {
            guard.insert(NamedTrackId("ambience", ambience.id()), ambience);
            guard.insert(NamedTrackId("player", player.id()), player);
            guard.insert(NamedTrackId("player:vocal", vocal.id()), vocal);
            guard.insert(NamedTrackId("player:emissive", emissive.id()), emissive);
            guard.insert(NamedTrackId("player:mental", mental.id()), mental);
        }
        if let Ok(mut guard) = SCENE.clone().borrow_mut().try_lock() {
            *guard = Some(environment);
        }
        Ok(manager)
    }
    pub(crate) fn create() -> anyhow::Result<()> {
        match Self::setup() {
            Ok(manager) => match AUDIO.clone().borrow_mut().try_lock() {
                Ok(mut guard) => {
                    *guard = Self(Some(manager));
                    Ok(())
                }
                Err(_) => anyhow::bail!("unable to store audioware's handle"),
            },
            Err(_) => anyhow::bail!("unable to setup audioware's audio engine"),
        }
    }
    pub(crate) fn destroy() -> anyhow::Result<()> {
        match AUDIO.clone().borrow_mut().try_lock() {
            Ok(mut guard) => {
                *guard = Self(None);
                Ok(())
            }
            Err(_) => anyhow::bail!("unable to destroy audioware"),
        }
    }
}
