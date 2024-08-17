fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-env-changed=OUT_DIR");
    let version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("cargo package version");
    let red = format!("pub const AUDIOWARE_VERSION: ::red4ext_rs::SemVer = ::red4ext_rs::SemVer::new({}, {}, {});", version.major, version.minor, version.patch);
    let out = std::env::var("OUT_DIR").expect("out dir");
    let out = std::path::PathBuf::from(out).join("version.rs");
    std::fs::write(&out, red).expect("write version.rs to out dir");
    println!("cargo:warning={}", format!("plugin version written to: {}", out.display()));
}