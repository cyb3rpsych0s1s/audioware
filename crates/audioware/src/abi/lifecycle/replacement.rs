use crate::{EventHookTypes, EventName};

#[derive(Debug)]
pub enum ReplacementNotification {
    Mute(EventName),
    MuteSpecific(EventName, EventHookTypes),
    Unmute(EventName),
    UnmuteSpecific(EventName, EventHookTypes),
}

impl std::fmt::Display for ReplacementNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "replacement {}",
            match self {
                Self::Mute(event_name) => format!("mute: {}", event_name),
                Self::MuteSpecific(event_name, event_type) =>
                    format!("mute: {}, {}", event_name, event_type),
                Self::Unmute(event_name) => format!("unmute: {}", event_name),
                Self::UnmuteSpecific(event_name, event_type) =>
                    format!("unmute: {}, {}", event_name, event_type),
            }
        )
    }
}
