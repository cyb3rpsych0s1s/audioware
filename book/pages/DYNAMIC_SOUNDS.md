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

## Going further

Coupled with [Native integration], here's how you can for example change the audio when the player is chased by NCPD, with 2 different instrumentals based on the wanted level and slight volume variations based on context.

You can find the example in `ChangeCombatMusic.reds`.

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/PH53zc11r6c?si=w0-nb_IWAnkl2Uvq" title="Add heat-stage based music" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```
