//! Cyberpunk 2077 dialog line types.

use serde::Deserialize;

/// See [NativeDB](https://nativedb.red4ext.com/scnDialogLineType).
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum ScnDialogLineType {
    #[serde(rename = "none")]
    None = 0,
    #[default]
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

impl ScnDialogLineType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Regular => "regular",
            Self::Holocall => "holocall",
            Self::SceneComment => "scene_comment",
            Self::OverHead => "over_head",
            Self::Radio => "radio",
            Self::GlobalTv => "global_tv",
            Self::Invisible => "invisible",
            Self::OverHeadAlwaysVisible => "over_head_always_visible",
            Self::OwnerlessRegular => "ownerless_regular",
            Self::AlwaysCinematicNoSpeaker => "always_cinematic_no_speaker",
            Self::GlobalTvAlwaysVisible => "global_tv_always_visible",
            Self::Narrator => "narrator",
        }
    }
}

#[cfg(not(test))]
unsafe impl red4ext_rs::NativeRepr for ScnDialogLineType {
    const NAME: &'static str = "scnDialogLineType";
}
