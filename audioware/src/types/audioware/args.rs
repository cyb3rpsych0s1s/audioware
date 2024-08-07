use kira::{
    sound::{static_sound::StaticSoundSettings, streaming::StreamingSoundSettings, PlaybackRate},
    tween::Value,
    OutputDestination, Volume,
};
use red4ext_rs::{types::Ref, NativeRepr};

use super::{ToTween, Tween};

#[repr(C)]
pub struct Args {
    start_position: f32,
    volume: f32,
    panning: f32,
    playback_rate: f32,
    loop_region: LoopRegion,
    fade_in_tween: Ref<Tween>,
}

unsafe impl NativeRepr for Args {
    const NAME: &'static str = "Audioware.Args";
}

pub struct Unchecked<T>(T);

macro_rules! impl_unchecked {
    ($ty:ident) => {
        impl From<self::Args> for self::Unchecked<$ty> {
            fn from(value: Args) -> Self {
                let out: $ty = $ty {
                    start_time: kira::StartTime::Immediate,
                    start_position: kira::sound::PlaybackPosition::Seconds(
                        value.start_position as f64,
                    ),
                    loop_region: Some(value.loop_region.into()),
                    volume: kira::tween::Value::Fixed(Volume::Amplitude(value.volume as f64)),
                    playback_rate: kira::tween::Value::Fixed(PlaybackRate::Factor(
                        value.playback_rate as f64,
                    )),
                    panning: kira::tween::Value::Fixed(value.panning as f64),
                    fade_in_tween: value.fade_in_tween.into_tween(),
                    ..Default::default()
                };
                Self(out)
            }
        }
    };
}

impl_unchecked!(StaticSoundSettings);
impl_unchecked!(StreamingSoundSettings);

#[derive(Debug)]
#[repr(C)]
pub struct LoopRegion {
    pub starts: f32,
    pub ends: f32,
}

unsafe impl NativeRepr for LoopRegion {
    const NAME: &'static str = "Audioware.LoopRegion";
}

impl From<LoopRegion> for kira::sound::Region {
    fn from(value: LoopRegion) -> Self {
        Self {
            start: kira::sound::PlaybackPosition::Seconds(value.starts as f64),
            end: kira::sound::EndPosition::Custom(kira::sound::PlaybackPosition::Seconds(
                value.ends as f64,
            )),
        }
    }
}
