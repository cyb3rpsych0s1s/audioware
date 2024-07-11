use red4rs::{
    export_plugin, exports, systems::RttiRegistrator, wcstr, ClassExport, Exportable, Plugin,
    SdkEnv, SemVer, U16CStr,
};
use system::AudiowareSystem;

mod system;
mod types;

pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = SemVer::new(1, 0, 0);

    fn on_init(env: &SdkEnv) {
        // we can request the RTTI to invoke our functions to do some setup
        RttiRegistrator::add(Some(register), Some(post_register));
    }

    fn exports() -> impl Exportable {
        exports![ClassExport::<AudiowareSystem>::builder()
            .base("gameScriptableSystem")
            .build(),]
    }
}

export_plugin!(Audioware);

unsafe extern "C" fn register() {}

unsafe extern "C" fn post_register() {}
