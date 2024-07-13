use red4ext_rs::NativeRepr;
use serde::Deserialize;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum ScnDialogLineType {
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "regular")]
    Regular = 1,
    #[serde(rename = "holocall")]
    Holocall = 2,
    #[serde(rename = "scene_comment")]
    SceneComment = 3,
    #[serde(rename = "over_head")]
    OverHead = 4,
    #[serde(rename = "radio")]
    Radio = 5,
    #[serde(rename = "global_tv")]
    GlobalTv = 6,
    #[serde(rename = "invisible")]
    Invisible = 7,
    #[serde(rename = "over_head_always_visible")]
    OverHeadAlwaysVisible = 9,
    #[serde(rename = "ownerless_regular")]
    OwnerlessRegular = 10,
    #[serde(rename = "always_cinematic_no_speaker")]
    AlwaysCinematicNoSpeaker = 11,
    #[serde(rename = "global_tv_always_visible")]
    GlobalTvAlwaysVisible = 12,
    #[serde(rename = "narrator")]
    Narrator = 13,
}

unsafe impl NativeRepr for ScnDialogLineType {
    const NAME: &'static str = "scnDialogLineType";
}
