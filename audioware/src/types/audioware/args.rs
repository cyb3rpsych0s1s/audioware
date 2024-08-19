use red4ext_rs::{class_kind::Native, types::Ref, NativeRepr, ScriptClass};

use super::Tween;

#[derive(Default, Clone)]
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

unsafe impl ScriptClass for Args {
    type Kind = Native;
    const NAME: &'static str = <Self as NativeRepr>::NAME;
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct LoopRegion {
    pub starts: f32,
    pub ends: f32,
}

unsafe impl NativeRepr for LoopRegion {
    const NAME: &'static str = "Audioware.LoopRegion";
}

unsafe impl ScriptClass for LoopRegion {
    type Kind = Native;
    const NAME: &'static str = <Self as NativeRepr>::NAME;
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
