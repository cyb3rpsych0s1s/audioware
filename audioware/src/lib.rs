use plugin::AudiowarePlugin;
use red4rs::{
    export_plugin, exports, global, log, systems::RttiRegistrator, types::Ref, wcstr, ClassExport,
    Exportable, GameApp, GlobalExport, Plugin, PluginOps, SdkEnv, SemVer, StateListener, U16CStr,
};
use system::{AudiowareSystem, Yolo};

mod plugin;
mod system;
mod types;

pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = SemVer::new(1, 0, 0);

    fn on_init(env: &SdkEnv) {
        RttiRegistrator::add(Some(register), Some(post_register));
        env.add_listener(
            red4rs::StateType::Initialization,
            StateListener::default().with_on_exit(on_exit_initialization),
        );
    }

    fn exports() -> impl Exportable {
        exports![
            GlobalExport(global!(c"Audioware.CallYoloOn", call_yolo_on)),
            // ClassExport::<AudiowarePlugin>::builder()
            //     .base("IScriptable")
            //     .build(),
        ]
    }
}

export_plugin!(Audioware);

unsafe extern "C" fn register() {}

unsafe extern "C" fn post_register() {}

unsafe extern "C" fn on_exit_initialization(_game: &GameApp) {
    let env = Audioware::env();
    log::info!(env, "on exit initialization: Audioware");
}

fn call_yolo_on(system: Ref<AudiowareSystem>) {
    if let Some(x) = unsafe { system.fields() } {
        x.yolo()
    }
}
