# AudioSettingsExt

Earlier we saw that [settings](./SETTINGS.md) can be defined in [manifest](./MANIFEST.md),
but these settings can also be specified in scripts:

```swift
let ext = new AudioSettingsExt();
ext.fadeIn = LinearTween.Immediate(2.0);

GameInstance
.GetAudioSystemExt(game)
.Play(n"still_dre", GetPlayer(game).GetEntityID(), n"V", scnDialogLineType.Regular, settings);
```

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/1JWgtmSyGg8?si=-t9C7K4KkJuySHpW" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```
