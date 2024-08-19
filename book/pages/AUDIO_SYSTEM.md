# AudioSystem

Once defined, each [audio ID](./MANIFEST.md#anatomy) is automatically registered on startup,
making it available for scripting in-game.

If you simply to play a sound, you can use [AudioSystem](https://nativedb.red4ext.com/AudioSystem) as you normally would for vanilla sounds.

```swift
GameInstance.GetAudioSystem(game).Play(n"my_custom_audio");
```

It also accepts any of the usual parameters that `AudioSystem`'s methods support.
