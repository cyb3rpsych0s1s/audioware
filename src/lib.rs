use red4ext_rs::plugin::Plugin;
use red4ext_rs::plugin::Version;
use red4ext_rs::prelude::*;

pub struct Audioware;
impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);
    fn register() {
        info!("on attach audioware");
    }
    fn unload() {
        info!("on detach audioware");
    }
}
