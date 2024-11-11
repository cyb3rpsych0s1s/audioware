use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        LazyLock, OnceLock,
    },
    thread::JoinHandle,
};

use bitflags::bitflags;
use crossbeam::channel::{bounded, Receiver, Sender};
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
use std::sync::{Mutex, RwLock};

use crate::{
    abi::{
        command::Command,
        lifecycle::{Board, Lifecycle, Session, System},
    },
    config::BufferSize,
    engine::Engine,
    error::Error,
    utils::{fails, lifecycle},
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

pub fn run<B: Backend>(rl: Receiver<Lifecycle>, rc: Receiver<Command>, engine: Engine<B>) {
    let force_sync = false;
    let force_reclaim = false;
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
                    fails!("failed to notify plugin game loop: {e}");
                }
            }
        } else {
            fails!("plugin game channel is not initialized");
        }
    } else {
        fails!("plugin game loop is not initialized");
    }
}
