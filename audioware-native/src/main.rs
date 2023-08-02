#![allow(dead_code)]

use std::time::Duration;

use kira::{
    clock::ClockSpeed,
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    track::{
        effect::{delay::DelayBuilder, filter::FilterBuilder, reverb::ReverbBuilder},
        TrackBuilder, TrackRoutes,
    },
    tween::Tween,
    Volume,
};

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
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let reverb = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(ReverbBuilder::new().mix(1.0));
        builder.add_effect(
            FilterBuilder::new()
                .mode(kira::track::effect::filter::FilterMode::LowPass)
                .resonance(1.0),
        );
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
    let mut cobcyjgmeab = manager.play(come_on_biomon_cant_you_just_give_me_a_break)?;
    cobcyjgmeab.set_playback_rate(
        2.0,
        Tween {
            duration: Duration::from_secs(3),
            start_time: kira::StartTime::ClockTime(clock.time() + 15),
            ..Default::default()
        },
    )?;
    clock.start()?;
    std::thread::sleep(Duration::from_secs(10));
    Ok(())
}

pub fn main() -> Result<(), anyhow::Error> {
    third_test()
}
