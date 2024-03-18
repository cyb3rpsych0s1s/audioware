set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load

SPECIFIC_TARGET := 'nightly-2024-01-10'

# default to steam default game dir
DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")

game_dir            := env_var_or_default("GAME_DIR", DEFAULT_GAME_DIR)
plugin_name         := 'audioware'

# codebase (here)
red4ext_bin_dir     := join(justfile_directory(), "target")
redscript_repo_dir  := join(justfile_directory(), "audioware", "reds")

# game files
red4ext_deploy_dir    := join("red4ext", "plugins", plugin_name)
redscript_deploy_dir  := join("r6", "scripts", capitalize(plugin_name))
red_cache_dir         := join("r6", "cache")

# cli
zoltan_exe            := env_var_or_default("ZOLTAN_EXE", "")

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
build PROFILE='debug' TO=game_dir: (setup join(TO, red4ext_deploy_dir))
  @'{{ if PROFILE == "release" { `cargo +nightly-2024-01-10 build --release` } else { `cargo +nightly-2024-01-10 build` } }}'
  @just copy '{{ join(red4ext_bin_dir, PROFILE, plugin_name + ".dll") }}' '{{ join(TO, red4ext_deploy_dir, plugin_name + ".dll") }}'
  @just now

alias b := build

dev: (build) reload

ci TO: (setup join(TO, red4ext_deploy_dir)) (setup join(TO, redscript_deploy_dir)) (build 'release' TO) (reload TO)

optimize TO:
    upx --best --lzma '{{ join(TO, red4ext_deploy_dir, plugin_name + ".dll") }}'

clear:
    @if(Test-Path "{{ join(red_cache_dir, 'final.redscripts.bk') }}" ) { \
        Write-Host "replacing {{ join(red_cache_dir, 'final.redscripts.bk') }} with {{ join(red_cache_dir, 'final.redscripts.bk') }}"; \
        cp -Force '{{ join(red_cache_dir, "final.redscripts.bk") }}' '{{ join(red_cache_dir, "final.redscripts") }}'; \
        Remove-Item -Force -Path '{{ join(red_cache_dir, "final.redscripts.bk") }}'; \
    } else { \
        Write-Host "missing {{ join(red_cache_dir, 'final.redscripts.bk') }}"; \
    }

reload TO=game_dir: (setup join(TO, redscript_deploy_dir))
  @just copy-recurse '{{ join(redscript_repo_dir, "*") }}' '{{ join(TO, redscript_deploy_dir) }}'
  @just now

alias r := reload

uninstall FROM=game_dir:
  just delete '{{ join(FROM, red4ext_deploy_dir) }}'
  just delete '{{ join(FROM, redscript_deploy_dir) }}'

# 🎨 lint code
format:
  @cargo +{{SPECIFIC_TARGET}} fmt

# 🎨 lint code
@lint:
  cargo +{{SPECIFIC_TARGET}} clippy --fix --allow-dirty --allow-staged
  cargo +{{SPECIFIC_TARGET}} fix --allow-dirty --allow-staged
  just format

alias l := lint

qa:
  @cargo +{{SPECIFIC_TARGET}} clippy -- -D warnings
  @cargo +{{SPECIFIC_TARGET}} fix
  @cargo +{{SPECIFIC_TARGET}} fmt --check

test:
  @cargo +{{SPECIFIC_TARGET}} test

alias t := test

check:
  @cargo +{{SPECIFIC_TARGET}} check --all

alias c := check

@doc:
  cargo +{{SPECIFIC_TARGET}} doc --open --no-deps

# TODO: finish updating all patterns
offsets:
  {{zoltan_exe}} '.\addresses.hpp' '{{ join(game_dir, "bin", "x64", "Cyberpunk2077.exe") }}' -f 'std=c++23' --rust-output '.\addresses.rs'
