use std::path::PathBuf;

/// Specify audio memory usage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Usage<K> {
    /// Used with [kira static sounds](https://docs.rs/kira/latest/kira/sound/static_sound/index.html).
    Static(K, PathBuf),
    /// Used with [kira streaming](https://docs.rs/kira/latest/kira/sound/streaming/index.html).
    Streaming(K, PathBuf),
}

impl<K> std::fmt::Display for Usage<K>
where
    K: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Usage::Static(key, path) => write!(
                f,
                "static:{} ({})",
                key,
                path.display().to_string().as_str()
            ),
            Usage::Streaming(key, path) => write!(
                f,
                "streaming:{} ({})",
                key,
                path.display().to_string().as_str()
            ),
        }
    }
}
