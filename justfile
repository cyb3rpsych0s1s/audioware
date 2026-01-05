set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set dotenv-load

# default to steam default game dir
DEFAULT_GAME_DIR := join("C:\\", "Program Files (x86)", "Steam", "steamapps", "common", "Cyberpunk 2077")

game_dir            := env("GAME_DIR", DEFAULT_GAME_DIR)
plugin_name         := 'audioware'

# codebase (here)
red4ext_bin_dir     := join(justfile_directory(), "target")
redscript_repo_dir  := join(justfile_directory(), "crates", "audioware", "reds")
rustdoc_target_dir  := join(justfile_directory(), "book", "pages", "docs")

# game files
red4ext_deploy_dir    := join("red4ext", "plugins", plugin_name)
redscript_deploy_dir  := join("r6", "scripts", capitalize(plugin_name))
red_cache_dir         := join("r6", "cache")
red4ext_logs_dir      := join("red4ext", "logs")

# cli
zoltan_exe            := env("ZOLTAN_EXE", "")
wwiser_exe            := env("WWISER_EXE", "")

[private]
setup path:
  @if (!(Test-Path '{{path}}')) { [void](New-Item '{{path}}' -ItemType Directory); Write-Host "Created folder at {{path}}"; }

[private]
delete path:
  @if (Test-Path '{{path}}') { [void](Remove-Item -Force -Recurse '{{path}}'); Write-Host "Deleted folder at {{path}}"; }

[private]
no-debug path:
  @if (Test-Path '{{path}}') { Get-ChildItem -Path '{{path}}' -Recurse -Filter 'Debug*.reds' | Remove-Item -Force; } else { Write-Host "Unknown folder: {{path}}"; exit 1 }
  @just delete '{{ join(path, redscript_deploy_dir, "Debug") }}'

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
build PROFILE='debug' FEATURES='' TO=game_dir: (setup join(TO, red4ext_deploy_dir))
  @if    ('{{PROFILE}}' -eq "debug")   { cargo build --features='{{FEATURES}}' } \
  elseif ('{{PROFILE}}' -eq "staging") { cargo build --profile staging --features='{{FEATURES}}' } \
  elseif ('{{PROFILE}}' -eq "release") { cargo build --release --features='{{FEATURES}}' } \
  else                                 { Write-Host "unknown profile: '{{PROFILE}}'"; exit 1 }
  @just copy '{{ join(red4ext_bin_dir, PROFILE, plugin_name + ".dll") }}' '{{ join(TO, red4ext_deploy_dir, plugin_name + ".dll") }}'
  @just now

alias b := build

dev FEATURES='hot-reload,research,redengine': (build 'debug' FEATURES) reload

lldb PROFILE='debug' FEATURES='hot-reload' TO=game_dir: (build PROFILE FEATURES) reload
  @just copy '{{ join(red4ext_bin_dir, PROFILE, plugin_name + ".pdb") }}' '{{ join(TO, red4ext_deploy_dir, plugin_name + ".pdb") }}'
  @just now

staging TO=game_dir: (lldb 'staging' '')
  @just no-debug '{{TO}}'; Write-Host "Removed debug files";
  @just now

ci TO PROFILE='release' FEATURES='': (setup join(TO, red4ext_deploy_dir)) (setup join(TO, redscript_deploy_dir)) (build PROFILE FEATURES TO) (reload TO)
  @if('{{PROFILE}}' -ieq 'release') { just no-debug '{{TO}}'; Write-Host "Removed debug files"; } else { Write-Host "Kept debug files untouched"; }
  @if('{{PROFILE}}' -ieq 'debug')   { just copy '{{ join(red4ext_bin_dir, "debug", plugin_name + ".pdb") }}' '{{ join(TO, red4ext_deploy_dir, plugin_name + ".pdb") }}'; }
  @just now

optimize TO:
    upx --best --lzma '{{ join(TO, red4ext_deploy_dir, plugin_name + ".dll") }}'

# FIXME:
# clear TO=game_dir:
#     @if(Test-Path "{{ join(TO, red_cache_dir, 'final.redscripts') }}" ) { \
#         Write-Host "replacing {{ join(TO, red_cache_dir, 'final.redscripts.modded') }} with {{ join(TO, red_cache_dir, 'final.redscripts') }}"; \
#         cp -Force '{{ join(TO, red_cache_dir, "final.redscripts") }}' '{{ join(TO, red_cache_dir, "final.redscripts.modded") }}'; \
#         Remove-Item -Force -Path '{{ join(TO, red_cache_dir, "final.redscripts") }}'; \
#     } else { \
#         Write-Host "missing {{ join(TO, red_cache_dir, 'final.redscripts') }}"; \
#     }

reload TO=game_dir: (setup join(TO, redscript_deploy_dir))
  @just copy-recurse '{{ join(redscript_repo_dir, "*") }}' '{{ join(TO, redscript_deploy_dir) }}'
  @just now

alias r := reload

uninstall FROM=game_dir:
  just delete '{{ join(FROM, red4ext_deploy_dir) }}'
  just delete '{{ join(FROM, redscript_deploy_dir) }}'

# 🎨 lint code
format:
  @cargo fmt --all

# 🎨 lint code
@lint:
  cargo clippy --fix --allow-dirty --allow-staged --all-features --workspace
  cargo fix --allow-dirty --allow-staged --all-features --workspace
  just format

alias l := lint

qa:
  @cargo clippy -- -D warnings
  @cargo fix
  @cargo fmt --check

test:
  cargo test --workspace --exclude audioware --exclude audioware-bank

alias t := test

check PROFILE='':
  @cargo check --release
  @cargo check --features='hot-reload'

alias c := check

@doc:
  cargo doc --open --no-deps

# TODO: finish updating all patterns
offsets:
  {{zoltan_exe}} '.\addresses.hpp' '{{ join(game_dir, "bin", "x64", "Cyberpunk2077.exe") }}' -f 'std=c++23' --rust-output '.\addresses.rs'

checksum TO:
  Get-FileHash -Path "{{TO}}" -Algorithm SHA256 | Select-Object -ExpandProperty Hash

@log TO=game_dir:
  $folder = '{{ join(TO, red4ext_logs_dir) }}'; \
  $files = Get-ChildItem -Path $folder -Filter "audioware-*.log"; \
  $logs = @(); \
  foreach ($file in $files) { \
    if ($file.Name -match "audioware-(\d{4})-(\d{2})-(\d{2})-(\d{2})-(\d{2})-(\d{2})") { \
      $dateString = "$($matches[1])-$($matches[2])-$($matches[3]) $($matches[4]):$($matches[5]):$($matches[6])"; \
      $date = [datetime]::ParseExact($dateString, "yyyy-MM-dd HH:mm:ss", $null); \
      $logs += [PSCustomObject]@{ \
        File = $file.FullName; \
        Date = $date; \
      } \
    } \
  } \
  $latest = $logs | Sort-Object Date -Descending | Select-Object -First 1; \
  if ($latest) { \
    Write-Output "The most recent log file is: $($latest.File)"; \
    Get-Content -Path $latest.File -Wait; \
  } else { \
    Write-Output "No log files found."; \
  }

smash FROM=game_dir:
  {{ join(justfile_directory(), "community", "redscript-cli.exe") }} decompile -f -i '{{ join(FROM, red_cache_dir, "final.redscripts") }}' -o '{{ join(justfile_directory(), "..", "dump_smasher") }}'

@style:
    cd book; mdbook-admonish generate-custom ./theme/css/mdbook-admonish-custom.css

# 📖 read book directly
@read: style preassemble
    cd book; mdbook build --open

# 🖊️  book with live hot reload
@draft: style preassemble
    cd book; mdbook watch --open

# 📕 preassemble book: rustdoc (for release in CI in 2 steps)
@preassemble:
    cargo build; cargo doc --document-private-items --target-dir '{{rustdoc_target_dir}}'
    just delete '{{ join(rustdoc_target_dir, "debug") }}'
    just delete '{{ join(rustdoc_target_dir, "CACHEDIR.TAG") }}'

# 📕 assemble book (for release in CI)
@assemble: style
    cd book; mdbook build; cd ..

@wwiser BNK OPTIONS='-v':
  python {{wwiser_exe}} "{{BNK}}" {{OPTIONS}}
