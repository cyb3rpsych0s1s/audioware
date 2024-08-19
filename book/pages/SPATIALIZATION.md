# Spatialization

Audioware supports audio spatialization, which means audio *that moves around along its emitter*, getting louder when closer and softer when further, along with left-right [panning](https://en.wikipedia.org/wiki/Panning_(audio)).

## Registration

Audio emitter(s) must be registered before you can emit audio from them, but they are automatically cleaned up whenever emitter despawns or dies.

```swift
if !GameInstance.GetAudioSystemExt(game).IsRegisteredEmitter(emitterID) {
    GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID);
}
```

```admonish warning title="Types"
Audio emitter have to be positioned so they can only be [Entity](https://nativedb.red4ext.com/Entity) or classes inheriting from it like [GameObject](https://nativedb.red4ext.com/GameObject), devices, vehicles, NPCs, etc.
```

```admonish hint title="Cleanup"
You don't need to manually unregister your audio emitter(s), even if you can do so: Audioware does it automatically whenever emitter despawns or dies. Dying emitter still emit.
```

```admonish hint
V cannot be an audio emitter because (s)he is the listener.
```

## Usage

Then, simply use the `OnEmitter` variants of the methods:

```swift
// ⚠️ emitterID and emitterCName must be both valid and non-default
GameInstance.GetAudioSystemExt(game).PlayOnEmitter(n"my_custom_audio", emitterID, emitterCName);

// if should stop at some point...
GameInstance.GetAudioSystemExt(game).StopOnEmitter(n"my_custom_audio", emitterID, emitterCName);
```

## Auto-registration

Whenever you want to turn all particular entities of a kind into audio emitters automatically, you can reach out for [Codeware game events](https://github.com/psiberx/cp2077-codeware/wiki#game-events).

Here's a dummy example on how to use `F1` to register any audio emitter in crosshair.

```swift
import Audioware.*

public class AutoEmittersSystem extends ScriptableSystem {
    private func OnAttach() {
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnKeyInput")
        // listen to F1 being pressed or released
        .AddTarget(InputTarget.Key(EInputKey.IK_F1));
    }
    private cb func OnKeyInput(evt: ref<KeyInputEvent>) {
        // when F1 is released
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        // some songs defined in manifest
        let sounds = [ 
            n"my_custom_song_01",
            n"my_custom_song_02", 
            n"my_custom_song_03",
            n"my_custom_song_04", 
            n"my_custom_song_05"
        ];
        // get a random sound above
        let eventName = sounds[RandRange(0, ArraySize(sounds) -1)];
        // prepare some settings
        let tween = new LinearTween();
        tween.startTime = RandRangeF(1.0, 3.0);
        tween.duration = RandRangeF(3.0, 4.5);
        let emitterID: EntityID;
        let emitterCName: CName = n"DummyTest";

        let game = this.GetGameInstance();
        // get entity V currently looks at (crosshair)
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();
        if !GameInstance.GetAudioSystemExt(game).IsRegisteredEmitter(emitterID) {
            GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID);
        }
        GameInstance.GetAudioSystemExt(game).PlayOnEmitter(eventName, emitterID, emitterCName);
    }
}
```

```admonish example title="YouTube demo"
[![YouTube demo](https://img.youtube.com/vi/cngYFyFaapo/0.jpg)](https://www.youtube.com/watch?v=cngYFyFaapo&list=PLMu2na7a3T6MHJq_JJ6yx_2qRv4MYX9ez&index=2)
```
