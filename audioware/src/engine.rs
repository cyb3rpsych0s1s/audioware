use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    spatial::scene::{SpatialSceneHandle, SpatialSceneSettings},
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle, TrackRoutes},
};
use lazy_static::lazy_static;
use red4ext_rs::types::EntityId;

use crate::{
    audio::{SoundId, StaticAudio},
    banks::BANKS,
    Audioware,
};

lazy_static! {
    static ref AUDIO: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
    static ref TRACKS: Arc<Mutex<HashMap<String, TrackHandle>>> =
        Arc::new(Mutex::new(HashMap::default()));
    static ref SCENE: Arc<Mutex<Option<SpatialSceneHandle>>> = Arc::new(Mutex::new(None));
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
            guard.insert("ambience".to_string(), ambience);
            guard.insert("player".to_string(), player);
            guard.insert("player:vocal".to_string(), vocal);
            guard.insert("player:emissive".to_string(), emissive);
            guard.insert("player:mental".to_string(), mental);
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
    fn get_audio(key: impl Into<SoundId>) -> Option<StaticAudio> {
        BANKS
            .clone()
            .try_lock()
            .ok()
            .and_then(|x| x.get_sound(key))
            .map(|x| x.audio())
    }
    pub(crate) fn play(&mut self, sound: impl Into<SoundId>) {
        if let Some(audio) = Self::get_audio(sound) {
            if let Ok(mut guard) = AUDIO.clone().borrow_mut().try_lock() {
                if let Some(manager) = guard.0.as_mut() {
                    let data = audio.data.unwrap();
                    if let Ok(track) = TRACKS.clone().try_lock() {
                        let track = track.get("player").unwrap();
                        data.settings.output_destination(track);
                        let _ = manager.play(data);
                    }
                }
            }
        }
    }
}

pub trait IsPlayer {
    fn is_player(&self) -> bool;
}

impl IsPlayer for EntityId {
    fn is_player(&self) -> bool {
        self == &EntityId::from(1)
    }
}
