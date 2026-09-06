# Dynamic effects

Starting from `1.10.0`, Audioware exposes a new API to control audio effect for emitter-based sounds _while they play_.

## Usage

```swift
public class MySystem extends ScriptableSystem {
    private let effect: ref<DynamicEffect>;
    private func OnLookAt() -> Void {
        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        let id = target.GetEntityID();
        // create a feedback of +2dB with 20% of blended unprocessed signal
        this.effect = DynamicDelay.Create(2.0, 0.2) as DynamicEffect;
        // add the effect to the emitter settings
        let settings = new EmitterSettings();
        settings.effects = [ this.effect ];
        
        if IsDefined(target)
        // don't forget to register your effect(s) on emitter registration
        && GameInstance.GetAudioSystemExt(game).RegisterEmitter(id, n"PUT_YOUR_MOD_TAG_NAME_HERE", n"None", settings) {
            this.sound = DynamicEmitterEvent.Create(n"PUT_YOUR_SOUND_NAME_HERE", n"PUT_YOUR_MOD_TAG_NAME_HERE");
            // enqueue and play sound
            target.QueueEvent(this.sound);
        }
    }
    // then, at a later point
    public func UpdateFeedback() {
        if IsDefined(this.effect) {
            // e.g. increase feedback by +4dB
            this.effect.SetFeedback(4.0);
        }
    }
    private func OnDetach() -> Void {
        if IsDefined(this.effect) {
            this.effect = null;
        }
        if IsDefined(this.sound) {
            this.sound = null;
        }
    }
}
```

```admonish tip
Note that once a dynamic effect has effectively been registered with an emitter, it cannot be reused with another one.
```
