#[derive(Debug)]
pub enum System {
    Attach,
    Detach,
    PlayerAttach,
    PlayerDetach,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "game system {}",
            match self {
                Self::Attach => "attach",
                Self::Detach => "detach",
                Self::PlayerAttach => "player attach",
                Self::PlayerDetach => "player detach",
            }
        )
    }
}
