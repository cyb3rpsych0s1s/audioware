use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, SpokenLocale, WrittenLocale};
use engine::Engine;
use error::Error;
use hooks::*;
use red4ext_rs::{
    call, export_plugin_symbols, exports, global, log,
    types::{CName, EntityId, GameEngine, Opt, Ref},
    wcstr, Exportable, GameApp, GlobalExport, Plugin, PluginOps, RttiRegistrator, RttiSystem,
    ScriptClass, SdkEnv, SemVer, StateListener, U16CStr,
};
use states::{GameState, State};
use types::{AsAudioSystem, AudioSystem, LocalizationPackage, Subtitle, Vector4};
use utils::{plog_error, plog_info, plog_warn};

mod engine;
mod error;
mod hooks;
mod states;
mod types;
mod utils;

pub struct Audioware;

impl Audioware {
    fn register_listeners(env: &SdkEnv) {
        RttiRegistrator::add(Some(register), Some(post_register));
        env.add_listener(
            red4ext_rs::StateType::Initialization,
            StateListener::default().with_on_exit(on_exit_initialization),
        );
        env.add_listener(
            red4ext_rs::StateType::Running,
            StateListener::default().with_on_exit(on_exit_running),
        );
    }

    fn load_banks(env: &SdkEnv) {
        let report = Banks::setup();
        let status = if report.errors.is_empty() {
            "successfully"
        } else {
            "partially"
        };
        log::info!(env, "banks {status} initialized:\n{report}");
        for error in report.errors {
            log::error!(env, "{error}");
        }
        log::info!(
            env,
            "as_if_I_didnt_know_already: {}",
            Banks::exists(&CName::new("as_if_I_didnt_know_already"))
        );
    }

    fn load_engine(env: &SdkEnv) -> Result<(), Error> {
        Engine::setup()?;
        Ok(())
    }

    fn attach_hooks(env: &SdkEnv) {
        parameter::attach_hook(env);
        play::attach_hook(env);
        play_on_emitter::attach_hook(env);
        stop::attach_hook(env);
        switch::attach_hook(env);
    }
}

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = SemVer::new(1, 0, 0);

    fn on_init(env: &SdkEnv) {
        GameState::set(GameState::Load);
        Self::register_listeners(env);
        Self::load_banks(env);
        if let Err(e) = Self::load_engine(env) {
            log::error!(env, "Unable to load engine: {e}");
        }
        Self::attach_hooks(env);
    }

    #[allow(clippy::transmute_ptr_to_ref)] // upstream lint
    fn exports() -> impl Exportable {
        exports![
            GlobalExport(global!(c"Audioware.RegisterListener", register_listener)),
            GlobalExport(global!(
                c"Audioware.UnregisterListener",
                unregister_listener
            )),
            GlobalExport(global!(c"Audioware.PLog", plog_info)),
            GlobalExport(global!(c"Audioware.PLogWarning", plog_warn)),
            GlobalExport(global!(c"Audioware.PLogError", plog_error)),
            GlobalExport(global!(c"Audioware.RegisterEmitter", register_emitter)),
            GlobalExport(global!(c"Audioware.UnregisterEmitter", unregister_emitter)),
            GlobalExport(global!(c"Audioware.EmittersCount", emitters_count)),
            GlobalExport(global!(c"Audioware.DefineSubtitles", define_subtitles)),
            GlobalExport(global!(c"Audioware.SetGameState", set_game_state)),
            GlobalExport(global!(c"Audioware.SetPlayerGender", set_player_gender)),
            GlobalExport(global!(c"Audioware.UnsetPlayerGender", unset_player_gender)),
            GlobalExport(global!(c"Audioware.SetGameLocales", set_game_locales)),
            GlobalExport(global!(c"Audioware.TestPlay", test_play)),
        ]
    }
}

export_plugin_symbols!(Audioware);

unsafe extern "C" fn register() {}

unsafe extern "C" fn post_register() {}

unsafe extern "C" fn on_exit_initialization(_game: &GameApp) {
    let env = Audioware::env();
    log::info!(env, "on exit initialization: Audioware");
    test_play();
    test_static();
    utils::info("it should be able to call FTLog");
    utils::warn("it should be able to call FTLogWarning");
    utils::error("it should be able to call FTLogError");
}

unsafe extern "C" fn on_exit_running(_game: &GameApp) {
    let env = Audioware::env();
    log::info!(env, "on exit running: Audioware");
    GameState::set(GameState::Unload);
}

fn register_listener(emitter_id: EntityId) {
    log::info!(Audioware::env(), "register listener {:?} V", emitter_id);
    Engine::register_listener(emitter_id);
}

fn unregister_listener(emitter_id: EntityId) {
    log::info!(Audioware::env(), "unregister listener {:?} V", emitter_id);
    Engine::unregister_listener(emitter_id);
}

fn register_emitter(emitter_id: EntityId, emitter_name: CName) {
    log::info!(
        Audioware::env(),
        "TODO: register emitter {:?} {}",
        emitter_id,
        emitter_name
    );
}

fn unregister_emitter(emitter_id: EntityId) {
    log::info!(
        Audioware::env(),
        "TODO: unregister emitter {:?}",
        emitter_id
    );
}

fn emitters_count() -> i32 {
    log::info!(Audioware::env(), "TODO: emitters count");
    0
}

fn define_subtitles(package: Ref<LocalizationPackage>) {
    package.subtitle("custom_subtitle", "female", "male");
}

fn set_player_gender(value: PlayerGender) {
    PlayerGender::set(Some(value));
}

fn unset_player_gender() {
    PlayerGender::set(None);
}

fn test_play() {
    let rtti = RttiSystem::get();
    let class = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
    let engine = GameEngine::get();
    let game = engine.game_instance();
    let system = game
        .get_system(class.as_type())
        .cast::<AudioSystem>()
        .unwrap();
    system.play(CName::new("ono_v_pain_long"), Opt::Default, Opt::Default);
}

fn test_static() {
    let env = Audioware::env();
    let from = Vector4 {
        x: 0.,
        y: 1.,
        z: 2.,
        w: 3.,
    };
    let to = Vector4 {
        x: 3.,
        y: 2.,
        z: 1.,
        w: 0.,
    };
    let distance = call!("Vector4"::"Distance"(from, to) -> f32).unwrap();
    log::info!(env, "distance: {distance}");

    let euler = call!("MathHelper"::"EulerNumber;"() -> f32).unwrap();
    log::info!(env, "Euler number: {euler}");

    let threshold = call!("PlayerPuppet"::"GetCriticalHealthThreshold;"() -> f32).unwrap();
    log::info!(env, "player critical health threshold: {threshold}");
}

fn set_game_state(after: GameState) {
    GameState::set(after);
}

fn set_game_locales(spoken: CName, written: CName) {
    let env = Audioware::env();
    let (spoken, written): (SpokenLocale, WrittenLocale) = (
        match spoken.try_into() {
            Ok(x) => x,
            Err(e) => {
                log::error!(env, "Invalid spoken language: {}", e);
                return;
            }
        },
        match written.try_into() {
            Ok(x) => x,
            Err(e) => {
                log::error!(env, "Invalid written language: {}", e);
                return;
            }
        },
    );
    SpokenLocale::set(spoken);
    WrittenLocale::set(written);
}
