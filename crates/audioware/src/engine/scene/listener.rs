use debug_ignore::DebugIgnore;
use kira::listener::ListenerHandle;
use red4ext_rs::types::{EntityId, WeakRef};

use crate::CameraComponent;

use super::dilation::Dilation;

#[derive(Debug)]
pub struct Listener {
    pub id: EntityId,
    pub handle: ListenerHandle,
    pub dilation: Dilation,
    pub overriden: Option<DebugIgnore<WeakRef<CameraComponent>>>,
}
