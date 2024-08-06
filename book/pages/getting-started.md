# Getting started

## Manifest

First of all, audio files path and associated metadata (subtitles, settings, etc) must be properly defined in a manifest.

Your mod's manifest and audio assets can be located either:
- under `<game-folder>\mods\<mod-name>`
- or `<game folder>\r6\audioware\<mod name>`

`Audioware`'s manifest is very similar to [REDmod](https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators/modding-tools/redmod/audio-modding#audio-modding-manually)'s, except that it uses `YAML` format instead of `JSON`.

In its simplest form, here's how it looks like:

```yaml
version: 1.0.0
sfx:
  my_custom_sfx: my-custom-sfx.wav
```

## Use in-game

As simple as:
```swift reds
@addMethod(PlayerPuppet)
public func TestAudioware() -> Void {
    Game.GetAudioSystem(this.GetGame()).Play(n"my_custom_sfx");
}
```