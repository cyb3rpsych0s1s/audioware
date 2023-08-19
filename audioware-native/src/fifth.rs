use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::SpatialSceneSettings,
    },
    track::{
        effect::{
            reverb::{ReverbBuilder, ReverbHandle},
            Effect, EffectBuilder,
        },
        TrackBuilder, TrackHandle, TrackRoutes,
    },
    tween::Tween,
};
use lazy_static::lazy_static;

#[derive(Default)]
pub struct Audioware(Option<AudioManager<DefaultBackend>>);

lazy_static! {
    static ref MANAGER: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
    static ref DATA: Arc<Mutex<HashMap<String, StaticSoundData>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref TRACKS: Arc<Mutex<HashMap<String, Box<TrackHandle>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref SOUNDS: Arc<Mutex<HashMap<String, Box<StaticSoundHandle>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref EMITTERS: Arc<Mutex<HashMap<String, Box<EmitterHandle>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref LISTENERS: Arc<Mutex<HashMap<String, Box<ListenerHandle>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct Plugin;
impl Plugin {
    pub fn set(audioware: Audioware) -> bool {
        if let Ok(mut guard) = MANAGER.clone().try_lock() {
            *guard = audioware;
            return true;
        }
        false
    }
}
pub struct SoundsRegistry;
impl SoundsRegistry {
    pub fn insert(key: impl AsRef<str>, value: Box<StaticSoundHandle>) -> bool {
        if let Ok(mut guard) = SOUNDS.clone().try_lock() {
            guard.insert(key.as_ref().to_string(), value);
            return true;
        }
        false
    }
}
use glam::{Quat, Vec3};

pub fn fifth_test() -> Result<(), anyhow::Error> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let mut scene = manager.add_spatial_scene(SpatialSceneSettings::default())?;
    let _listener = scene.add_listener(Vec3::ZERO, Quat::IDENTITY, ListenerSettings::default())?;
    let emitter = scene.add_emitter(Vec3::new(0.5, 0.5, 0.0), EmitterSettings::default())?;
    let sound = StaticSoundData::from_file(
        "dialog.wav",
        StaticSoundSettings::new().output_destination(&emitter),
    )?;
    let duration = sound.duration();
    manager.play(sound)?;
    *MANAGER.clone().try_lock().unwrap() = Audioware(Some(manager));
    EMITTERS
        .clone()
        .try_lock()
        .unwrap()
        .insert("emitter".to_string(), Box::new(emitter));
    let whole = std::thread::spawn(move || {
        std::thread::sleep(duration);
    });
    let halfway = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs_f64(2.0));
        if let Some(ref mut guard) = EMITTERS.clone().try_lock().unwrap().get_mut("emitter") {
            guard
                .set_position(
                    Vec3::new(0.5, 20.0, 0.0),
                    Tween {
                        start_time: kira::StartTime::Immediate,
                        duration: Duration::from_secs_f64(5.0),
                        easing: kira::tween::Easing::Linear,
                    },
                )
                .unwrap();
        }
    });

    halfway.join().unwrap();
    whole.join().unwrap();
    Ok(())
}
