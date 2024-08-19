# AudioSettingsExt

Earlier we saw that [settings](./SETTINGS.md) can be defined in [manifest](./MANIFEST.md),
but these settings can also be specified in scripts:

```swift
// builder is a mutable ref
let builder: ref<AudioSettingsExtBuilder> = AudioSettingsExtBuilder.Create();
// it supports the same settings as definable in manifest, except start_position
builder.SetFadeInTween(ElasticTween.ImmediateIn(5.0, 0.25));
builder.SetPanning(0.3);
builder.SetPlaybackRate(1.1);
builder.SetVolume(0.9);
// once built it returns an immutable ref with different type
let settings: ref<AudioSettingsExt> = builder.Build();

GameInstance
.GetAudioSystemExt(game)
.Play(n"still_dre", GetPlayer(game).GetEntityID(), n"V", scnDialogLineType.Regular, settings);
```

```admonish hint
The AudioSettingsExtBuilder also accepts a shorter syntax:
```

```swift
GameInstance
    .GetAudioSystemExt(game)
    .Play(
        n"still_dre",
        GetPlayer(game).GetEntityID(),
        n"V",
        scnDialogLineType.Regular, 
        AudioSettingsExtBuilder.Create()
            .WithFadeInTween(ElasticTween.ImmediateIn(5.0, 0.25))
            .WithPanning(0.3)
            .WithPlaybackRate(1.1)
            .WithVolume(0.9)
            .Build()
    );
```
