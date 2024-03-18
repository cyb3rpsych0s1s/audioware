# Getting started

## Manifest

First, all audio files path and associated subtitles must be properly defined in a manifest.

`audioware`'s manifest is very similar to [REDmod](https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators/modding-tools/redmod/audio-modding#audio-modding-manually)'s, except that it uses `YAML` format instead of `JSON`.

The manifest must be located under `<your mod folder>\voices.yml`, in game folder.

> In the future, `audioware` will accept multiple manifests per mod folder, but for now it has to be named `voices`.

Here's a fictional example of a manifest:

```yaml
version: 1.0.0
voices:
  # an example of V's voice (which has to define both male and female gender variants)
  ono_hhuh:
    fem:
    en-us:
      # path must be relative to mod's folder, under <game folder>/mods/<mod folder>
      # path can be formatted as Unix-style or Windows-style, it doesn't matter
      file: path/to/file/for/english/female/version.Wav
      # subtitle that will automatically displayed with Codeware
      subtitle: Again?
    fr-fr:
      file: path/to/file/for/french/female/version.Wav
      subtitle: Encore ?
    male:
      en-us:
        file: path/to/file/for/english/male/version.Wav
        subtitle: Again?
      fr-fr:
        file: path/to/file/for/french/male/version.Wav
        subtitle: Encore ?

  # an example of Judy's voice (no gender variant)
  judy_lullaby:
  en-us:
    file: path/to/another/file.Wav
    subtitle: La-lalala-lala
```

## Use in-game

As simple as:
```swift reds
@addMethod(PlayerPuppet)
public func TestAudioware() -> Void {
    Game.GetAudioSystem(game).Play(n"ono_hhuh", this.GetEntityID(), n"V");
}
```