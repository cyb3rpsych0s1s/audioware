use std::time::Duration;

use kira::{
    clock::ClockSpeed,
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    track::{effect::delay::DelayBuilder, TrackBuilder},
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
