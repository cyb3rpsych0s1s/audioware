# Sections

Manifest can contain any of the following sections.

Sections provide "good defaults", a way to classify your audio assets, and even more.

## SFX

`sfx` section is meant to define simple sounds.

```yml
my_custom_sfx: ./somewhere/sfx.ogg
```

| Defaults        |                  | Editable? |
|-----------------|------------------|-----------|
| usage           | `in-memory`      |✅         |
| volume settings | `SfxVolume`      |❌         |

## Onos

`onos` (*onomatopeia*) section is meant to define audio with 2 files each, one per gender.

```yml
my_custom_ono:
    fem: ./somewhere/ono.wav
    male: ./somewhere/else/ono.wav
```

| Defaults        |                  | Editable? |
|-----------------|------------------|-----------|
| usage           | `in-memory`      |✅         |
| volume settings | `DialogueVolume` |❌         |

```admonish info
Useful for audio that do not require any subtitle, but still have a notion of gender.

> e.g. goons grunts and other onos.
```

## Voices

`voices` (sometimes called *voiceovers*) section is meant to define audio with multiple files each and optional subtitles.

| Defaults        |                  | Editable? |
|-----------------|------------------|-----------|
| usage           | `on-demand`      |✅         |
| volume settings | `DialogueVolume` |❌         |

### Simple Voice

```yml
my_simple_voice:
  en-us: ./some/voice.wav
```

```admonish info
Useful for audio that have to be translated into multiple languages, but the notion of gender does not matter.

> e.g. a vending machine promotional speech
```

#### Simple Voice with subtitle

```yml
my_simple_voice:
  en-us:
    file: ./some/voice.wav
    subtitle: "hello world"
```

```admonish info
Useful to add simple subtitle to be played along your audio.

> e.g. NPC dialogues or chatters.
```

```admonish tip
Defining subtitle will *automatically* register them with [Codeware Localization](https://github.com/psiberx/cp2077-codeware/wiki#localization) and play them alongside audio, for the proper gender and locale(s).
```

### Plural Voice

```yml
version: 1.0.0
voices:
  my_plural_voice:
    en-us:
      fem: ./fem_intro.mp3
      male: ./male_intro.mp3
      subtitle: "Let me introduce myself, I'm V."
  my_other_plural_voice:
    en-us:
      fem:
        file: ./fem_wake_up.mp3
        subtitle: "Looks yourself in the mirror, girl."
      male:
        file: ./male_wake_up.mp3
        subtitle: "Look yourself in the mirror, dude."
```

```admonish info
Useful for dialogues that are both locale-based *and* gender-based, with subtitle.

> e.g. V's dialogues.
```

## Music

`music` defines songs and ambience music.

```yml
version: 1.0.0
music:
  gorillaz_feel_good_inc: ./feel-good-inc.mp3
```

| Defaults        |               | Editable? |
|-----------------|---------------|-----------|
| usage           | `streaming`   |✅         |
| volume settings | `MusicVolume` |❌         |
