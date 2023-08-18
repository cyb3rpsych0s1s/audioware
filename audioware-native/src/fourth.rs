use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle},
    tween::Tween,
};
use lazy_static::lazy_static;

#[derive(Default)]
pub struct Audioware(Option<AudioManager<DefaultBackend>>);

lazy_static! {
    static ref MANAGER: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
    static ref SOUNDS: Arc<Mutex<HashMap<String, StaticSoundData>>> =
        Arc::new(Mutex::new(HashMap::default()));
    static ref TRACKS: Arc<Mutex<HashMap<String, Box<TrackHandle>>>> =
        Arc::new(Mutex::new(HashMap::default()));
}

pub fn fourth_test() -> Result<(), anyhow::Error> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let reverb = Box::new(manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(ReverbBuilder::new().mix(1.0));
        builder
    })?);
    let as_if_i_didnt_know_already = StaticSoundData::from_file(
        "fem_v_aiidka.wav",
        StaticSoundSettings::new()
            .output_destination(&*reverb)
            .panning(0.0),
    )
    .expect("first sound");
    SOUNDS.clone().try_lock().unwrap().insert(
        "as_if_i_didnt_know_already".to_string(),
        as_if_i_didnt_know_already.clone(),
    );
    let mut aiidka = manager.play(as_if_i_didnt_know_already).expect("play");
    aiidka
        .set_playback_rate(
            3.0,
            Tween {
                duration: Duration::from_secs(3),
                start_time: kira::StartTime::Immediate,
                easing: kira::tween::Easing::InPowi(2),
            },
        )
        .expect("set playback rate");
    *MANAGER.clone().try_lock().unwrap() = Audioware(Some(manager));
    TRACKS
        .clone()
        .try_lock()
        .unwrap()
        .insert("reverb".into(), reverb);
    println!("launched");
    std::thread::sleep(Duration::from_secs(7));
    Ok(())
}
