#[derive(Debug)]
pub enum Board {
    UIMenu(bool),
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UIMenu(value) => write!(f, "ui menu ({value})"),
        }
    }
}
