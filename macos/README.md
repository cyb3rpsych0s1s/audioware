# Audioware on macOS

The native macOS build of Cyberpunk 2077 (2.3.1, Apple Silicon). Everything Mac-specific is under `cfg(target_os = "macos")`
or in this folder; the Windows build is unchanged.

## Build

Needs Rust stable for `aarch64-apple-darwin`, CMake and the Xcode Command Line Tools (bindgen uses their libclang).

Audioware builds on red4ext-rs with macOS support (jac3km4/red4ext-rs#99). Its `build.rs` builds RED4ext.SDK with CMake and
needs a macOS copy of the SDK:

```sh
git clone -b mac-rs https://github.com/codeman101/RED4ext.SDK-macos
RED4EXT_SDK_DIR=/path/to/RED4ext.SDK-macos cargo build --release -p audioware
```

The result is `target/release/libaudioware.dylib`.

## Install

Next to the game's `Cyberpunk2077.app`:

- `red4ext/plugins/audioware/audioware.dylib` (the dylib above, renamed)
- `red4ext/plugins/audioware/Scripts/` (the files of `crates/audioware/reds`, without the Debug ones, as in the Windows release)
- `red4ext/plugins/audioware/mac-hooks.txt` (optional, see below)

Mods keep their usual folders (`r6/audioware/<mod>/`); Audioware finds the game folder above the `.app` bundle.

## Loading

The plugin exports `audioware_registerNatives` and `audioware_start`. The macOS launcher calls the first to register the
natives when scripts initialize and the second right after it to start the audio engine and attach the hooks. Support for
the public macOS RED4ext loader is planned as a follow-up.

## Hook modes

`mac-hooks.txt` holds one word:

- `on` (the default, also when the file is missing): the real hooks.
- `off`: no hook is attached. The plugin loads, registers its natives and plays sounds through its script API.
- `probe`: every hook is attached with a pass-through detour that logs its first calls and a running count.

`off` and `probe` are for checking a new game version. The hook targets are listed in `hook-offsets.txt`; each is a
function start in the Mac binary whose body was compared with its Windows counterpart.
