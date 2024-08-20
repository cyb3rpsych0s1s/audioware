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

~~~admonish hint collapsible=true, title='Alternate builder shorter syntax <span style="color: hotpink; font-size: 0.75em">click to open</span>'
The `AudioSettingsExtBuilder` also accepts a shorter syntax:
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
~~~

```admonish youtube title="YouTube demo"
<iframe width="100%" height="420" src="https://www.youtube.com/embed/eE5jRxl8HAY?si=FJTsTnAUuWfqmMrC" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```
