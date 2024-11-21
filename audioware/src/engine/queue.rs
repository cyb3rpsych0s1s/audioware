use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        LazyLock, OnceLock,
    },
    thread::JoinHandle,
    time::Duration,
};

use audioware_manifest::SpokenLocale;
use bitflags::bitflags;
use crossbeam::channel::{bounded, tick, Receiver, Sender};
use kira::manager::{
    backend::cpal::{CpalBackend, CpalBackendSettings},
    AudioManagerSettings,
};
use red4ext_rs::{
    log::{self},
    types::CName,
    SdkEnv,
};
use std::sync::{Mutex, RwLock};

use crate::{
    abi::{
        command::Command,
        lifecycle::{Board, Codeware, Lifecycle, Session, System},
    },
    config::BufferSize,
    engine::DilationUpdate,
    error::Error,
    utils::{fails, lifecycle},
};

use super::Engine;

bitflags! {
    struct Flags: u32 {
        const INITIALIZING = 1 << 0;
        const LOADING = 1 << 1;
        const IN_MENU = 1 << 2;
        const IN_GAME = 1 << 3;
    }
}

static THREAD: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();
static LIFECYCLE: OnceLock<RwLock<Option<Sender<Lifecycle>>>> = OnceLock::new();
static COMMAND: OnceLock<RwLock<Option<Sender<Command>>>> = OnceLock::new();
static STATE: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(Flags::empty().bits()));

fn load(env: &SdkEnv) -> Result<(Engine<CpalBackend>, usize), Error> {
    let buffer_size = BufferSize::read_ini();
    let mut backend_settings = CpalBackendSettings::default();
    if buffer_size != BufferSize::Auto {
        backend_settings.buffer_size = cpal::BufferSize::Fixed(buffer_size as u32);
        log::info!(env, "buffer size read from .ini: {}", buffer_size as u32);
    }
    let manager_settings = AudioManagerSettings::<CpalBackend> {
        backend_settings,
        ..Default::default()
    };
    let capacity = manager_settings.capacities.command_capacity;
    Ok((Engine::try_new(manager_settings)?, capacity))
}

pub fn spawn(env: &SdkEnv) -> Result<(), Error> {
    lifecycle!("spawn plugin thread");
    STATE.store(Flags::LOADING.bits(), Ordering::Release);
    let (engine, capacity) = load(env)?;
    let _ = THREAD.set(Mutex::new(Some(
        std::thread::Builder::new()
            .name("audioware".into())
            .spawn(move || {
                lifecycle!("initialize channels...");
                let (sl, rl) = bounded::<Lifecycle>(32);
                let (sc, rc) = bounded::<Command>(capacity);
                let _ = LIFECYCLE.set(RwLock::new(Some(sl)));
                let _ = COMMAND.set(RwLock::new(Some(sc)));
                lifecycle!("initialized channels");
                self::run(rl, rc, engine);
            })?,
    )));
    lifecycle!("spawned plugin thread");
    Ok(())
}

pub fn run(rl: Receiver<Lifecycle>, rc: Receiver<Command>, mut engine: Engine<CpalBackend>) {
    let spoken = SpokenLocale::default();
    let mut gender = None;
    let s = |x| Duration::from_secs_f32(x);
    let ms = |x| Duration::from_millis(x);
    let reclamation = tick(s(if cfg!(debug_assertions) { 3. } else { 60. }));
    let synchronization = tick(ms(15));
    let mut should_sync = false;
    'game: loop {
        for l in rl.try_iter() {
            lifecycle!("> {l}");
            match l {
                Lifecycle::Terminate => {
                    break 'game;
                }
                Lifecycle::ReportInitialization => engine.report_initialization(false),
                #[cfg(debug_assertions)]
                Lifecycle::HotReload => {
                    engine.hot_reload();
                    continue 'game;
                }
                Lifecycle::SetListenerDilation {
                    value,
                    reason,
                    ease_in_curve,
                } => engine.set_listener_dilation(DilationUpdate::Set {
                    value,
                    reason,
                    ease_in_curve,
                }),
                Lifecycle::UnsetListenerDilation {
                    reason,
                    ease_out_curve,
                } => engine.unset_listener_dilation(DilationUpdate::Unset {
                    reason,
                    ease_out_curve,
                }),
                Lifecycle::SetEmitterDilation {
                    entity_id,
                    value,
                    ease_in_curve,
                    reason,
                } => engine.set_emitter_dilation(
                    entity_id,
                    DilationUpdate::Set {
                        reason,
                        value,
                        ease_in_curve,
                    },
                ),
                Lifecycle::UnsetEmitterDilation {
                    entity_id,
                    ease_out_curve,
                } => engine.unset_emitter_dilation(
                    entity_id,
                    DilationUpdate::Unset {
                        reason: CName::undefined(),
                        ease_out_curve,
                    },
                ),
                Lifecycle::Codeware(Codeware::SetPlayerGender { gender: value }) => {
                    gender = Some(value)
                }
                Lifecycle::Codeware(Codeware::UnsetPlayerGender) => gender = None,
                Lifecycle::RegisterEmitter {
                    entity_id,
                    emitter_name,
                    emitter_settings,
                    sender,
                } => {
                    let registered =
                        engine.register_emitter(entity_id, emitter_name, emitter_settings);
                    let _ = sender.try_send(registered);
                }
                Lifecycle::UnregisterEmitter { entity_id, sender } => {
                    let unregistered = engine.unregister_emitter(entity_id);
                    let _ = sender.try_send(unregistered);
                }
                Lifecycle::OnEmitterDies { entity_id } => engine.on_emitter_dies(entity_id),
                Lifecycle::OnEmitterIncapacitated { entity_id } => {
                    engine.on_emitter_incapacitated(entity_id)
                }
                Lifecycle::OnEmitterDefeated { .. } => {}
                Lifecycle::SetVolume { setting, value } => engine.set_volume(setting, value),
                Lifecycle::Session(Session::BeforeStart) => engine.reset(),
                Lifecycle::Session(Session::Start) => {}
                Lifecycle::Session(Session::End) => {}
                Lifecycle::Session(Session::Ready) => {}
                Lifecycle::Session(Session::Pause) => {
                    should_sync = false;
                }
                Lifecycle::Session(Session::Resume) => {
                    should_sync = true;
                }
                Lifecycle::Session(Session::BeforeEnd) => {
                    should_sync = false;
                    engine.scene = None;
                }
                Lifecycle::System(System::Attach) => {}
                Lifecycle::System(System::Detach) => {}
                Lifecycle::System(System::PlayerAttach) => match engine.try_new_scene() {
                    Ok(_) => {
                        should_sync = true;
                    }
                    Err(e) => lifecycle!("failed to create new scene: {e}"),
                },
                Lifecycle::System(System::PlayerDetach) => engine.stop_scene_emitters(),
                Lifecycle::Board(Board::UIMenu(true)) => {
                    should_sync = false;
                    engine.pause()
                }
                Lifecycle::Board(Board::UIMenu(false)) => {
                    should_sync = engine.scene.is_some();
                    engine.resume()
                }
                Lifecycle::Board(Board::ReverbMix(value)) => engine.set_reverb_mix(value),
                Lifecycle::Board(Board::Preset(value)) => engine.set_preset(value),
            }
        }
        if should_sync && engine.any_emitter() && synchronization.try_recv().is_ok() {
            engine.sync_scene();
        }
        if engine.any_handle() && reclamation.try_recv().is_ok() {
            engine.reclaim();
        }
        for c in rc.try_iter().take(8) {
            lifecycle!("> {c}");
            match c {
                Command::PlayVanilla {
                    event_name,
                    entity_id,
                    emitter_name,
                } => engine.play(event_name, entity_id, emitter_name, spoken, gender, None),
                Command::Play {
                    sound_name,
                    entity_id,
                    emitter_name,
                    tween,
                    ..
                } => engine.play(sound_name, entity_id, emitter_name, spoken, gender, tween),
                Command::PlayExt {
                    sound_name,
                    entity_id,
                    emitter_name,
                    ext,
                    ..
                } => engine.play_ext(sound_name, entity_id, emitter_name, spoken, gender, ext),
                Command::PlayOnEmitter {
                    event_name,
                    entity_id,
                    emitter_name,
                    tween,
                } => engine.play_on_emitter(
                    event_name,
                    entity_id,
                    emitter_name,
                    tween,
                    spoken,
                    gender,
                ),
                Command::PlayOverThePhone { .. } => {}
                Command::StopOnEmitter {
                    event_name,
                    entity_id,
                    emitter_name,
                    tween,
                } => engine.stop_on_emitter(event_name, entity_id, emitter_name, tween),
                Command::StopVanilla {
                    event_name,
                    entity_id,
                    emitter_name,
                } => engine.stop(event_name, entity_id, emitter_name, None),
                Command::Stop {
                    event_name,
                    entity_id,
                    emitter_name,
                    tween,
                } => engine.stop(event_name, entity_id, emitter_name, tween),
                Command::StopFor { .. } => {}
                Command::Switch { .. } => {}
            }
        }
    }
    let _ = LIFECYCLE
        .get()
        .and_then(|x| x.write().ok().map(|mut x| x.take()));
    let _ = COMMAND
        .get()
        .and_then(|x| x.write().ok().map(|mut x| x.take()));
    lifecycle!("closed engine");
}

#[allow(dead_code)]
pub fn join() {
    lifecycle!("join engine thread");
    if let Some(Err(e)) = THREAD
        .get()
        .and_then(|x| x.try_lock().ok())
        .and_then(|mut x| x.take())
        .map(|x| x.join())
    {
        fails!("failed to join engine thread: {e:?}");
    }
}

pub fn notify(lifecycle: Lifecycle) {
    lifecycle!("{lifecycle}");
    if let Some(x) = LIFECYCLE.get() {
        if let Ok(x) = x.try_read() {
            if let Some(x) = x.as_ref() {
                if let Err(e) = x.try_send(lifecycle) {
                    fails!("failed to notify plugin lifecycle: {e}");
                }
            }
        } else {
            fails!("plugin game channel is not initialized");
        }
    } else {
        fails!("plugin game loop is not initialized");
    }
}

pub fn send(command: Command) {
    lifecycle!("{command}");
    if let Some(x) = COMMAND.get() {
        if let Ok(x) = x.try_read() {
            if let Some(x) = x.as_ref() {
                if let Err(e) = x.try_send(command) {
                    fails!("failed to send plugin command: {e}");
                }
            }
        } else {
            fails!("plugin command channel is not initialized");
        }
    } else {
        fails!("plugin game loop is not initialized");
    }
}
