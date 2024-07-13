use red4ext_rs_bindings::ScnDialogLineType;
use serde::{Deserialize, Deserializer};

#[repr(u32)]
#[derive(Deserialize)]
pub(crate) enum ScnDialogLineTypeDef {
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

impl From<ScnDialogLineTypeDef> for ScnDialogLineType {
    fn from(value: ScnDialogLineTypeDef) -> Self {
        match value {
            ScnDialogLineTypeDef::None => Self::None,
            ScnDialogLineTypeDef::Regular => Self::Regular,
            ScnDialogLineTypeDef::Holocall => Self::Holocall,
            ScnDialogLineTypeDef::SceneComment => Self::SceneComment,
            ScnDialogLineTypeDef::OverHead => Self::OverHead,
            ScnDialogLineTypeDef::Radio => Self::Radio,
            ScnDialogLineTypeDef::GlobalTv => Self::GlobalTv,
            ScnDialogLineTypeDef::Invisible => Self::Invisible,
            ScnDialogLineTypeDef::OverHeadAlwaysVisible => Self::OverHeadAlwaysVisible,
            ScnDialogLineTypeDef::OwnerlessRegular => Self::OwnerlessRegular,
            ScnDialogLineTypeDef::AlwaysCinematicNoSpeaker => Self::AlwaysCinematicNoSpeaker,
            ScnDialogLineTypeDef::GlobalTvAlwaysVisible => Self::GlobalTvAlwaysVisible,
            ScnDialogLineTypeDef::Narrator => Self::Narrator,
        }
    }
}

pub fn deserialize_scn_dialog_line_type<'de, D>(
    deserializer: D,
) -> Result<ScnDialogLineType, D::Error>
where
    D: Deserializer<'de>,
{
    let helper = ScnDialogLineTypeDef::deserialize(deserializer)?;
    Ok(helper.into())
}

pub fn deserialize_optional_scn_dialog_line_type<'de, D>(
    deserializer: D,
) -> Result<Option<ScnDialogLineType>, D::Error>
where
    D: Deserializer<'de>,
{
    let helper = Option::<ScnDialogLineTypeDef>::deserialize(deserializer)?;
    Ok(helper.map(Into::into))
}
