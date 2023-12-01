set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load

# default to steam default game dir
DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")

game_dir            := env_var_or_default("GAME_DIR", DEFAULT_GAME_DIR)
plugin_name         := 'audioware'

# codebase (here)
red4ext_bin_dir     := join(justfile_directory(), "target")
redscript_repo_dir  := join(justfile_directory(), "audioware", "reds")

# game files
red4ext_deploy_dir    := join(game_dir, "red4ext", "plugins", plugin_name)
redscript_deploy_dir  := join(game_dir, "r6", "scripts", capitalize(plugin_name))

# cli
zoltan_exe            := env_var("ZOLTAN_EXE")

[private]
setup path:
  @if (!(Test-Path '{{path}}')) { [void](New-Item '{{path}}' -ItemType Directory); Write-Host "Created folder at {{path}}"; }

[private]
delete path:
  @if (Test-Path '{{path}}') { [void](Remove-Item -Force -Recurse '{{path}}'); Write-Host "Deleted folder at {{path}}"; }

[private]
copy from to:
  @Copy-Item -Force '{{from}}' '{{to}}'
  @Write-Host "Copied {{from}} -> {{to}}"

[private]
copy-recurse from to:
  @Copy-Item -Force -Recurse '{{from}}' '{{to}}'
  @Write-Host "Copied {{from}} -> {{to}}"

# log current time
[private]
now:
  @Write-Host "$(Get-Date) $_"

# 📦 build Rust RED4Ext plugin
build PROFILE='debug': (setup red4ext_deploy_dir)
  @'{{ if PROFILE == "release" { `cargo build --release` } else { `cargo build` } }}'
  @just copy '{{ join(red4ext_bin_dir, PROFILE, plugin_name + ".dll") }}' '{{ join(red4ext_deploy_dir, plugin_name + ".dll") }}'
  @just now

alias b := build

dev: (build 'debug') reload

reload: (setup redscript_deploy_dir)
  @just copy-recurse '{{ join(redscript_repo_dir, "*") }}' '{{redscript_deploy_dir}}'
  @just now

alias r := reload

install PROFILE='debug': (build PROFILE) reload

alias i := install

uninstall: (delete red4ext_deploy_dir) (delete redscript_deploy_dir)

# 🎨 lint code
lint:
  @cargo clippy --fix --allow-dirty --allow-staged
  @cargo fix --allow-dirty --allow-staged
  @cargo fmt

alias l := lint

# TODO: finish updating all patterns
offsets:
  {{zoltan_exe}} '.\addresses.hpp' '{{ join(game_dir, "bin", "x64", "Cyberpunk2077.exe") }}' -f 'std=c++23' --rust-output '.\addresses.rs'
