use crossbeam::channel::Sender;

pub enum Queue {
    Missing,
    Loading,
    Ready {
        lifecycle: Lifecycle,
        commands: Commands,
    },
    Unloading,
}

pub struct Lifecycle;

impl Queue {
    pub fn lifecycle(&self) -> Option<Sender<Lifecycle>> {
        match self {
            Queue::Ready { .. } => todo!(),
            _ => None,
        }
    }
}

pub struct Commands;
impl Commands {}