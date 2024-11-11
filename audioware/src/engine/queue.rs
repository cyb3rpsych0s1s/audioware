use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        LazyLock, OnceLock,
    },
    thread::JoinHandle,
};

use audioware_bank::BankData;
use audioware_manifest::{PlayerGender, SpokenLocale};
use bitflags::bitflags;
use crossbeam::channel::{bounded, Receiver, Sender};
use either::Either;
use kira::manager::{
    backend::{
        cpal::{CpalBackend, CpalBackendSettings},
        Backend,
    },
    AudioManagerSettings,
};
use red4ext_rs::{
    log::{self},
    PluginOps, SdkEnv,
};
use snowflake::ProcessUniqueId;
use std::sync::{Mutex, RwLock};

use crate::{
    abi::{
        command::Command,
        lifecycle::{Board, Lifecycle, Session, System},
    },
    config::BufferSize,
    engine::{Emitter, Engine, Handle},
    error::Error,
    utils::{fails, lifecycle},
    ToTween,
};

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

pub fn spawn(env: &SdkEnv) -> Result<(), Error> {
    lifecycle!("spawn plugin thread");
    STATE.store(Flags::LOADING.bits(), Ordering::Release);
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
    let engine = Engine::try_new(manager_settings)?;
    let _ = THREAD.set(Mutex::new(Some(std::thread::spawn(move || {
        lifecycle!("initialize channels...");
        let (sl, rl) = bounded::<Lifecycle>(32);
        let (sc, rc) = bounded::<Command>(capacity);
        let _ = LIFECYCLE.set(RwLock::new(Some(sl)));
        let _ = COMMAND.set(RwLock::new(Some(sc)));
        lifecycle!("initialized channels");
        self::run(rl, rc, engine);
    }))));
    lifecycle!("spawned plugin thread");
    Ok(())
}

pub fn run<B: Backend>(rl: Receiver<Lifecycle>, rc: Receiver<Command>, mut engine: Engine<B>) {
    use crate::states::State;
    let force_sync = false;
    let force_reclaim = false;
    let spoken = SpokenLocale::get();
    let gender = PlayerGender::get();
    'game: loop {
        let synced = false;
        let reclaimed = false;
        for l in rl.try_iter() {
            lifecycle!("> {l}");
            match l {
                Lifecycle::Terminate => {
                    break 'game;
                }
                Lifecycle::Shutdown => {}
                Lifecycle::RegisterEmitter { .. } => {}
                Lifecycle::UnregisterEmitter { .. } => {}
                Lifecycle::SyncScene => {}
                Lifecycle::Reclaim => {
                    // engine.reclaim();
                }
                Lifecycle::Session(Session::BeforeStart) => {
                    // engine.reset();
                }
                Lifecycle::Session(Session::Start) => {}
                Lifecycle::Session(Session::End) => {}
                Lifecycle::Session(Session::Ready) => {}
                Lifecycle::Session(Session::Pause) => {}
                Lifecycle::Session(Session::Resume) => {}
                Lifecycle::Session(Session::BeforeEnd) => {}
                Lifecycle::System(System::Attach) => {}
                Lifecycle::System(System::Detach) => {}
                Lifecycle::System(System::PlayerAttach) => {}
                Lifecycle::System(System::PlayerDetach) => {}
                Lifecycle::Board(Board::UIMenu(value)) => {}
                _ => {}
            }
        }
        for c in rc.try_iter().take(8) {
            lifecycle!("> {c:?}");
            match c {
                Command::PlayVanilla { .. } => {}
                Command::Play { .. } => {}
                Command::PlayExt {
                    sound_name,
                    entity_id,
                    emitter_name,
                    ..
                } => {
                    if let Ok(key) = engine.banks.try_get(&sound_name, &spoken, gender.as_ref()) {
                        let data = engine.banks.data(key);
                        let emitter = Emitter::new(entity_id, emitter_name);
                        match data {
                            Either::Left(data) => {
                                if let Ok(handle) = engine.manager.play(data) {
                                    engine.statics.insert(
                                        ProcessUniqueId::new(),
                                        Handle::new(handle, sound_name, emitter),
                                    );
                                }
                            }
                            Either::Right(data) => {
                                if let Ok(handle) = engine.manager.play(data) {
                                    engine.streams.insert(
                                        ProcessUniqueId::new(),
                                        Handle::new(handle, sound_name, emitter),
                                    );
                                }
                            }
                        }
                    }
                }
                Command::PlayOnEmitter { .. } => {}
                Command::PlayOverThePhone { .. } => {}
                Command::StopOnEmitter { .. } => {}
                Command::Pause { .. } => {}
                Command::Resume { .. } => {}
                Command::StopVanilla { .. } => {}
                Command::Stop {
                    event_name,
                    entity_id,
                    emitter_name,
                    tween,
                } => {
                    if engine.banks.exists(&event_name) {
                        let emitter = Emitter::new(entity_id, emitter_name);
                        for ref mut ref_multi in engine.statics.iter_mut() {
                            if ref_multi.value().event_name == event_name
                                && ref_multi.value().emitter == emitter
                            {
                                ref_multi
                                    .value_mut()
                                    .handle
                                    .stop(tween.clone().into_tween().unwrap_or_default());
                            }
                        }
                        for ref mut ref_multi in engine.streams.iter_mut() {
                            if ref_multi.value().event_name == event_name
                                && ref_multi.value().emitter == emitter
                            {
                                ref_multi
                                    .value_mut()
                                    .handle
                                    .stop(tween.clone().into_tween().unwrap_or_default());
                            }
                        }
                    }
                }
                Command::StopFor { .. } => {}
                Command::Switch { .. } => {}
                Command::SetVolume { .. } => {}
                Command::SetPreset { .. } => {}
                Command::SetReverbMix { .. } => {}
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
