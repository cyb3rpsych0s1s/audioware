# AudioSystemExt

[AudioSystemExt](https://github.com/cyb3rpsych0s1s/audioware/blob/main/audioware/reds/Ext.reds) is an enhanced system over Cyberpunk's [AudioSystem](https://nativedb.red4ext.com/AudioSystem).

This system exposes both *the exact same API* as its counterpart, but *also* similar methods with *additional* parameters.

For example, if you want your audio to fade-in linearly during 5secs:

```swift
import Audioware.LinearTween

//                                      ⬇️ notice 'Ext' extension here
let system = GameInstance.GetAudioSystemExt(game);
let v = GetPlayer(game).GetEntityID();

// play audio                            ⬇️ with 5s linear fade-in
system.Play(n"my_custom_audio", v, n"V", LinearTween.Immediate(5.));

// later on, stop audio                  ⬇️ with 2s linear fade-out
system.Stop(n"my_custom_audio", v, n"V", LinearTween.Immediate(2.));
```

```admonish warning
Note that any of these additional parameters <span style="color: #f3d772">only</span> work with audio defined in Audioware.

> e.g. you cannot use a fade-in tween with *non-reexported* vanilla audio, see below.
```

```admonish tip
If you want to use vanilla audio with Audioware, you can still convert + export them from WolvenKit as described [in their Wiki](https://wiki.redmodding.org/wolvenkit/wolvenkit-app/usage/video-and-audio#audio), then re-define them normally in your [manifest](./MANIFEST.md).

> ⚠️ make sure to use a [supported audio format](./GETTING-STARTED.md#-define-audios)
```

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/yUAQ5id3ZA0?si=yf8TKX2TPtd64uaV" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```

## Going further

Combined with [Codeware](https://github.com/psiberx/cp2077-codeware), you can e.g. quickly create atmosphere like so:

```swift
let weather = GameInstance.GetWeatherSystem(game);
weather.SetWeather(n"24h_weather_rain", 20.0, 9u);
GameInstance.GetAudioSystemExt(game).Play(n"milles_feuilles");
```

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/Vlk0Ve8j4ck?si=xX0WaPwi-fzH3TxW" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```
