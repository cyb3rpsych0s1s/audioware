const IMAGE_BASE: usize = 0x140000000;

/// memory address for [AudioSystem::Play](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L10)
pub const ON_AUDIOSYSTEM_PLAY: usize = 0x140974F58 - IMAGE_BASE;
/// memory address for [AudioSystem::Stop](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L12)
pub const ON_AUDIOSYSTEM_STOP: usize = 0x1424503C8 - IMAGE_BASE;
/// memory address for [AudioSystem::Switch](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L14)
pub const ON_AUDIOSYSTEM_SWITCH: usize = 0x140291688 - IMAGE_BASE;
