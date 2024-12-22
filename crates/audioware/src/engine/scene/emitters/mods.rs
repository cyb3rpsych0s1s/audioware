use kira::tween::Tween;
use red4ext_rs::types::CName;

use super::handles::Handles;

#[derive(Debug)]
pub struct EmitterMod {
    pub handles: Handles,
    pub name: Option<CName>,
}

impl EmitterMod {
    pub fn new(name: Option<CName>) -> Self {
        Self {
            name,
            handles: Handles::default(),
        }
    }
    pub fn stop_by_event_name(&mut self, event_name: CName, tween: Tween) {
        self.handles.stop_by_event_name(event_name, tween);
    }

    pub fn stop_emitters(&mut self, tween: Tween) {
        self.handles.stop(tween);
    }

    pub fn pause(&mut self, tween: Tween) {
        self.handles.pause(tween);
    }

    pub fn resume(&mut self, tween: Tween) {
        self.handles.resume(tween);
    }

    pub fn reclaim(&mut self) {
        self.handles.reclaim();
    }

    pub fn any_playing_handle(&self) -> bool {
        self.handles.any_playing_handle()
    }

    pub fn sync_dilation(&mut self, dilation: f64, tween: Tween) {
        self.handles.sync_dilation(dilation, tween);
    }
}
