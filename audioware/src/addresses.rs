#![allow(dead_code)]

const IMAGE_BASE: usize = 0x140000000;

/// memory address for internal event handler related to [RED4ext::ent::AudioEvent](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/ent/AudioEvent.hpp#L18)
pub const ON_ENT_AUDIO_EVENT: usize = 0x140453D34 - IMAGE_BASE;
pub const ON_MUSIC_EVENT: usize = 0x1424A6AC0 - IMAGE_BASE;
pub const ON_VOICE_EVENT: usize = 0x14068FD98 - IMAGE_BASE;
/// memory address for [AudioSystem::Play](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L10)
pub const ON_AUDIOSYSTEM_PLAY: usize = 0x140975FE4 - IMAGE_BASE;
/// memory address for [AudioSystem::Stop](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L12)
pub const ON_AUDIOSYSTEM_STOP: usize = 0x142440C30 - IMAGE_BASE;
pub const ON_AUDIOSYSTEM_SWITCH: usize = 0x140690804 - IMAGE_BASE;
pub const ON_AUDIOSYSTEM_GET_PLAYLIST_CURRENT_SONG: usize = 0x142440554 - IMAGE_BASE;
pub const ON_AUDIOSYSTEM_GET_PLAYLIST_SONGS: usize = 0x142440624 - IMAGE_BASE;
pub const ON_AUDIOSYSTEM_REQUEST_SONG_ON_PLAYLIST: usize = 0x1424408D0 - IMAGE_BASE;
pub const ON_AUDIOSYSTEM_REQUEST_SONG_ON_RADIO_STATION: usize = 0x1424409D4 - IMAGE_BASE;

pub const ON_AUDIOSYSTEM_OPEN_ACOUSTIC_PORTAL: usize = 0x1408E8100 - IMAGE_BASE;

/// memory address for [Entity::QueueEvent](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/entity/entity.swift#L6)
pub const ON_ENTITY_QUEUE_EVENT: usize = 0x1403B86B4 - IMAGE_BASE;
/// memory address for [IComponent::QueueEntityEvent](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/orphans.swift#L16430)
pub const ON_ICOMPONENT_QUEUE_ENTITY_EVENT: usize = 0x140A07B74 - IMAGE_BASE;
