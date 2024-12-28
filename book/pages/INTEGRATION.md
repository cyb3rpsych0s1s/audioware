# Integration

*How well engine matches our expectations when it comes to integrate seamlessly* with Cyberpunk 2077.

Let's review what you get for free.

## 🔉 Game volume settings

We've already seen that custom audio are affected by player's [game volume settings](./ROUTING.md#volume-settings).

## ⏯️ Dynamic pause/resume when in menus

All audios will be properly paused when entering any menu,  
and resumed when back in-game.

```admonish warning title="Resuming fading-out sounds"
Because the `resume` function in the engine simply resume any non-stopped sound,  
sound(s) that were currently stopping with a fading-out will be *entirely* resumed,  
not resuming their fading out as you could expect.

Providing this feature out-of-the-box requires keeping track of more state,
so it is not currently implemented.
```

```admonish idea title="But you can still do it anyway"
Remember that nothing prevents you to use [AudioSystem](./AUDIO_SYSTEM.md) / [AudioSystemExt](./AUDIO_SYSTEM_EXT.md) *while* in menu, so you can work around this limitation and implement your own logic there.
```

## 🏊‍♂️ Dynamic underwater preset

As previously stated, audio will dynamically have its frequencies adjusted by [Underwater preset](./PARAMETERS.md#preset) whenever V enter or exit water, for the tracks where it makes sense.

This is currently not implemented for cars.

## 🏊‍♂️ Dynamic time dilation

Since `1.3.0`, audio will dynamically have its pitch adjusted whenever time dilation changes (e.g. when using Sandevistan).

You can also opt-out on a per-sound basis.

## 🧹 Clean game sessions

Spatial scene along with its emitters, every track and currently playing sounds will be completely stopped and reset on every save load.

You don't need to worry about memory leaks.
