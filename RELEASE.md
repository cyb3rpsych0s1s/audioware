# release notes

- bump:
  1. `Plugin::VERSION` in *.\audioware\src\lib.rs*
  2. workspace version in *.\Cargo.toml*
- tag accordingly

## after patch update

- SDK: update `red4ext-rs` ➡️ `RED4ext.SDK`
- addresses:
  1. run `just offsets` command to retrieve addresses from hex patterns
  2. update addresses in *.\audioware\src\addresses.rs* with previous results
  3. ⚠️ troubleshooting: double-check in IDA
