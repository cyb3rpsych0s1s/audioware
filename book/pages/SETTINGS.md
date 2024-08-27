# Settings

Any sound will accept the following settings.

## 🗃️ Usage

This allows to specify how audio will be handled in memory.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  usage: on-demand
```

Each section already has its own default usage when left unspecified, see [Sections](./SECTIONS.md).

You can choose between possible values: `on-demand`, `in-memory` and `streaming`.

```admonish bulb title="Extra flexibility"
This gives you extra flexibility if e.g. you want to play a song which is traditionally defined in [music](./SECTIONS.md#music) as a [ono](./SECTIONS.md#onos) instead (which by default is loaded `in-memory`) while still being able to play it with `streaming`.
```

### in-memory

The audio is loaded *all-at-once* in-memory on game startup and kept around for the *whole duration* of the game session.

```admonish info title="Best used for"
<span style="color: #f3d772">Short</span> sounds that are meant to be played <span style="color: #f3d772">frequently</span>.
```

### on-demand

The audio is loaded *all-at-once* each time on-demand, and never kept around.
  
```admonish info title="Best used for"
<span style="color: #f3d772">Short</span> sounds that *you don't want to permanently allocate memory for*,  
or that are <span style="color: #f3d772">not</span> meant to be played <span style="color: #f3d772">frequently</span>.
```

### streaming

The audio is streamed on-demand.
  
```admonish info title="Best used for"
<span style="color: #f3d772">Long-lasting</span> sounds that should not be loaded all-at-once in-memory and only <span style="color: #f3d772">streamed</span> *on-demand*.
```

## 🔉 Volume

You can set `Volume` factor as follow:

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    volume: 2.0 # 2 times louder !
my_other_audio:
  file: ./somewhere/else/audio.ogg
  settings:
    volume: 0.5 # 2 times softer
```

```admonish info
No matter how high `Volume` is set, it will not play louder than 85dB.
```

## 🕟 Start time

This will play your audio with a delay.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    start_time: 10s # 10 seconds delay
```

## 🔚 Start position

This will play your audio further from start.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    start_position: 1s # start playing directly at 1s
```

```admonish warning
Note that digits with decimal(s) are not supported, so if you would like to start the audio at e.g. `1.2s`, please specify `120ms` instead.
```

## 🍕 Region

This will only play the specified region of audio.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    region:
      starts: 120ms # starts directly from 1.2s
      ends: 8s # ends at 8s
```

```admonish hint
You're not required to specify both `starts` and `ends`.

If left unspecified:

- `starts` will start at the beginning of the audio.
- `ends` will play until the end of the audio.

> Also kindly note that entire piece of audio still need to be loaded, in terms of memory, regardless its region.
```

## 🔁 Loop

This will loop audio until explicitly stopped.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    loop: true
```

## ⏩ Playback rate

This will play your audio faster, or slower.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    playback_rate: x0.5 # plays twice as slow
```

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    playback_rate: x2 # plays twice as fast
```

You can also specify the value in semitones as follow.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    playback_rate: 2♯ # adjusts by 2 semitones
```

## ↔️ Panning

This adjust from where the audio originates from, from left to right.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    panning: 0.0 # plays fully on left side
```

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    panning: 1.0 # plays fully on right side
```

```admonish warning
The value **must** be between `0.0` and `1.0` (inclusive).
```

## ⤴️ Fade-in tween

This will play your audio gradually fading-in.

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    fade_in_tween:
      start_time: 1s # starts playing directly from 1s
      duration: 3s # fade-in duration
      Linear: # linear fade-in curve (no value needed)
```

```yml
my_custom_audio:
  file: ./somewhere/audio.wav
  settings:
    fade_in_tween:
      start_time: 1s # starts playing directly from 1s
      duration: 3s # fade-in duration
      InPowi: 3 # easing-in with power 3
```

Possible values for `easing` can be found [here](https://docs.rs/kira/latest/kira/tween/enum.Easing.html).

```admonish hint
Note that fade-out can be specified as a parameter when calling methods like `Stop`, `Switch`, etc. see [AudioSettingsExt](./AUDIO_SETTINGS_EXT.md).
```
