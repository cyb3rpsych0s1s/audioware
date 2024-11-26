use crate::engine::eq::Preset;

#[derive(Debug)]
pub enum Board {
    UIMenu(bool),
    ReverbMix(f32),
    Preset(Preset),
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UIMenu(value) => write!(f, "ui menu ({value})"),
            Self::ReverbMix(value) => write!(f, "reverb mix ({value})"),
            Self::Preset(value) => write!(f, "preset ({value})"),
        }
    }
}
