# Dynamic emitter events

Starting from `1.7.6`, Audioware exposes a new API to control spatial emitter sounds _while they play_.

## Usage

```swift
public class MySystem extends ScriptableSystem {
    private let sound: ref<DynamicEmitterEvent>;
    private func OnLookAt() -> Void {
        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        let id = target.GetEntityID();
        
        if IsDefined(target)
        && !IsDefined(this.sound)
        // don't forget to register your emitter first
        && GameInstance.GetAudioSystemExt(game).RegisterEmitter(id, n"PUT_YOUR_MOD_TAG_NAME_HERE") {
            this.sound = DynamicEmitterEvent.Create(n"PUT_YOUR_SOUND_NAME_HERE", n"PUT_YOUR_MOD_TAG_NAME_HERE");
            // enqueue and play sound
            target.QueueEvent(this.sound);
        }
    }
    // then, at a later point
    public func UpdateVolume() {
        if IsDefined(this.sound) {
            // e.g. increase volume
            this.sound.SetVolume(1.2);
        }
    }
    private func OnDetach() -> Void {
        if IsDefined(this.sound) {
            this.sound = null;
        }
    }
}
```
