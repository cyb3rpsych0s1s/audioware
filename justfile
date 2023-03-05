set dotenv-load

# default to steam default game dir
DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")
DEFAULT_FMOD_LOCAL_DIR := join("C:\\", "Program Files (x86)", "FMOD SoundSystem", "FMOD Studio API Windows")

game_dir := env_var_or_default("GAME_DIR", DEFAULT_GAME_DIR)
fmod_local_dir := env_var_or_default("FMOD_DIR", DEFAULT_FMOD_LOCAL_DIR)

fmod_core_dir := join(fmod_local_dir, "api", "core", "lib", "x64")
fmod_studio_dir := join(fmod_local_dir, "api", "studio", "lib", "x64")

# codebase (here)
red4ext_in_dir := join("target", "release")
redscript_in_dir := "reds"
fmod_in_dir := join("vendor", "fmod")

# game files
red4ext_out_dir := join("red4ext", "plugins")
redscript_out_dir := join("r6", "scripts")

# 📦 vendor FMOD lib
vendor:
  cp '{{ join(fmod_core_dir, "fmod.dll") }}' '{{ join(".", fmod_in_dir, "fmod.dll") }}'
  cp '{{ join(fmod_core_dir, "fmod_vc.lib") }}' '{{ join(".", fmod_in_dir, "fmod.lib") }}'
  cp '{{ join(fmod_studio_dir, "fmodstudio.dll") }}' '{{ join(".", fmod_in_dir, "fmodstudio.dll") }}'
  cp '{{ join(fmod_studio_dir, "fmodstudio_vc.lib") }}' '{{ join(".", fmod_in_dir, "fmodstudio.lib") }}'

# 📦 link FMOD lib for cargo build
link PROFILE='debug':
 cp '{{ join(".", fmod_in_dir, "fmod.dll") }}' '{{ join(".", "target", PROFILE, "deps", "fmod.dll") }}'
 cp '{{ join(".", fmod_in_dir, "fmod.lib") }}' '{{ join(".", "target", PROFILE, "deps", "fmod.lib") }}'
 cp '{{ join(".", fmod_in_dir, "fmodstudio.dll") }}'  '{{ join(".", "target", PROFILE, "deps", "fmodstudio.dll") }}'
 cp '{{ join(".", fmod_in_dir, "fmodstudio.lib") }}'  '{{ join(".", "target", PROFILE, "deps", "fmodstudio.lib") }}'

# 📦 build Rust RED4Ext plugin
build PROFILE='debug':
  @'{{ if PROFILE == "release" { `cargo build --release` } else { `cargo build` } }}'

# 📦 bundle mod files (for release in CI)
bundle: (build "release")
  mkdir -p '{{ join(".", red4ext_out_dir, "audioware") }}'
  mkdir -p '{{ join(".", redscript_out_dir, "audioware") }}'
  cp -R '{{ join(".", redscript_in_dir) }}'/* '{{ join(".", redscript_out_dir, "audioware") }}'/
  cp '{{ join(".", red4ext_in_dir) }}'/*.dll '{{ join(".", red4ext_out_dir, "audioware") }}'/
  cp '{{ join(".", fmod_in_dir, "fmod.dll") }}' '{{ join(".", red4ext_out_dir, "audioware", "fmod.dll") }}'
  cp '{{ join(".", fmod_in_dir, "fmod.lib") }}' '{{ join(".", red4ext_out_dir, "audioware", "fmod.lib") }}'
  cp '{{ join(".", fmod_in_dir, "fmodstudio.dll") }}' '{{ join(".", red4ext_out_dir, "audioware", "fmodstudio.dll") }}'
  cp '{{ join(".", fmod_in_dir, "fmodstudio.lib") }}' '{{ join(".", red4ext_out_dir, "audioware", "fmodstudio.lib") }}'

# 📦 install locally
install: bundle
  cp -R '{{ join(".", red4ext_out_dir) }}'/* '{{ join(game_dir, red4ext_out_dir) }}'
  cp -R '{{ join(".", redscript_out_dir) }}'/* '{{ join(game_dir, redscript_out_dir) }}'
  mkdir -p '{{ join(".", redscript_out_dir, "fakemod") }}'
  cp -R '{{ join(".", "fakemod") }}'/* '{{ join(game_dir, redscript_out_dir, "fakemod") }}'

# 🗑️  clear out plugin files in game files
uninstall:
  rm -rf '{{ join(game_dir, red4ext_out_dir, "audioware") }}'
  rm -rf '{{ join(game_dir, redscript_out_dir, "audioware") }}'
  rm -rf '{{ join(game_dir, redscript_out_dir, "fakemod") }}'
