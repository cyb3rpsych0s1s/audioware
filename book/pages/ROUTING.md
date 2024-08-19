# Routing

Each section in the manifest will be routed to different tracks internally.

This is important because each track will come with different behavior.

## Volume settings

Depending on which section audio is defined, it will be affected by a specific game volume setting.

```admonish example title="Cyberpunk volume settings"
![Cyberpunk volume settings](./settings.png)

| track  | volume           
|--------|----------------
| sfx    | SfxVolume      
| onos   | DialogueVolume 
| voices | DialogueVolume 
| music  | MusicVolume    

Any audio gets affected by `MasterVolume`, as expected.
```

```admonish example title="YouTube demo"
[![See it in action!](https://img.youtube.com/vi/eE5jRxl8HAY/0.jpg)](https://www.youtube.com/watch?v=eE5jRxl8HAY&list=PLMu2na7a3T6MHJq_JJ6yx_2qRv4MYX9ez&index=4)
```

## Parameters

Likewise each track will be affected, or not, by [preset](./PARAMETERS.md#preset) and [reverb mix](./PARAMETERS.md#reverb-mix).

| track  | preset | reverb |
|--------|--------|--------|
| sfx    | ✅     | ✅      |
| onos   | ✅     | ✅      |
| voices | ✅     | ✅      |
| music  | ❌     | ❌      |

## Going beyond

This might sounds restrictive at first, but it's actually a way to provide good defaults while being easily worked-around when needed.

```admonish hint title="Tip"
Imagine you want to play a song affected by [underwater preset](./PARAMETERS.md#preset) when V dives underwater.

Even if you'd usually go for [music](./SECTIONS.md#music) section, nothing prevents from defining your audio in [sfx](./SECTIONS.md#sfx) instead with *streaming* [usage](./SETTINGS.md#usage) for example.
```
