#![allow(dead_code)]

use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use kira::{
    clock::ClockSpeed,
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    track::{
        effect::{delay::DelayBuilder, filter::FilterBuilder, reverb::ReverbBuilder},
        TrackBuilder, TrackHandle, TrackRoutes,
    },
    tween::Tween,
    Volume,
};
use lazy_static::lazy_static;
use metadata::MediaFileMetadata;

#[derive(Default)]
pub struct Audioware(Option<AudioManager<DefaultBackend>>);

lazy_static! {
    static ref MANAGER: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
    static ref SOUNDS: Arc<Mutex<HashMap<String, StaticSoundData>>> =
        Arc::new(Mutex::new(HashMap::default()));
    static ref TRACKS: Arc<Mutex<HashMap<String, Box<TrackHandle>>>> =
        Arc::new(Mutex::new(HashMap::default()));
}

pub fn first_test() -> Result<(), anyhow::Error> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let track = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(
            DelayBuilder::new()
                .delay_time(0.3)
                .feedback(Volume::Amplitude(0.4)),
        );
        builder
    })?;
    let clock = manager.add_clock(ClockSpeed::TicksPerSecond(10.))?;
    let as_if_i_didnt_know_already = StaticSoundData::from_file(
        "fem_v_aiidka.wav",
        StaticSoundSettings::new()
            .output_destination(&track)
            .start_time(clock.time())
            .panning(0.0),
    )?;
    manager.play(as_if_i_didnt_know_already)?;
    let come_on_biomon_cant_you_just_give_me_a_break = StaticSoundData::from_file(
        "fem_v_cobcygmab_02.wav",
        StaticSoundSettings::new()
            .output_destination(&track)
            .start_time(clock.time() + 5)
            .panning(1.0),
    )?;
    manager.play(come_on_biomon_cant_you_just_give_me_a_break)?;
    clock.start()?;
    std::thread::sleep(Duration::from_secs(8));
    Ok(())
}

pub fn second_test() -> Result<(), anyhow::Error> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let reverb = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(ReverbBuilder::new().mix(1.0));
        builder
    })?;
    let clock = manager.add_clock(ClockSpeed::TicksPerSecond(10.))?;
    let more = manager.add_sub_track({
        let mut builder = TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.8));
        builder.add_effect(DelayBuilder::new().delay_time(0.3));
        builder
    })?;
    let less = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.4)))?;
    let as_if_i_didnt_know_already = StaticSoundData::from_file(
        "fem_v_aiidka.wav",
        StaticSoundSettings::new()
            .output_destination(&less)
            .start_time(clock.time())
            .panning(0.0),
    )?;
    manager.play(as_if_i_didnt_know_already)?;
    let come_on_biomon_cant_you_just_give_me_a_break = StaticSoundData::from_file(
        "fem_v_cobcygmab_02.wav",
        StaticSoundSettings::new()
            .output_destination(&more)
            .start_time(clock.time() + 5)
            .panning(1.0),
    )?;
    manager.play(come_on_biomon_cant_you_just_give_me_a_break)?;
    clock.start()?;
    std::thread::sleep(Duration::from_secs(10));
    Ok(())
}

pub fn third_test() -> Result<(), anyhow::Error> {
    println!("in cell handler");
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let reverb = manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().mix(1.0));
            builder.add_effect(
                FilterBuilder::new()
                    .mode(kira::track::effect::filter::FilterMode::LowPass)
                    .resonance(1.0),
            );
            builder
        })
        .expect("reverb");
    let clock = manager
        .add_clock(ClockSpeed::TicksPerSecond(10.))
        .expect("clock");
    let more = manager
        .add_sub_track({
            let mut builder =
                TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.8));
            builder.add_effect(DelayBuilder::new().delay_time(0.3));
            builder
        })
        .expect("more");
    let less = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.4)))
        .expect("less");
    let as_if_i_didnt_know_already = StaticSoundData::from_file(
        "fem_v_aiidka.wav",
        StaticSoundSettings::new()
            .output_destination(&less)
            .start_time(clock.time())
            .panning(0.0),
    )
    .expect("first sound");
    let duration_aiidka = MediaFileMetadata::new(&Path::new("fem_v_aiidka.wav"))
        .expect("metadata")
        ._duration;
    println!("aiidka lasts for: {duration_aiidka:#?}");
    println!(
        "aiidka with another format: {:#?}",
        MediaFileMetadata::new(&Path::new("fem_v_aiidka.wav"))
            .expect("yoyoyo")
            .duration
    );
    let mut aiidka = manager.play(as_if_i_didnt_know_already).expect("play");
    aiidka
        .set_playback_rate(
            3.0,
            Tween {
                duration: Duration::from_secs(3),
                start_time: kira::StartTime::ClockTime(clock.time() + 5),
                easing: kira::tween::Easing::InPowi(2),
            },
        )
        .expect("set playback rate");
    let come_on_biomon_cant_you_just_give_me_a_break = StaticSoundData::from_file(
        "fem_v_cobcygmab_02.wav",
        StaticSoundSettings::new()
            .output_destination(&more)
            .start_time(clock.time() + 5)
            .panning(1.0),
    )
    .expect("second sound");
    let duration_cobcyjgmeab = MediaFileMetadata::new(&Path::new("fem_v_cobcygmab_02.wav"))
        .expect("metadata")
        ._duration;
    println!("cobcyjgmeab lasts for: {duration_cobcyjgmeab:#?}");
    let mut cobcyjgmeab = manager
        .play(come_on_biomon_cant_you_just_give_me_a_break)
        .expect("play");
    cobcyjgmeab
        .set_playback_rate(
            2.0,
            Tween {
                duration: Duration::from_secs(3),
                start_time: kira::StartTime::ClockTime(clock.time() + 15),
                ..Default::default()
            },
        )
        .expect("set playback rate");
    clock.start().expect("start clock");
    std::thread::sleep(Duration::from_secs(10));
    Ok(())
}

fn fourth_test() -> Result<(), anyhow::Error> {
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

fn initialize() {}

pub fn main() -> Result<(), anyhow::Error> {
    // first_test()
    // second_test()
    // third_test()
    fourth_test()
}
