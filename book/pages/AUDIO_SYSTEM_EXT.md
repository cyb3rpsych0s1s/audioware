# AudioSystemExt

[AudioSystemExt](../../audioware/reds/Ext.reds) is an enhanced system over Cyberpunk's [AudioSystem](https://nativedb.red4ext.com/AudioSystem).

This system exposes both *the exact same API* as its counterpart, but *also* similar methods with *additional* parameters.

For example, if you want your audio to fade-in linearly during 5secs:

```swift
import Audioware.LinearTween

//                                      ⬇️ notice extension here
let system = GameInstance.GetAudioSystemExt(game);
//                                                                   ⬇️ additional parameter
system.Play(n"my_custom_audio", GetPlayer(game).GetEntityID(), n"V", LinearTween.Immediate(5.));
```

```admonish warning
Note that any of these additional parameters **only** work with audio defined in Audioware.

> e.g. you cannot use a fade-in tween with *non-reexported* vanilla audio, see below.
```

```admonish tip
If you want to use vanilla audio with Audioware, you can still convert + export them from WolvenKit as described [in their Wiki](https://wiki.redmodding.org/wolvenkit/wolvenkit-app/usage/video-and-audio#audio), then re-define them normally in your [Manifest](./MANIFEST.md).

> ⚠️ make sure to use a [supported audio format](./getting-started.md#-define-audios)
```

Using Codeware, you could e.g. quickly create atmosphere like so:

```swift
let weather = GameInstance.GetWeatherSystem(game);
    weather.SetWeather(n"24h_weather_rain", 20.0, 9u);
    GameInstance.GetAudioSystemExt(game).Play(n"milles_feuilles");
```

```admonish example title="YouTube demo"
[![See it in action!](https://img.youtube.com/vi/Vlk0Ve8j4ck/0.jpg)](https://www.youtube.com/watch?v=Vlk0Ve8j4ck&list=PLMu2na7a3T6MHJq_JJ6yx_2qRv4MYX9ez)
```
