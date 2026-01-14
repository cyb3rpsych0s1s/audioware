# Dynamic sound events

Starting from `1.7.0`, Audioware exposes a new API to control sounds _while they play_.

## Usage

```swift
public class MySystem extends ScriptableSystem {
    private let sound: ref<DynamicSoundEvent>;
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let player = owner as PlayerPuppet;
        if IsDefined(player)
        && !IsDefined(this.sound) {
            this.sound = DynamicSoundEvent.Create(n"PUT_YOUR_SOUND_NAME_HERE");
            // enqueue and play sound
            player.QueueEvent(this.sound);
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

```admonish tip
Note that once a dynamic sound event has effectively been stopped, it cannot be restarted or further controlled.
```

## Going further

Coupled with [Native integration](./NATIVE_INTEGRATION.md) feature, here's how you can for example change the audio when the player is chased by NCPD based on the wanted level:

- first instrumental slowly fades-in after 2 stars
- then it gets replaced by a more "punchy" variant instrumental with scratches after 3 stars
- plus some slight contextual volume variations (player running, crouching, etc).

You can find the example in `ChangeCombatMusic.reds`.

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/PH53zc11r6c?si=w0-nb_IWAnkl2Uvq" title="Add heat-stage based music" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```
