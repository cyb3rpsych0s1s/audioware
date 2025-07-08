use red4ext_rs::{ScriptClass, class_kind::Native, types::IScriptable};

use super::Event;

#[repr(C)]
pub struct Surface {
    base: Event,
}

unsafe impl ScriptClass for Surface {
    const NAME: &'static str = "gameaudioeventsSurface";
    type Kind = Native;
}

impl AsRef<Event> for Surface {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for Surface {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct Dive {
    base: Event,
}

unsafe impl ScriptClass for Dive {
    const NAME: &'static str = "gameaudioeventsDive";
    type Kind = Native;
}

impl AsRef<Event> for Dive {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for Dive {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct Emerge {
    base: Event,
    pub oxygen: f32,
}

unsafe impl ScriptClass for Emerge {
    const NAME: &'static str = "gameaudioeventsEmerge";
    type Kind = Native;
}

impl AsRef<Event> for Emerge {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for Emerge {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}
