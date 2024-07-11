use red4rs::{export_plugin, exports, wcstr, ClassExport, Exportable, Plugin, SemVer, U16CStr};
use system::AudiowareSystem;

mod system;
mod types;

pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = SemVer::new(1, 0, 0);

    fn exports() -> impl Exportable {
        exports![ClassExport::<AudiowareSystem>::builder()
            .base("gameScriptableSystem")
            .build()]
    }
}

export_plugin!(Audioware);
