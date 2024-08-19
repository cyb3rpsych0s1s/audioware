# Manifest

Let's take a closer at manifest's anatomy.

## Anatomy

A manifest expects a `version` and sections like `sfx`, `onos`, `voices` or `music`.

Each section contains one or multiple `audio ID`(s) which points to `audio file path`.

```yml
version: 1.0.0
# ⮦ section
sfx:
# ⮦ audio ID       ⮦ audio file path
  my_custom_audio: some.mp3
```

```admonish danger
each audio ID (e.g. `my_custom_audio`) is automatically added to game's [CName pool](https://jekky.dev/red4ext-rs/red4ext_rs/types/struct.CNamePool.html) on startup, so make sure they are *truly* uniques (across all game and all mods).
```

```admonish warning
Each audio file path validated on game startup: if it fails the game will not crash, but the error will be reported in the logs at `red4ext\logs\audioware-xyz.log` and in the [CET Game Log](https://wiki.redmodding.org/cyber-engine-tweaks/console/console).
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
