# Spatialization

Thanks to [kira][kira] Audioware supports audio spatialization, which means audio *that moves along its emitter*, getting louder when closer and softer when further, along with left-right [panning](https://en.wikipedia.org/wiki/Panning_(audio)).

## Registration

Audio emitter(s) must be registered before you can emit audio from them, but they are automatically cleaned up whenever emitter despawns or dies.
You must provide a `tag_name` which Audioware uses to track emitters internally.

```swift
GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, n"MyMod");
```

```admonish warning title="Types"
Audio emitter have to be positioned so they can only be [Entity](https://nativedb.red4ext.com/Entity) or classes inheriting from it like [GameObject](https://nativedb.red4ext.com/GameObject), devices, vehicles, NPCs, etc.
```

~~~admonish hint title="Cleanup"
You don't need to manually unregister your audio emitter(s), even if you can do so:  
Audioware does it automatically whenever emitter despawns or dies. Dying emitter <span style="color: hotpink">still</span> emit.
~~~

~~~admonish hint title="Dying emitter (1.3.0+)"
You don't need to manually fade out or stop your audio on dying emitter(s), even if you can do so: Audioware does it automatically whenever emitter dies (stop) or gets incapacitated / defeated (fade-out).
~~~

```admonish hint
V cannot be an audio emitter because (s)he is the listener.
```

## Usage

Then, simply use the `OnEmitter` variants of the methods:

```swift
// ⚠️ emitterID and emitterCName must be both valid and non-default
GameInstance.GetAudioSystemExt(game).PlayOnEmitter(n"my_custom_audio", emitterID, n"MyMod");

// if should stop at some point...
GameInstance.GetAudioSystemExt(game).StopOnEmitter(n"my_custom_audio", emitterID, n"MyMod");
```

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/GZWnAjhhFOQ?si=BRe8h-x7A5SZUO2X" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
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
        let tagName: CName = n"MyMod";

        let game = this.GetGameInstance();
        // get entity V currently looks at (crosshair)
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();
        if !GameInstance.GetAudioSystemExt(game).IsRegisteredEmitter(emitterID, tagName) {
            GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, tagName);
        }
        GameInstance.GetAudioSystemExt(game).PlayOnEmitter(eventName, emitterID, tagName);
    }
}
```

```admonish youtube title="YouTube demo"
This particular showcase is a smoke test: applying most CPU-intensive sounds (music streaming) to multiple entities in the vicinity and triggering auto-unregistration.

It aims at demonstrating that both performances stay correct (even on my low-end laptop!),  
and unregistration happens seamlessly (including during simultaneous kills).

<iframe width="100%" height="420" src="https://www.youtube.com/embed/cngYFyFaapo?si=q1I_8g7t5A0d6uGs" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```

## Integration

Since version `1.4.1+` audio emitters are now affected by [reverb mix](./PARAMETERS.md#reverb-mix),
and optionally by [environmental preset](./PARAMETERS.md#preset).

These can be opted-in/out by defining on `EmitterSettings`:

```swift
let settings = new EmitterSettings();
settings.affectedByReverbMix = false;
settings.affectedByEnvironmentalPreset = false;
```

```admonish youtube title="Youtube demo"
<iframe width="100%" height="420" src="https://youtu.be/EDbfk1vfur8" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```

[kira]: https://docs.rs/kira/latest/kira/spatial/index.html "kira spatial scene"
