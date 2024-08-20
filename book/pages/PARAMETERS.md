# Parameters

Audioware exposes the following parameters.

## Reverb Mix

Allows to alter reverb like e.g. when in a cavern.

```swift
let value: Float; // reverb can be between 0.0 and 1.0 (inclusive)
GameInstance.GetBlackboardSystem(game)
    .Get(GetAllBlackboardDefs().Audioware_Settings)
    .SetFloat(GetAllBlackboardDefs().Audioware_Settings.ReverbMix, value, true);
```

```admonish info title="Routing"
This preset only affects audio played on `sfx`, `onos` and `voices`.
```

```admonish warning title="Important"
Keep it mind that forgetting to reset reverb to normal once finished will annoy players.

For this very reason, reverb is <span style="color: #f3d772">automatically reset</span> on each save load.
```

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/oxZ1hjodSDA?si=mxYtDAWgBKa9sZa8" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```

## Preset

Allows to alter frequencies like e.g. underwater or on the phone.

```admonish info title="Integration"
Whenever V dives into water this preset will be automatically set to `Preset.Underwater`, and switched back to `Preset.None` whenever V eventually reaches the surface.
```

You can also set it manually, if needed:

```swift
let value: Preset; // possible values: None, Underwater, OnThePhone
GameInstance.GetBlackboardSystem(game)
    .Get(GetAllBlackboardDefs().Audioware_Settings)
    .SetInt(GetAllBlackboardDefs().Audioware_Settings.AudioPreset, value, true);
```

```admonish info title="Routing"
This preset only affects audio played on `sfx`, `onos` and `voices`.
```

```admonish warning title="Important"
Forgetting to reset preset to `Preset.None` once finished will ruin players immersion.

For this very reason, preset is <span style="color: #f3d772">automatically reset</span> on each save load.
```

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/CPQ2s2q-TCc?si=Lvh8kH6FFcc55wQ3" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```
