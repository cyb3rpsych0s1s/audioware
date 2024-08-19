# AudioSystem

Once defined, each [audio ID](./MANIFEST.md#anatomy) is automatically registered on startup,
making it available for scripting in-game.

If you simply want to play any custom sound, you can use [AudioSystem](https://nativedb.red4ext.com/AudioSystem) as you normally would for vanilla sounds.

```swift
GameInstance.GetAudioSystem(game).Play(n"my_custom_audio");
```

It also accepts any of the usual parameters that `AudioSystem`'s methods support.

Currently all these methods are supported:

- [Play](https://nativedb.red4ext.com/gameGameAudioSystem#Play)
- [Stop](https://nativedb.red4ext.com/gameGameAudioSystem#Stop)
- [Switch](https://nativedb.red4ext.com/gameGameAudioSystem#Switch)
- [PlayOnEmitter](https://nativedb.red4ext.com/gameGameAudioSystem#PlayOnEmitter)
- [Parameter](https://nativedb.red4ext.com/gameGameAudioSystem#Parameter): see [Parameters](./PARAMETERS.md)
