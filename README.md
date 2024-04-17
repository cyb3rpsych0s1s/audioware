# audioware

![Cyberpunk 2077 version compatibility](https://img.shields.io/badge/Cyberpunk_2077-patch_2.12a-yellow) [![Nexus](https://img.shields.io/badge/Nexus-Audioware-orange)](https://www.nexusmods.com/cyberpunk2077/mods/12001) [![download](https://img.shields.io/github/v/release/cyb3rpsych0s1s/audioware?display_name=tag&label=Download)](https://github.com/cyb3rpsych0s1s/audioware/releases/latest) [![build](https://github.com/cyb3rpsych0s1s/audioware/actions/workflows/quality.yml/badge.svg)](https://github.com/cyb3rpsych0s1s/audioware/actions)

CP2077 modding tool to expose a new audio backend.

Currently in its infancy, further information will be provided at a later time.

## features

- [x] automatically registers voices audio files with their localized subtitles from file in YAML format.
- [x] allows playing sounds **without** [REDmod](https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators/modding-tools/redmod/audio-modding#audio-modding-manually).
- [x] seamlessly integrates with vanilla `AudioSystem`, but currently restricted to:
      - `Play`
      - `Stop`
      - `Switch`
- [x] update reverb via blackboard
- [x] update player EQ preset via blackboard (`None` / `OnThePhone` / `Underwater`)
- [x] convenience method for NPC to talk over the phone, e.g.:
```swift
GameInstance.GetAudioSystem(game).PlayOverThePhone(n"some_custom_dialog_for_vik", n"Vik");
```

### dependent features

Thanks to [Codeware](https://github.com/psiberx/cp2077-codeware), this plugin will _automatically_:

- [x] register all subtitles for [localization](https://github.com/psiberx/cp2077-codeware/wiki#localization) via `ModLocalizationProvider` and `ModLocalizationPackage`.
- [x] show/hide subtitle in the appropriate language whenever a custom voice audio is played.

## credits

This initial release would never have been possible without the following persons, immense token of appreciation to:

- [@psiberx](https://github.com/psiberx): for being a formidable libraries author, one of the pillars of Cyberpunk community and having guided me literally throughout all this journey.
- [@jekky](https://github.com/jac3km4): for the very same reasons as above, but also for being the author of [red4ext-rs](https://github.com/jac3km4/red4ext-rs) on which this plugin is built upon, and for accepting my comments, reviewing my Pull Requests, providing me explanations and so on.
- [@DevNullx64](https://github.com/DevNullx64): for helping in my first steps in assembly and understanding basic C++, and simply for being a formidable friend.
- [@WopsS](https://github.com/WopsS): for being the author of [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK) on which all Cyberpunk modding tools are based.
- [@Nibana](https://linktr.ee/nibanamusic): for providing extensive explanations about audio in games in general, and testing out sound effects.
- [@catbus00](https://github.com/catbus00): for supporting me all along these months of struggle :)
