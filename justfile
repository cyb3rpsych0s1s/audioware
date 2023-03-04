set dotenv-load

# default to steam default game dir
DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")

# codebase (here)
RED4EXT_IN_DIR := join("target", "release")
REDSCRIPT_IN_DIR := "reds"
FMOD_IN_DIR := join("vendor", "fmod")

# game files
RED4EXT_OUT_DIR := join("red4ext", "plugins")
REDSCRIPT_OUT_DIR := join("r6", "scripts")

# 📦 build Rust RED4Ext plugin
build:
  cargo build --release

# 📦 bundle mod files (for release in CI)
bundle: build
  mkdir -p '{{ join(".", RED4EXT_OUT_DIR) }}'
  mkdir -p '{{ join(".", REDSCRIPT_OUT_DIR) }}'
  cp -R '{{ join(".", REDSCRIPT_IN_DIR) }}'/* '{{ join(".", REDSCRIPT_OUT_DIR) }}'
  cp '{{ join(".", RED4EXT_IN_DIR) }}'/*.dll '{{ join(".", RED4EXT_OUT_DIR, "audioware") }}'
  cp '{{ join(".", FMOD_IN_DIR) }}'/*.dll '{{ join(".", RED4EXT_OUT_DIR, "audioware") }}'
