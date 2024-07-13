use red4ext_rs::{
    export_plugin, exports, global, log,
    types::{CName, EntityId, Ref},
    wcstr, Exportable, GameApp, GlobalExport, Plugin, PluginOps, RttiRegistrator, SdkEnv, SemVer,
    StateListener, U16CStr,
};
use types::{LocalizationPackage, Subtitle};

mod types;

pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = SemVer::new(1, 0, 0);

    fn on_init(env: &SdkEnv) {
        RttiRegistrator::add(Some(register), Some(post_register));
        env.add_listener(
            red4ext_rs::StateType::Initialization,
            StateListener::default().with_on_exit(on_exit_initialization),
        );
    }

    #[allow(clippy::transmute_ptr_to_ref)] // upstream lint
    fn exports() -> impl Exportable {
        exports![
            GlobalExport(global!(c"Audioware.RegisterListener", register_listener)),
            GlobalExport(global!(
                c"Audioware.UnregisterListener",
                unregister_listener
            )),
            GlobalExport(global!(c"Audioware.RegisterEmitter", register_emitter)),
            GlobalExport(global!(c"Audioware.UnregisterEmitter", unregister_emitter)),
            GlobalExport(global!(c"Audioware.EmittersCount", emitters_count)),
            GlobalExport(global!(c"Audioware.DefineSubtitles", define_subtitles)),
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

fn register_listener(emitter_id: EntityId) {
    let env = Audioware::env();
    log::info!(env, "TODO: register listener {:?} V", emitter_id);
}

fn unregister_listener(emitter_id: EntityId) {
    let env = Audioware::env();
    log::info!(env, "TODO: unregister listener {:?} V", emitter_id);
}

fn register_emitter(emitter_id: EntityId, emitter_name: CName) {
    let env = Audioware::env();
    log::info!(
        env,
        "TODO: register emitter {:?} {}",
        emitter_id,
        emitter_name
    );
}

fn unregister_emitter(emitter_id: EntityId) {
    let env = Audioware::env();
    log::info!(env, "TODO: unregister emitter {:?}", emitter_id);
}

fn emitters_count() -> i32 {
    let env = Audioware::env();
    log::info!(env, "TODO: emitters count");
    0
}

fn define_subtitles(package: Ref<LocalizationPackage>) {
    package.subtitle("custom_subtitle", "female", "male");
}
