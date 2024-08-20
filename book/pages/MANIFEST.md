# Manifest

A manifest is a simple `YAML`[^YAML] file to describe your sounds.
Audioware then use them to build sounds banks on game startup.

Let's take a closer look at a Manifest's anatomy.

## Anatomy

A manifest is a `.yml`[^YAML] file located in a folder named after your mod inside one of 2 depots.

It expects a `version` and sections like `sfx`, `onos`, `voices` or `music`.

It must be defined:

- in a folder named after your mod (e.g. `mods\MyMod`) itself located inside any valid depot (`mods` or `r6\audioware`),
- alongside audio files: files can be located in sub-folders, at your discretion.

Each section contains one or multiple `audio ID`(s) which points to an `audio file path`.

When using a simple audio file, you can write it inline.

```yml
version: 1.0.0
# ⬇️ section
sfx:
# ⬇️ audio ID      ⬇️ audio file path
  my_custom_audio: some.mp3
```

But you can also write nested properties for settings:

```yml
version: 1.0.0
sfx:
  my_custom_audio:
    file: some.mp3 
    settings:
      volume: 4.0 # 4 times louder!
```

All audio accepts multiple optional [settings](./SETTINGS.md).

### Validation

What is worth mentioning is that Audioware will actually validate *many properties* of your audios on game startup, here's a *non-exhaustive* list.

```admonish info title="Validation error(s)"
In any case error(s) *will never* crash your game, Audioware will instead ignore the invalid entries and report them both in the logs at `red4ext\logs\audioware-xyz.log` and in [CET Game Log](https://wiki.redmodding.org/cyber-engine-tweaks/console/console).
```

```admonish danger title="IDs must be uniques"
each audio ID (e.g. `my_custom_audio`) is automatically added to game's [CName pool](https://jekky.dev/red4ext-rs/red4ext_rs/types/struct.CNamePool.html) on startup, so make sure they are *truly* uniques (across all game and all mods).
```

```admonish warning title="Files must be located inside depot"
Audio files can be defined at your convenience at the root of your mod folder inside its depot, or any sub-folder, but they **cannot** be located outside.
```

```admonish warning title="Files must be valid"
Each audio file is briefly preloaded on game startup to make sure they will play just fine during your game session.
```

```admonish warning title="Audio settings must be valid"
Each setting defined alongside audios must be valid.

> e.g. specifying a `start_position` that is shorter than audio's duration is **not**!
```

```admonish tip title="Validation deep-dive"
If you would like to know exactly how Audioware validates entries, consider browsing [unit-tests files](https://github.com/cyb3rpsych0s1s/audioware/tree/main/manifest/src/de).
```

### Guarantees

The reason behind these numerous validation checks: it then allows Audioware to make <span style="color: #f3d772">assumptions</span> about your sounds bank.

```admonish info title="Guarantees"
Upholding these invariants <span style="color: hotpink">guarantees</span> for example that any `audio ID` that makes it into Audioware both <span style="color: hotpink">exists</span> and can be <span style="color: hotpink">safely loaded</span> without further need for runtime validation, <span style="color: hotpink">increasing overall in-game performances!</span>
```

```admonish danger title="Don't be THAT person!"
Of course if you delete audio file(s) while your game is running, Audioware will **crash** as soon as called with e.g. `Play`. This is **expected**.

Let's be pragmatic 1sec: if you do so, you probably *deserve* your game to crash anyway 😂
```

```admonish warning title="NO hot-reloading"
Since everything is loaded at-most **once** on startup, Audioware does **not** currently provide any way to hot-reload your audio assets in-game.

> 💡 To get you going fast during mod development, you can rely on [AudioSettingsExt](./AUDIO_SETTINGS_EXT.md) instead to adjust audio effects until you get them to your liking, then write them down in your [Manifest](./MANIFEST.md).
```

[^YAML]: YAML is a file format, see [how to write your own](https://circleci.com/blog/what-is-yaml-a-beginner-s-guide/).
