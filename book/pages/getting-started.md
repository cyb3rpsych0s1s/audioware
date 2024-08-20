# Getting started

Welcome to audioware book !

Audioware is a native plugin to play custom audios in Cyberpunk 2077, *without* REDmod.

It's built with emphasis on getting you going fast, providing sensible defaults and seamless integration with the game while not compromising on performances.

Here's the simplest way to test it out in under 5min.

## ↘️ Install

- grab [Audioware latest release](https://github.com/cyb3rpsych0s1s/audioware/releases/latest) and unzip it in root game folder.
- make sure you have both [Codeware 1.11.1+](https://github.com/psiberx/cp2077-codeware/releases) and [TweakXL 1.10.2+](https://github.com/psiberx/cp2077-tweak-xl/releases) installed too.

## 📄 Define audios

Create a folder e.g. `MyMod` for your mod, located in either depot:

- under `mods\MyMod`
- or `r6\audioware\MyMod`
but not both !

Create a manifest e.g. `audios.yml`.

In its simplest form, here's how it looks like:

```yml
version: 1.0.0
sfx:
  my_custom_audio: some.mp3 # accepts most common formats like .wav / .ogg / .mp3 / .flac
```

## ⏯️ Use in-game

```swift
GameInstance.GetAudioSystem(game).Play(n"my_custom_audio");
```

---

If you want to look at what you can further do, head over to [How to use?](./USAGE.md) for more.
