use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, SpokenLocale, WrittenLocale};
use engine::{AudioRegion, AudioSettingsExt, AudioSettingsExtBuilder, Engine};
use ext::AudioSystemExt;
use hooks::*;
use red4ext_rs::{
    call, export_plugin_symbols, exports, global, log, methods, static_methods,
    types::{CName, GameEngine, IScriptable, Opt, RedArray, RedString, ScriptableSystem},
    wcstr, ClassExport, Exportable, GameApp, GlobalExport, Plugin, PluginOps, RttiRegistrator,
    RttiSystem, ScriptClass, SdkEnv, SemVer, StateListener, U16CStr,
};
use spatialization::ExtSystem;
use states::{GameState, State, ToggleState};
use types::{
    Args, AsAudioSystem, AudioSystem, EmitterDistances, EmitterSettings, GameObject, LoopRegion,
    Vector4,
};
use utils::{plog_error, plog_info, plog_warn};

mod config;
mod engine;
mod error;
mod ext;
mod hooks;
mod macros;
mod spatialization;
mod states;
mod types;
mod utils;

#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\version.rs"));
#[cfg(not(target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/version.rs"));

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
        #[cfg(debug_assertions)]
        log::info!(
            env,
            "as_if_I_didnt_know_already: {}",
            Banks::exists(&CName::new("as_if_I_didnt_know_already"))
        );
    }

    fn load_engine(env: &SdkEnv) {
        if let Err(e) = Engine::setup() {
            log::error!(env, "Unable to load engine: {e}");
        }
    }

    fn attach_hooks(env: &SdkEnv) {
        // native methods
        global_parameter::attach_hook(env);
        load_save_in_game::attach_hook(env);
        on_transform_updated::attach_hook(env);
        parameter::attach_hook(env);
        play::attach_hook(env);
        play_on_emitter::attach_hook(env);
        stop::attach_hook(env);
        switch::attach_hook(env);
        // queue_event::attach_hook(env); // 🌊
        // queue_event_for_entity_id::attach_hook(env); // 🌊
        // native event handlers
        #[cfg(debug_assertions)]
        {
            crate::hooks::events::vehicle::attach_hook(env);
            // crate::hooks::events::choice::attach_hook(env); // not thoroughly tested yet
            // crate::hooks::events::audio::attach_hook(env); // 🌊
            // crate::hooks::events::spawn_effect_event::attach_hook(env); // 🌊
            crate::hooks::events::sound_event::attach_hook(env);
            crate::hooks::events::music::attach_hook(env);
            // sound_parameter::attach_hook(env); // redundant with global_parameter
            crate::hooks::events::surface::attach_hook(env);
            crate::hooks::events::dive::attach_hook(env);
            crate::hooks::events::emerge::attach_hook(env);
            crate::hooks::events::dialog_line::attach_hook(env);
            crate::hooks::events::dialog_line_end::attach_hook(env);
            crate::hooks::events::sound_play_vo::attach_hook(env);
            crate::hooks::events::play_sound::attach_hook(env);
            crate::hooks::events::stop_sound::attach_hook(env);
            crate::hooks::events::sound_switch::attach_hook(env);
            // crate::hooks::events::stop_tagged_sounds::attach_hook(env); // ❌
            crate::hooks::events::stop_dialog_line::attach_hook(env);
            crate::hooks::events::play_sound_on_emitter::attach_hook(env);
            crate::hooks::events::stop_sound_on_emitter::attach_hook(env);
            crate::hooks::events::set_parameter_on_emitter::attach_hook(env);
            crate::hooks::events::voice_event::attach_hook(env);
            crate::hooks::events::voice_played_event::attach_hook(env);
        }
    }
}

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    fn on_init(env: &SdkEnv) {
        Self::register_listeners(env);
        Self::load_banks(env);
        Self::load_engine(env);
        Self::attach_hooks(env);
    }

    #[allow(clippy::transmute_ptr_to_ref)] // upstream lint
    fn exports() -> impl Exportable {
        exports![
            ClassExport::<EmitterDistances>::builder().build(),
            ClassExport::<EmitterSettings>::builder().build(),
            ClassExport::<LoopRegion>::builder().build(),
            ClassExport::<Args>::builder().build(),
            GlobalExport(global!(c"Audioware.Version", version)),
            GlobalExport(global!(c"Audioware.SemanticVersion", semantic_version)),
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
            GlobalExport(global!(
                c"Audioware.DefineSubtitles",
                Engine::define_subtitles
            )),
            GlobalExport(global!(
                c"Audioware.SupportedLanguages",
                Engine::supported_languages
            )),
            GlobalExport(global!(
                c"Audioware.SetGameState",
                GameState::set_and_toggle
            )),
            GlobalExport(global!(c"Audioware.SetPlayerGender", set_player_gender)),
            GlobalExport(global!(c"Audioware.UnsetPlayerGender", unset_player_gender)),
            GlobalExport(global!(c"Audioware.SetGameLocales", set_game_locales)),
            GlobalExport(global!(c"Audioware.Pause", Engine::pause)),
            GlobalExport(global!(c"Audioware.Resume", Engine::resume)),
            GlobalExport(global!(c"Audioware.SetReverbMix", Engine::set_reverb_mix)),
            GlobalExport(global!(c"Audioware.SetPreset", Engine::set_preset)),
            GlobalExport(global!(c"Audioware.SetVolume", Engine::set_volume)),
            ClassExport::<ExtSystem>::builder()
                .base(ScriptableSystem::NAME)
                .build(),
            ClassExport::<AudioSystemExt>::builder()
                .base(IScriptable::NAME)
                .methods(methods![
                    final c"Play" => AudioSystemExt::play,
                    c"Stop" => AudioSystemExt::stop,
                    c"Switch" => AudioSystemExt::switch,
                    c"PlayOverThePhone" => AudioSystemExt::play_over_the_phone,
                    c"IsRegisteredEmitter" => AudioSystemExt::is_registered_emitter,
                    c"EmittersCount" => AudioSystemExt::emitters_count,
                    c"PlayOnEmitter" => AudioSystemExt::play_on_emitter,
                    c"StopOnEmitter" => AudioSystemExt::stop_on_emitter,
                    c"OnEmitterDies" => AudioSystemExt::on_emitter_dies,
                ])
                .build(),
            ClassExport::<AudioRegion>::builder()
                .base(IScriptable::NAME)
                .methods(methods![
                    c"SetStart" => AudioRegion::set_start,
                    c"SetEnd" => AudioRegion::set_end,
                ])
                .build(),
            ClassExport::<AudioSettingsExt>::builder()
                .base(IScriptable::NAME)
                .build(),
            ClassExport::<AudioSettingsExtBuilder>::builder()
                .base(IScriptable::NAME)
                .static_methods(static_methods![
                    c"Create" => AudioSettingsExtBuilder::create
                ])
                .methods(methods![
                    c"SetStartPosition" => AudioSettingsExtBuilder::set_start_position,
                    c"SetLoopRegionStarts" => AudioSettingsExtBuilder::set_loop_region_starts,
                    c"SetLoopRegionEnds" => AudioSettingsExtBuilder::set_loop_region_ends,
                    c"SetVolume" => AudioSettingsExtBuilder::set_volume,
                    c"SetFadeInTween" => AudioSettingsExtBuilder::set_fade_in_tween,
                    c"SetPanning" => AudioSettingsExtBuilder::set_panning,
                    c"SetPlaybackRate" => AudioSettingsExtBuilder::set_playback_rate,
                    c"Build" => AudioSettingsExtBuilder::build,
                ])
                .build()
        ]
    }
}

export_plugin_symbols!(Audioware);

unsafe extern "C" fn register() {}

unsafe extern "C" fn post_register() {}

unsafe extern "C" fn on_exit_initialization(_game: &GameApp) {
    let env = Audioware::env();
    log::info!(env, "on exit initialization: Audioware");
    // test_play();
    // test_static();
    // test_get_player();
    // test_is_player();
    // scan_globals("PropagateSubtitle");
    #[cfg(debug_assertions)]
    {
        utils::info("it should be able to call FTLog");
        utils::warn("it should be able to call FTLogWarning");
        utils::error("it should be able to call FTLogError");
    }
}

unsafe extern "C" fn on_exit_running(_game: &GameApp) {
    let env = Audioware::env();
    log::info!(env, "on exit running: Audioware");
    GameState::set(GameState::Unload);
    Engine::shutdown();
}

// TODO: replace with upstream conversion when PR merged.
fn version() -> RedString {
    RedString::from("1.0.0-alpha.10")
}

// TODO: replace with upstream conversion when PR merged.
fn semantic_version() -> RedArray<u32> {
    RedArray::from_iter([1, 0, 0, 1, 10])
}

fn set_player_gender(value: PlayerGender) {
    PlayerGender::set(Some(value));
}

fn unset_player_gender() {
    PlayerGender::set(None);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn test_static() {
    // CallbackSystemTarget => native: true, size: 0x40, value holder size: 0x0, align: 0x4, parent: IScriptable
    #[rustfmt::skip] #[cfg(debug_assertions)] scan_repr("CallbackSystemTarget");
    // EntityTarget => native: true, size: 0x68, value holder size: 0x0, align: 0x4, parent: CallbackSystemTarget
    #[rustfmt::skip] #[cfg(debug_assertions)] scan_repr("EntityTarget");
    // WorldPosition => native: true, size: 0xC, value holder size: 0x0, align: 0x4, parent: None
    #[rustfmt::skip] #[cfg(debug_assertions)] scan_repr("WorldPosition");
    // e.g. static => SetX (SetX)
    // #[rustfmt::skip] #[cfg(debug_assertions)] scan_class("WorldPosition");
    // e.g. static => FindEntityByID (FindEntityByID)
    // #[rustfmt::skip] #[cfg(debug_assertions)] scan_class("ScriptGameInstance");

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
    let cls = match rtti.get_class(CName::new(class_name)) {
        Some(cls) => cls,
        None => {
            log::error!(env, "class {class_name} does not exist.");
            return;
        }
    };
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

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn scan_repr(cls_name: &str) {
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let cls = rtti.get_class(CName::new(cls_name)).unwrap();
    log::info!(
        env,
        "{} => native: {}, size: {:#02X}, value holder size: {:#02X}, align: {:#02X}, parent: {}",
        cls.name(),
        cls.flags().is_native(),
        cls.size(),
        cls.holder_size(),
        cls.alignment(),
        cls.base().map(|x| x.name()).unwrap_or_default()
    );
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn whoami(cname_hash: u64) {
    let env = Audioware::env();
    let cname = CName::from(cname_hash);
    log::info!(env, "whoami: {}", cname.as_str());
}
