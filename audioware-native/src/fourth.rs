use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle},
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

pub fn fourth_test() -> Result<(), anyhow::Error> {
    const HALF_SEC: f32 = 0.5;
    const ONE_SEC: u64 = 1;
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let reverb = Box::new(manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(ReverbBuilder::new().mix(0.4));
        builder
    })?);
    let as_if_i_didnt_know_already = StaticSoundData::from_file(
        "fem_v_aiidka.wav",
        StaticSoundSettings::new()
            .output_destination(&*reverb)
            .panning(0.0),
    )
    .expect("first sound");
    let aiidka = Box::new(
        manager
            .play(as_if_i_didnt_know_already.clone())
            .expect("play"),
    );
    SoundsRegistry::insert("aiidka", aiidka);
    SOUNDS
        .clone()
        .try_lock()
        .unwrap()
        .get_mut("aiidka".into())
        .unwrap()
        .set_playback_rate(
            2.0,
            Tween {
                duration: Duration::from_secs_f32(HALF_SEC),
                start_time: kira::StartTime::Immediate,
                easing: kira::tween::Easing::InPowi(2),
            },
        )
        .expect("set playback rate");
    *MANAGER.clone().try_lock().unwrap() = Audioware(Some(manager));
    DATA.clone().try_lock().unwrap().insert(
        "as_if_i_didnt_know_already".to_string(),
        as_if_i_didnt_know_already,
    );
    TRACKS
        .clone()
        .try_lock()
        .unwrap()
        .insert("reverb".into(), reverb);
    println!("launched");
    std::thread::sleep(Duration::from_secs_f32(HALF_SEC));
    println!("{HALF_SEC} after");
    SOUNDS
        .clone()
        .try_lock()
        .unwrap()
        .get_mut("aiidka".into())
        .unwrap()
        .set_playback_rate(
            3.0,
            Tween {
                duration: Duration::from_secs(ONE_SEC),
                start_time: kira::StartTime::Immediate,
                easing: kira::tween::Easing::InPowi(1),
            },
        )
        .expect("set playback rate later");
    println!("switch playback rate");
    std::thread::sleep(Duration::from_secs(ONE_SEC));
    println!("{ONE_SEC} after");
    Ok(())
}
