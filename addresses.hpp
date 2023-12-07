///// EVENT HANDLERS

/// @pattern 48 89 5C 24 ? 48 89 74  24 ? 57 48 83 EC ? 48  8D B9 ? ? ? ? 48 8B F2 48 83 3F ? 48 8B D9 74 ?
/// memory address for internal event handler related to [RED4ext::ent::AudioEvent](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/ent/AudioEvent.hpp#L18)
typedef void EntAudioEvent_Handler(void* aObject, void* entAudioEvent);

/// @pattern 4C 8B DC 53 48 83 EC ?  48 8B 99 ? ? ? ? C6  81 ? ? ? ? ? 33 C9 48 85 DB 74 ?
typedef void MusicEvent_Handler(void* aObject, void* musicEvent);

/// @pattern 48 89 5C 24 ? 48 89 74  24 ? 55 57 41 56 48 8B  EC 48 83 EC ? 65 48 8B  04 25 ? ? ? ? 48 8B  F9 B9 ? ? ? ? 48 8B  F2 4C 8B 00 42 8B 04 01  39 05 ? ? ? ? 0F 8F  ? ? ? ?
typedef void VoiceEvent_Handler(void* aObject, void* voiceEvent);

///// NATIVES

/// @pattern 48 89 5C 24 ? 48 89 74  24 ? 48 89 7C 24 ? 55  48 8B EC 48 83 EC ? 48  8B 02 48 8D 3D ? ? ?  ? FE 42 ? 4C 8D 45 ?  33 F6 45 33 C9 48 89 75  ? 48 8B DA 48 89 72 ?  48 89 72 ? 0F B6 08 48  FF C0 48 89 02 8B C1  48 8B 4A ? FF ? ? 48 8B  03 4C 8D 45 ? FE 43 ?  45 33 C9 48 89 75 ? 48  8B D3 48 89 73 ? 48 89  73 ? 0F B6 08 48 FF C0  48 89 03 8B C1 48 8B 4B  ? FF 14 C7 48 8B 03 4C  8D 45 ? FE 43 ? 45 33  C9 48 89 75 ? 48 8B D3  48 89 73 ? 48 89 73 ?  0F B6 08 48 FF C0 48 89  03 8B C1 48 8B 4B ? FF  14 C7 48 8B 55 ? 48 FF  03 48 85 D2 0F 85 ? ?  ? ?
/// memory address for [AudioSystem::Play](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L10)
typedef void AudioSystem_Play(void* aContext, void* aFrame, float* aOut, void* a4);

/// @pattern 48 89 5C 24 ? 48 89 74  24 ? 48 89 7C 24 ? 55  48 8B EC 48 83 EC ? 48  8B 02 48 8D 3D ? ? ?  ? FE 42 ? 4C 8D 45 ?  33 F6 45 33 C9 48 89 75  ? 48 8B DA 48 89 72 ?  48 89 72 ? 0F B6 08 48  FF C0 48 89 02 8B C1  48 8B 4A ? FF ? ? 48 8B  03 4C 8D 45 ? FE 43 ?  45 33 C9 48 89 75 ? 48  8B D3 48 89 73 ? 48 89  73 ? 0F B6 08 48 FF C0  48 89 03 8B C1 48 8B 4B  ? FF 14 C7 48 8B 03 4C  8D 45 ? FE 43 ? 45 33  C9 48 89 75 ? 48 8B D3  48 89 73 ? 48 89 73 ?  0F B6 08 48 FF C0 48 89  03 8B C1 48 8B 4B ? FF  14 C7 48 8B 55 ? 48 FF  03 48 85 D2 74 32
/// memory address for [AudioSystem::Stop](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/systems/audioSystem.swift#L12)
typedef void AudioSystem_Stop(void* aContext, void* aFrame, float* aOut, void* a4);

/// @pattern 48 89 5C 24 ? 57 48 83  EC ? 48 8B 02 4C 8D 15  ? ? ? ? 48 83 62 ?  ? 0F 57 C0 48 83 62 ?  ? 48 8B F9 FE 42 ? 45  33 C9 48 8B 4A ? 48 8B  DA F3 0F 7F 44 24 ? 44  0F B6 00 48 FF C0 48 89  02 41 8B C0 4C 8D 44 24  ? 41 FF 14 C2 48 FF 03  48 8D 54 24 ? 48 8B CF  E8 ? ? ? ? 48 8D 4C  24 ? E8 ? ? ? ? 48  8B 5C 24 ? 48 83 C4 ?  5F C3
/// @nth 4/23
/// memory address for [Entity::QueueEvent](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/core/entity/entity.swift#L6)
typedef void Entity_QueueEvent(void* aContext, void* aFrame, float* aOut, void* a4);

/// @pattern 48 89 5C 24 ? 57 48 83  EC ? 48 8B 02 4C 8D 15  ? ? ? ? 48 83 62 ?  ? 0F 57 C0 48 83 62 ?  ? 48 8B F9 FE 42 ? 45  33 C9 48 8B 4A ? 48 8B  DA F3 0F 7F 44 24 ? 44  0F B6 00 48 FF C0 48 89  02 41 8B C0 4C 8D 44 24  ? 41 FF 14 C2 48 FF 03  48 8D 54 24 ? 48 8B CF  E8 ? ? ? ? 48 8D 4C  24 ? E8 ? ? ? ? 48  8B 5C 24 ? 48 83 C4 ?  5F C3
/// @nth 9/23
/// memory address for [IComponent::QueueEntityEvent](https://codeberg.org/adamsmasher/cyberpunk/src/branch/master/orphans.swift#L16164)
typedef void IComponent_QueueEntityEvent(void* aContext, void* aFrame, float* aOut, void* a4);
