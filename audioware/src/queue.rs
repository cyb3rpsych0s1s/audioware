use std::{sync::OnceLock, thread::JoinHandle};

use crossbeam::channel::{bounded, Receiver, Sender, TryRecvError};
use kira::manager::{
    backend::{
        cpal::{CpalBackend, CpalBackendSettings},
        Backend,
    },
    AudioManagerSettings,
};
use red4ext_rs::{
    log::{self, error},
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
    utils::lifecycle,
    Audioware,
};

static THREAD: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();
static LIFECYCLE: OnceLock<RwLock<Option<Sender<Lifecycle>>>> = OnceLock::new();
static COMMAND: OnceLock<RwLock<Option<Sender<Command>>>> = OnceLock::new();

pub fn spawn(_env: &SdkEnv) -> Result<(), Error> {
    lifecycle!("spawn plugin thread");
    let buffer_size = BufferSize::read_ini();
    let mut backend_settings = CpalBackendSettings::default();
    if buffer_size != BufferSize::Auto {
        backend_settings.buffer_size = cpal::BufferSize::Fixed(buffer_size as u32);
        log::info!(
            Audioware::env(),
            "buffer size read from .ini: {}",
            buffer_size as u32
        );
    }
    let manager_settings = AudioManagerSettings::<CpalBackend> {
        backend_settings,
        ..Default::default()
    };
    let engine = Engine::try_new(manager_settings)?;
    let _ = THREAD.set(Mutex::new(Some(std::thread::spawn(move || {
        lifecycle!("initialize channels...");
        let (sl, rl) = bounded::<Lifecycle>(32);
        let (sc, rc) = bounded::<Command>(128);
        let _ = LIFECYCLE.set(RwLock::new(Some(sl)));
        let _ = COMMAND.set(RwLock::new(Some(sc)));
        lifecycle!("initialized channels");
        self::run(rl, rc, engine);
    }))));
    lifecycle!("spawned plugin thread");
    Ok(())
}

pub fn run<B: Backend>(rl: Receiver<Lifecycle>, rc: Receiver<Command>, engine: Engine<B>) {
    'game: loop {
        match rl.try_recv() {
            Ok(x) => match x {
                Lifecycle::Terminate => {
                    lifecycle!("> terminate");
                    break 'game;
                }
                Lifecycle::Shutdown => {
                    lifecycle!("> shutdown");
                }
                Lifecycle::RegisterEmitter { .. } => {}
                Lifecycle::UnregisterEmitter { .. } => {}
                Lifecycle::SyncScene => {}
                Lifecycle::Reclaim => {
                    lifecycle!("> reclaim");
                    // engine.reclaim();
                }
                Lifecycle::Session(Session::BeforeStart) => {
                    lifecycle!("> session before start");
                    // engine.reset();
                }
                Lifecycle::Session(Session::Start) => {
                    lifecycle!("> session start")
                }
                Lifecycle::Session(Session::End) => {
                    lifecycle!("> session end")
                }
                Lifecycle::Session(Session::Ready) => {
                    lifecycle!("> session ready")
                }
                Lifecycle::Session(Session::Pause) => {
                    lifecycle!("> session pause")
                }
                Lifecycle::Session(Session::Resume) => {
                    lifecycle!("> session resume")
                }
                Lifecycle::Session(Session::BeforeEnd) => {
                    lifecycle!("> session before end")
                }
                Lifecycle::System(System::Attach) => {
                    lifecycle!("> system attach")
                }
                Lifecycle::System(System::Detach) => {
                    lifecycle!("> system detach")
                }
                Lifecycle::System(System::PlayerAttach) => {
                    lifecycle!("> system player attach")
                }
                Lifecycle::System(System::PlayerDetach) => {
                    lifecycle!("> system player detach")
                }
                Lifecycle::Board(Board::UIMenu(value)) => {
                    lifecycle!("> board ui menu {value}");
                }
            },
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                error!(
                    Audioware::env(),
                    "plugin game lifecycle channel is disconnected"
                );
                break 'game;
            }
        }
        match rc.try_recv() {
            Ok(x) => {}
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                error!(
                    Audioware::env(),
                    "plugin game commands channel is disconnected"
                );
                break 'game;
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
                    error!(Audioware::env(), "failed to notify plugin game loop: {}", e);
                }
            }
        } else {
            error!(Audioware::env(), "plugin game channel is not initialized");
        }
    } else {
        error!(Audioware::env(), "plugin game loop is not initialized");
    }
}
