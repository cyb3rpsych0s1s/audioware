//! Hooks for native functions and event handlers.

#[rustfmt::skip]
#[doc(hidden)]
mod offsets {
    // lifecycle
    pub(super) const LOAD_SOUNDBANKS: u32                   = 0xEC201278; // 0x1409B551C (2.13)
    // AudioSystem
    pub(super) const PARAMETER: u32                         = 0x7A491F19; // 0x14244FE90
    #[allow(dead_code)]
    pub(super) const GLOBAL_PARAMETER: u32                  = 0x4BA8216A; // 0x140E43ABC
    pub(super) const PLAY: u32                              = 0xCDB11D0E; // 0x140974F58
    pub(super) const PLAY_ON_EMITTER: u32                   = 0x48D20A5;  // 0x141C01EF0
    pub(super) const STOP: u32                              = 0xD2781D1E; // 0x1424503F8
    pub(super) const SWITCH: u32                            = 0x15081DEA; // 0x140291688
    // GameObject
    pub(super) const ON_TRANSFORM_UPDATED: u32              = 0x2AF1B37;  // 0x14014E8D0
    // Entity
    pub(super) const QUEUE_EVENT: u32                       = 0x5E7D1BB0; // 0x1401EE8A8
    pub(super) const QUEUE_EVENT_FOR_ENTITY_ID: u32         = 0xC1661FE1; // 0x140C5D350
    // inkLogicController / inkIGameController
    #[allow(dead_code)]
    pub(super) const ILC_QUEUE_EVENT: u32                   = 0xC87F2007; // 0x1409D4F9C
    // inkMenuScenario
    #[allow(dead_code)]
    pub(super) const IMS_QUEUE_EVENT: u32                   = 0x19751DF7; // 0x14123DC70
    // gameuiSaveHandlingController
    // note: LoadSaveInGame and LoadModdedSave share same underlying address
    pub(super) const LOAD_SAVE_IN_GAME: u32                 = 0x9AB824D9; // 0x14058E6B8
    
    // events handler
    pub(super) const PLAY_OR_STOP_SOUND_HANDLER: u32        = 0x297D0DEA; // 0x141D1FB8C
    pub(super) const STOP_OR_SWITCH_SOUND_HANDLER: u32      = 0x58A80EED; // 0x141D1FB98 / 0x141D1FB8C
    pub(super) const SOUND_SWITCH_HANDLER: u32              = 0x58AD0EEE; // 0x141D1FBB0 / 0x141D1FB98
    pub(super) const STOP_TAGGED_SOUNDS_OR_SOUND_PARAMETER_HANDLER: u32 = 0xDBD14617; // 0x140E17C14 // ❌
    #[allow(dead_code)]
    pub(super) const SOUND_PARAMETER_HANDLER: u32           = 0x58CD0EF6; // 0x141D1FBBC / 0x141D1FBA4 // ❌
    pub(super) const SOUND_PLAY_VO_HANDLER: u32             = 0x9E0C26F5; // 0x1409C20DC
    pub(super) const DIALOG_LINE_HANDLER: u32               = 0x10E71E89; // 0x1409C12A8
    pub(super) const DIALOG_LINE_END_HANDLER: u32           = 0x6F24331;  // 0x141188BF4
    pub(super) const STOP_DIALOG_LINE_HANDLER: u32          = 0x38D1DDC;  // 0x1417CDC70
    pub(super) const STOP_SOUND_ON_EMITTER_HANDLER: u32     = 0x2B7F217B; // 0x1424726A8
    pub(super) const PLAY_SOUND_ON_EMITTER_HANDLER: u32     = 0x2808216B; // 0x142472658
    pub(super) const SET_PARAMETER_ON_EMITTER_HANDLER: u32  = 0x932B2299; // 0x142472680
    pub(super) const VOICE_EVENT_HANDLER: u32               = 0xEBB01A67; // 0x1408C1A9C
    pub(super) const VOICE_PLAYED_EVENT_HANDLER: u32        = 0x2F42185;  // 0x1409C12B0
    pub(super) const SURFACE_EVENT_HANDLER: u32             = 0xF1D03EE5; // 0x1411889FC
    pub(super) const DIVE_EVENT_HANDLER: u32                = 0x636D3C63; // 0x141188AA4
    pub(super) const EMERGE_EVENT_HANDLER: u32              = 0x66603DFD; // 0x141188954
    pub(super) const MUSIC_EVENT_HANDLER: u32               = 0x77873E4B; // 0x1411E72E8
    pub(super) const SOUND_EVENT_HANDLER: u32               = 0xDA4F1A61; // 0x14252E384
    pub(super) const SPAWN_EFFECT_EVENT_HANDLER: u32        = 0xA4911D02; // 0x140394DE8
    pub(super) const AUDIO_EVENT_HANDLER: u32               = 0x10C412FD; // 0x140816DF4
    pub(super) const INTERACTION_CHOICE_EVENT_HANDLER: u32  = 0x5E971C49; // 0x14281097C
    pub(super) const VEHICLE_AUDIO_EVENT_HANDLER: u32       = 0x69EF1461; // 0x1417B43E0
}

pub mod global_parameter;
pub mod load_save_in_game;
pub mod load_soundbanks;
pub mod on_transform_updated;
pub mod parameter;
pub mod play;
pub mod play_on_emitter;
#[allow(dead_code)]
pub mod queue_event;
#[allow(dead_code)]
pub mod queue_event_for_entity_id;
pub mod stop;
pub mod switch;

#[allow(dead_code)]
pub mod events;
