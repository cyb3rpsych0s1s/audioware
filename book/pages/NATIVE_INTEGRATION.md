# Native sounds integration

Starting from `1.6.0`, Audioware allows to listen and/or mute _any_ vanilla audio event _at runtime_.

## Audio events callback system

Its callback system, albeit more simplistic, is very similar to [Codeware's CallbackSystem](https://github.com/psiberx/cp2077-codeware/wiki#reference), so you can feel at home.

## Register callbacks

```swift
class MyService extends ScriptableService {
  private cb func OnLoad() {
    let system = new AudioEventCallbackSystem();
    // listen to any 'game_occlusion' audio events
    system.RegisterCallback(n"game_occlusion", this, n"OnOcclusion");
    // listen only to 'Mixing_Output_Cinema' audio event of type 'Play'
    system
        .RegisterCallback(n"Mixing_Output_Cinema", this, n"OnMixingOutput")
        .AddTarget(EventTarget.ActionType(audioEventActionType.Play));
  }

  private cb func OnOcclusion(event: ref<SoundEvent>) {
    FTLog("occlusion changed!");
  }

  private cb func OnMixingOutput(event: ref<PlayEvent>) {
    FTLog("play mixing output!");
  }
}
```

```admonish info title="Difference with Codeware callback system"
As with Codeware's, we can register callbacks to class member or static methods, filter targets to be notified about and define whether callback are automatically unregistered when a game session, or persist for as long as the game runs. Audioware just does not expose option like `CallbackRunMode` yet.
```

## Additional considerations

No matter their initial type and origin, the final audio event types _roughly_ match [audioEventActionType enum](https://nativedb.red4ext.com/audioEventActionType), but here's the twist: when the game dispatches events, they are not played directly as-is.

They are enqueued first, then depending on many factors happening in-game (distance from the player, velocity, number of concurrent sounds already being played, etc) they can be played, rescheduled or simply cancelled. Their fields can also change all throughout processing until final play.

- it's worth noting that `Play` and `PlayAnimation` events are ultimately either played from memory (a.k.a `play` from the sound engine's perspective) or from an external file (a.k.a `play_external`).
- nother type of play event exist in the sound engine but not in scripting APIs: `PlayOneShot` for short "fire and forget" sounds like weapon firing for example.
- likewise, there's both `SetParameter` and `SetGlobalParameter` events, even if `SetGlobalParameter` is not a variant of [audioEventActionType enum](https://nativedb.red4ext.com/audioEventActionType).

For this reason, Audioware exposes an additional `enum` which is closer to the sound engine core types: `EventHookType`, which can be used when registering callbacks:

```swift
let system = new AudioEventCallbackSystem();
// listen to 'game_window_in_focus' global parameter event
system
    .RegisterCallback(n"game_window_in_focus", this, n"OnWindowsInFocus")
    .AddTarget(EventTarget.HookType(EventHookType.SetGlobalParameter));
```

```admonish info title="Conversion between EventHookType and audioEventActionType"
Audioware conversion from `audioEventActionType` to `EventHookType` is "best effort" only.
```