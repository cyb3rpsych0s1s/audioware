fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-env-changed=OUT_DIR");
    let version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("cargo package version");
    let mut pre: u32 = 3;
    let mut build: u32 = 0;
    if version.pre.starts_with("alpha") {
        pre = 0;
    } else if version.pre.starts_with("beta") {
        pre = 1;
    } else if version.pre.starts_with("rc") {
        pre = 2;
    }
    if version.pre.contains('.') {
        let suffix = version.pre.split('.').collect::<Vec<_>>();
        build = suffix
            .get(1)
            .unwrap()
            .parse()
            .expect("unable to read plugin build version");
    }
    let red = format!("/// auto-generated plugin version\npub const AUDIOWARE_VERSION: ::red4ext_rs::SemVer = ::red4ext_rs::SemVer::exact({}, {}, {}, {}, {});", version.major, version.minor, version.patch, pre, build);
    let out = std::env::var("OUT_DIR").expect("out dir");
    let out = std::path::PathBuf::from(out).join("version.rs");
    std::fs::write(&out, red).expect("write version.rs to out dir");
    println!(
        "cargo:warning={}",
        format_args!("plugin version {} written to: {}", version, out.display())
    );
}
