const IMAGE_BASE: usize = 0x140000000;

/// memory address for [AudioSystem::Play](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L10)
pub const ON_AUDIOSYSTEM_PLAY: usize = 0x140974F58 - IMAGE_BASE;
/// memory address for [AudioSystem::Stop](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L12)
pub const ON_AUDIOSYSTEM_STOP: usize = 0x1424503F8 - IMAGE_BASE;
/// TODO: hex pattern
pub const ON_AUDIOSYSTEM_GLOBAL_PARAMETER: usize = 0x140E43ABC - IMAGE_BASE;
/// TODO: hex pattern
pub const ON_AUDIOSYSTEM_PARAMETER: usize = 0x14244FE90 - IMAGE_BASE;

/// TODO: hex pattern
pub const ON_MUSIC_EVENT: usize = 0x1424B63C8 - IMAGE_BASE;

/// TODO: hex pattern
pub const ON_VOICE_EVENT: usize = 0x1408C1A9C - IMAGE_BASE;
/// TODO: hex pattern
pub const ON_VOICEPLAYED_EVENT: usize = 0x1409C12B0 - IMAGE_BASE;

/// TODO: hex pattern
/// NOTE: these ones are a combination of:
/// - gameaudioeventsPlaySound:                       0x141D1FB8C
/// - gameaudioeventsStopSound:         0x141D1FB98 / 0x141D1FB8C
/// - gameaudioeventsSoundSwitch:       0x141D1FB98 / 0x141D1FBB0
/// - gameaudioeventsStopTaggedSounds:  0x141D1FBA4
/// - gameaudioeventsSoundParameter:    0x141D1FBA4 / 0x141D1FBBC
#[rustfmt::skip]
pub const ON_PLAYSOUND_EVENT: usize         = 0x141D1FB8C - IMAGE_BASE;
#[rustfmt::skip] #[allow(dead_code)]
pub const ON_STOPSOUND_EVENT: usize         = 0x141D1FB98 - IMAGE_BASE;
#[rustfmt::skip]
pub const ON_SOUNDSWITCH_EVENT: usize       = 0x141D1FBB0 - IMAGE_BASE;
#[rustfmt::skip]
pub const ON_STOPTAGGEDSOUNDS_EVENT: usize  = 0x141D1FBA4 - IMAGE_BASE;
#[rustfmt::skip]
pub const ON_SOUNDPARAMETER_EVENT: usize    = 0x141D1FBBC - IMAGE_BASE;

/// TODO: hex pattern
pub const ON_DIALOGLINE_EVENT: usize = 0x1417CDC80 - IMAGE_BASE;
/// TODO: hex pattern
pub const ON_DIALOGLINEEND_EVENT: usize = 0x1417CDC70 - IMAGE_BASE;
/// TODO: hex pattern
pub const ON_STOPDIALOGLINE_EVENT: usize = 0x140A0C838 - IMAGE_BASE;
