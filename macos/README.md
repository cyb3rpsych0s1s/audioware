# Audioware on macOS

The native macOS build of Cyberpunk 2077 (2.5, Apple Silicon). Everything Mac-specific is under `cfg(target_os = "macos")`
or in this folder; the Windows build is unchanged.

## Requirements

- An Apple Silicon Mac with the native macOS version of Cyberpunk 2077 2.5 (Steam).
- Rust stable with the `aarch64-apple-darwin` target, CMake and the Xcode Command Line Tools (bindgen uses their libclang).
- The macOS builds of RED4ext, Codeware, ArchiveXL and TweakXL in the game folder. These macOS ports are not upstream yet.

## Build

Audioware builds on red4ext-rs with macOS support (jac3km4/red4ext-rs#99; `Cargo.toml` points at that PR's commit). Its
`build.rs` builds RED4ext.SDK with CMake and needs the macOS copy of the SDK for game 2.5:

```sh
git clone -b mac-rs-2.5 https://github.com/codeman101/RED4ext.SDK-macos
git clone -b macos https://github.com/codeman101/audioware
cd audioware
RED4EXT_SDK_DIR=/path/to/RED4ext.SDK-macos cargo build --release -p audioware
```

The result is `target/release/libaudioware.dylib`.

## Install

1. Open the game folder: in Steam, right-click Cyberpunk 2077, Manage, Browse local files. `Cyberpunk2077.app` is in it.
2. Create `red4ext/plugins/audioware/` there if it does not exist.
3. Copy `target/release/libaudioware.dylib` to `red4ext/plugins/audioware/audioware.dylib` (renamed).
4. Copy the `.reds` files of `crates/audioware/reds` to `red4ext/plugins/audioware/Scripts/`, without the Debug ones
   (`Debug.reds` and the `Debug/` folder), as in the Windows release.
5. Optional: create `red4ext/plugins/audioware/mac-hooks.txt` with one word, the hook mode (see below). Without the file the
   hooks are on.
6. Audioware mods go in their usual folder, `r6/audioware/<mod>/`. Audioware finds the game folder above the `.app` bundle.

To remove it, delete `red4ext/plugins/audioware/`.

## Loading

The plugin exports `audioware_registerNatives` and `audioware_start`. The launcher calls the first to register the natives
when scripts initialize, and the second right after it to start the audio engine and attach the hooks. Today only the
macOS launcher used to develop the port makes these calls; support for the public macOS RED4ext loader is planned as a
follow-up.

## Hook modes

`mac-hooks.txt` holds one word:

- `on` (the default, also when the file is missing): the real hooks.
- `off`: no hook is attached. The plugin loads, registers its natives and plays sounds through its script API.
- `probe`: every hook is attached with a pass-through detour that logs its first calls and a running count.

`off` and `probe` are for checking a new game version. The hook targets are listed in `hook-offsets.txt`; each is a
function start in the Mac binary whose body was compared with its Windows counterpart.

## Tested

Game 2.5 (Mac build 5814786) with the hooks on: the natives register, a bank loads and `Duration()` reads its sounds,
`AudioSystemExt.Play` and `Stop` are accepted, and a dynamic sound plays, reports its position and stops. The plugin log
has no errors.
