# Sections

Manifest can contain any of the following sections.

Sections provide "good defaults", a way to classify your audio assets, and even more.

## SFX

`sfx` section is meant to be define simple sounds.

- by default, all audios defined there are loaded `in-memory`.
- volume setting: `SfxVolume`.

## Onos

`onos` (*onomatopeia*) section is meant to define audio with 2 files each, one per gender.

Defaults:
- by default, all audios defined there are loaded `in-memory`.
- volume setting: `DialogueVolume`.

```yml
my_custom_ono:
    fem: ./somewhere/sfx.wav
    male: ./somewhere/else/sfx.wav
```

```admonish hint
Useful for audio that do not require any subtitle, but still have a notion of gender.

e.g. goons grunts and other onos.
```

## Voices

`voices` (sometimes called *voiceovers*) section is meant to define audio with multiple files each and optional subtitles.

- by default, all audios defined there are loaded `on-demand`.
- volume setting: `DialogueVolume`.

```admonish hint
Useful for audio that have to be translated into multiple languages, with or without any notion of gender and optional subtitle.
```

### Simple Voice

```yml
my_simple_voice:
  en-us: ./some/voice.wav
```

```admonish hint
e.g. a vending machine promotional speech
```

#### With subtitle

```yml
my_simple_voice:
  en-us:
    file: ./some/voice.wav
    subtitle: "hello world"
```

```admonish hint
e.g. NPC dialogues or chatters.
```

```admonish tip
Defining subtitle will *automatically* register them with [Codeware Localization](https://github.com/psiberx/cp2077-codeware/wiki#localization) and play them alongside audio, for the proper gender and locale(s).
```

### Plural Voice

Useful for dialogues that are both locale-based and gender-based, with subtitle.

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

```admonish hint
e.g. V's dialogues.
```

## Music

`music` defines songs and ambience music.

- by default, all audios defined there are loaded via `streaming`.
- volume setting: `MusicVolume`.

```yml
version: 1.0.0
music:
  gorillaz_feel_good_inc: ./feel-good-inc.mp3
```