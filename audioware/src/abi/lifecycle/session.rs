#[derive(Debug)]
pub enum Session {
    BeforeStart,
    Start,
    Ready,
    Pause,
    Resume,
    BeforeEnd,
    End,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "game session {}",
            match self {
                Self::BeforeStart => "before start",
                Self::Start => "start",
                Self::Ready => "ready",
                Self::Pause => "pause",
                Self::Resume => "resume",
                Self::BeforeEnd => "before end",
                Self::End => "end",
            }
        )
    }
}
