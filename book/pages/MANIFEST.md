# Manifest

Let's take a closer at manifest's anatomy.

## Anatomy

A manifest is a YAML file which expects a `version` and sections like `sfx`, `onos`, `voices` or `music`.

It must be defined in either depot (`mods` or `r6\audioware`) alongside audio files (files can be located in sub-folders).

Each section contains one or multiple `audio ID`(s) which points to `audio file path`.

```yml
version: 1.0.0
# ⬇️ section
sfx:
# ⬇️ audio ID      ⬇️ audio file path
  my_custom_audio: some.mp3
```

```admonish warning title="Validation"
Each audio file path validated on game startup: see below for more details.
```

### ⚙️ Optional settings

Previously a simple audio file path was used, so it can be written inline.

Same definition, with optional nested setting:

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

As stated above, Audioware will actually validate *many properties* of your audios on game startup, here's a *non-exhaustive* list.

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

The reason behind these numerous validation checks is that it then allows Audioware to make assumptions about your sounds bank.

```admonish info title="Guarantees"
Upholding these invariants guarantees for example that all `audio IDs` that makes it into its internal registry are both *guaranteed* to exists and safe to load without any need for further runtime validation, increasing overall in-game performances!

> Of course if you delete audio file(s) while your game is running, it will **crash** as soon as it's called with e.g. `Play`.
>
> But let's be pragmatic: if you do so, you probably deserve your game to crash anyway :D
```
