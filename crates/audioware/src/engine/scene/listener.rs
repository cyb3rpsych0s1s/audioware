use kira::listener::ListenerHandle;
use red4ext_rs::types::EntityId;

use super::dilation::Dilation;

#[derive(Debug)]
pub struct Listener {
    pub id: EntityId,
    pub handle: ListenerHandle,
    pub dilation: Dilation,
}
