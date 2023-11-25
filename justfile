set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load

# default to steam default game dir
DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")

game_dir            := env_var_or_default("GAME_DIR", DEFAULT_GAME_DIR)
plugin_name         := 'audioware'

# codebase (here)
red4ext_repo_dir    := join(justfile_directory(), "target")
redscript_repo_dir  := join(justfile_directory(), "reds")

# game files
red4ext_game_dir    := join(game_dir, "red4ext", "plugins", plugin_name)
redscript_game_dir  := join(game_dir, "r6", "scripts", plugin_name)

setup:
  @if (!(Test-Path '{{red4ext_game_dir}}'))   { [void](New-Item '{{red4ext_game_dir}}'   -ItemType Directory); Write-Host "Created folder at {{red4ext_game_dir}}"; }
  @if (!(Test-Path '{{redscript_game_dir}}')) { [void](New-Item '{{redscript_game_dir}}' -ItemType Directory); Write-Host "Created folder at {{redscript_game_dir}}"; }

# 📦 build Rust RED4Ext plugin
build PROFILE='debug': setup
  @'{{ if PROFILE == "release" { `cargo build --release` } else { `cargo build` } }}'
# Copy-Item -Force '{{red4ext_plugin_name}}' '{{ join(red4ext_plugin_game_dir, plugin_name + ".dll") }}'
# Copy-Item -Force -Recurse '{{ join(red4ext_scripts_dir, "*") }}' '{{red4ext_script_game_dir}}'
# Copy-Item -Force -Recurse '{{ join(repo_dir, "archive", "source", "raw", "addicted", "resources", "audioware.yml") }}' '{{ join(redmod_game_dir, "audioware.yml") }}'

# 🎨 lint code
lint:
  cargo clippy --fix --allow-dirty --allow-staged
  cargo fix --allow-dirty --allow-staged
  cargo fmt
