use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, SpokenLocale, WrittenLocale};
use engine::{Engine, Preset};
use error::Error;
use hooks::*;
use red4ext_rs::{
    call, export_plugin_symbols, exports, global, log,
    types::{CName, GameEngine, Opt},
    wcstr, Exportable, GameApp, GlobalExport, Plugin, PluginOps, RttiRegistrator, RttiSystem,
    ScriptClass, SdkEnv, SemVer, StateListener, U16CStr,
};
use states::{GameState, State};
use types::{AsAudioSystem, AudioSystem, GameObject, Vector4};
use utils::{plog_error, plog_info, plog_warn};

mod config;
mod engine;
mod error;
mod hooks;
mod macros;
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
        if let Err(e) = Engine::setup() {
            log::error!(env, "Unable to load engine: {e}");
        }
        Ok(())
    }

    fn attach_hooks(env: &SdkEnv) {
        // native methods
        load_save_in_game::attach_hook(env);
        on_transform_updated::attach_hook(env);
        parameter::attach_hook(env);
        play::attach_hook(env);
        play_on_emitter::attach_hook(env);
        stop::attach_hook(env);
        switch::attach_hook(env);
        // native event handlers
        #[cfg(debug_assertions)]
        {
            dialog_line::attach_hook(env);
            dialog_line_end::attach_hook(env);
            sound_play_vo::attach_hook(env);
            voice_play_event::attach_hook(env);
        }
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
            GlobalExport(global!(c"Audioware.PLog", plog_info)),
            GlobalExport(global!(c"Audioware.PLogWarning", plog_warn)),
            GlobalExport(global!(c"Audioware.PLogError", plog_error)),
            GlobalExport(global!(c"Audioware.Shutdown", Engine::shutdown)),
            GlobalExport(global!(
                c"Audioware.RegisterEmitter",
                Engine::register_emitter
            )),
            GlobalExport(global!(
                c"Audioware.UnregisterEmitter",
                Engine::unregister_emitter
            )),
            GlobalExport(global!(c"Audioware.EmittersCount", Engine::emitters_count)),
            GlobalExport(global!(
                c"Audioware.IsRegisteredEmitter",
                Engine::is_registered_emitter
            )),
            GlobalExport(global!(
                c"Audioware.DefineSubtitles",
                Engine::define_subtitles
            )),
            GlobalExport(global!(
                c"Audioware.SupportedLanguages",
                Engine::supported_languages
            )),
            GlobalExport(global!(c"Audioware.SetGameState", GameState::set)),
            GlobalExport(global!(c"Audioware.SetPlayerGender", set_player_gender)),
            GlobalExport(global!(c"Audioware.UnsetPlayerGender", unset_player_gender)),
            GlobalExport(global!(c"Audioware.SetGameLocales", set_game_locales)),
            GlobalExport(global!(
                c"Audioware.PlayOverThePhone",
                Engine::play_over_the_phone
            )),
            GlobalExport(global!(c"Audioware.Play", Engine::play)),
            GlobalExport(global!(c"Audioware.Stop", Engine::stop)),
            GlobalExport(global!(c"Audioware.Pause", Engine::pause)),
            GlobalExport(global!(c"Audioware.Resume", Engine::resume)),
            GlobalExport(global!(c"Audioware.Switch", Engine::switch)),
            GlobalExport(global!(c"Audioware.PlayOnEmitter", Engine::play_on_emitter)),
            GlobalExport(global!(c"Audioware.StopOnEmitter", Engine::stop_on_emitter)),
            GlobalExport(global!(c"Audioware.SetPlayerReverb", set_player_reverb)),
            GlobalExport(global!(c"Audioware.SetPlayerPreset", set_player_preset)),
            GlobalExport(global!(c"Audioware.TestPlay", test_play))
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
    // test_get_player();
    // test_is_player();
    // scan_globals("PropagateSubtitle");
    utils::info("it should be able to call FTLog");
    utils::warn("it should be able to call FTLogWarning");
    utils::error("it should be able to call FTLogError");
}

unsafe extern "C" fn on_exit_running(_game: &GameApp) {
    let env = Audioware::env();
    log::info!(env, "on exit running: Audioware");
    GameState::swap(GameState::Unload);
    Engine::shutdown();
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

#[allow(dead_code)]
fn test_is_player() {
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let cls = rtti.get_class(CName::new(GameObject::NAME)).unwrap();
    match cls.get_method(CName::new("IsPlayer;")) {
        Ok(x) => {
            log::info!(env, "IsPlayer ====> {x:#?}");
        }
        Err(e) => {
            log::error!(
                env,
                "IsPlayer ====> {}",
                e.into_iter()
                    .map(|x| x.as_function().name().as_str())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    };
}

#[allow(dead_code)]
fn test_get_player() {
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let methods = rtti.get_global_functions();
    match methods.iter().find(|x| {
        x.name() == CName::new("GetPlayer")
            || x.short_name() == CName::new("GetPlayer")
            || x.name() == CName::new("GetPlayer;GameInstance")
            || x.short_name() == CName::new("GetPlayer;GameInstance")
    }) {
        Some(x) => {
            log::info!(
                env,
                "GetPlayer ====> full: {}, short: {}\n{x:#?}",
                x.name().as_str(),
                x.short_name().as_str()
            );
        }
        None => {
            log::error!(env, "GetPlayer ====> NOT FOUND");
        }
    };
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

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn scan_class(class_name: &str) {
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let cls = rtti.get_class(CName::new(class_name)).unwrap();
    log::info!(env, "{} ({:#02X})", cls.name(), cls.size());
    let static_methods = cls.static_methods();
    for s in static_methods.iter() {
        log::info!(
            env,
            "static => {} ({})",
            s.as_function().name(),
            s.as_function().short_name()
        );
    }
    let member_methods = cls.methods();
    for m in member_methods.iter() {
        log::info!(
            env,
            "member => {} ({})",
            m.as_function().name(),
            m.as_function().short_name()
        );
    }
    let global_methods = rtti.get_global_functions();
    for g in global_methods.iter() {
        if g.name().as_str().contains(class_name) || g.short_name().as_str().contains(class_name) {
            log::info!(env, "global => {} ({})", g.name(), g.short_name());
        }
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn scan_globals(func_name: &str) {
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let global_methods = rtti.get_global_functions();
    for g in global_methods.iter() {
        if g.name().as_str().contains(func_name) || g.short_name().as_str().contains(func_name) {
            log::info!(env, "global => {} ({})", g.name(), g.short_name());
        }
    }
}

fn set_player_reverb(value: f32) {
    Engine::set_player_reverb(value);
}

fn set_player_preset(value: Preset) {
    Engine::set_player_preset(value);
}
