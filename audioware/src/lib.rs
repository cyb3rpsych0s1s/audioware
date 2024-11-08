#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../README.md")]

use std::{sync::OnceLock, thread::JoinHandle};

use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, SpokenLocale};
use config::BufferSize;
use crossbeam::channel::{bounded, Sender, TryRecvError};
use engine::{Command, Engine, Lifecycle};
use kira::manager::backend::cpal::CpalBackendSettings;
use parking_lot::Mutex;
use red4ext_rs::{
    export_plugin_symbols, log,
    types::{CName, EntityId, Opt, Ref},
    wcstr, Plugin, PluginOps, SemVer, U16CStr,
};
use states::State;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

mod config;
mod engine;
mod error;
mod ext;
mod states;
mod types;
mod utils;

pub use types::*;

/// Audio [Plugin] for Cyberpunk 2077.
pub struct Audioware;

static THREAD: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();
static LIFECYCLE: OnceLock<parking_lot::RwLock<Option<Sender<Lifecycle>>>> = OnceLock::new();
static COMMAND: OnceLock<parking_lot::RwLock<Option<Sender<Command>>>> = OnceLock::new();

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    fn on_init(env: &red4ext_rs::SdkEnv) {
        log::info!(env, "Scoping audioware");
        let mut backend_settings = CpalBackendSettings::default();
        let buffer_size = BufferSize::read_ini();
        if buffer_size != BufferSize::Auto {
            backend_settings.buffer_size = cpal::BufferSize::Fixed(buffer_size as u32);
            log::info!(
                Audioware::env(),
                "buffer size read from .ini: {}",
                buffer_size as u32
            );
        }
        let handle = std::thread::spawn(move || {
            let env = Audioware::env();
            log::info!(env, "Spawning audioware");
            let banks = Banks::new();
            let Ok(mut engine) = Engine::try_new(buffer_size, banks) else {
                log::error!(env, "Failed to instantiate engine");
                return;
            };

            let (sl, rl) = bounded::<Lifecycle>(32);
            let (sc, rc) = bounded::<Command>(128);
            let _ = LIFECYCLE.set(parking_lot::RwLock::new(Some(sl)));
            let _ = COMMAND.set(parking_lot::RwLock::new(Some(sc)));
            let spoken = SpokenLocale::get();
            // let written = WrittenLocale::get();
            let gender = PlayerGender::get();
            loop {
                match rl.try_recv() {
                    Ok(Lifecycle::RegisterEmitter {
                        entity_id,
                        emitter_name,
                        emitter_settings,
                        sender,
                    }) => {
                        if let Err(e) = engine.scene.register_emitter(
                            entity_id,
                            emitter_name.into_option(),
                            emitter_settings.into_option().map(Into::into),
                        ) {
                            log::error!(env, "Failed to register emitter: {}", e);
                            if let Err(e) = sender.send(false) {
                                log::error!(env, "Failed to callback register emitter: {}", e);
                            }
                        } else if let Err(e) = sender.send(true) {
                            log::error!(env, "Failed to callback register emitter: {}", e);
                        }
                    }
                    Ok(Lifecycle::UnregisterEmitter { entity_id }) => {
                        if let Err(e) = engine.scene.unregister_emitter(&entity_id) {
                            log::error!(env, "Failed to unregister emitter: {}", e);
                        }
                    }
                    Ok(_) => {}
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                }
                match rc.try_recv() {
                    Ok(Command::PlayVanilla {
                        event_name,
                        entity_id,
                        emitter_name,
                    }) => {
                        if let Err(e) = engine.play(
                            event_name,
                            entity_id.into_option(),
                            emitter_name.into_option(),
                            spoken,
                            gender,
                        ) {
                            log::error!(env, "Failed to play: {}", e);
                        }
                    }
                    Ok(Command::Stop {
                        event_name,
                        entity_id,
                        emitter_name,
                        tween,
                    }) => {
                        if let Err(e) = engine.stop(
                            &event_name,
                            entity_id.into_option().as_ref(),
                            emitter_name.into_option().as_ref(),
                            Ref::from(tween).into_tween(),
                        ) {
                            log::error!(env, "Failed to stop: {}", e);
                        }
                    }
                    Ok(_) => {}
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                }
            }
        });
        let _ = THREAD.set(Mutex::new(Some(handle)));
        log::info!(env, "did not block");
    }
}

export_plugin_symbols!(Audioware);

#[allow(dead_code)]
fn register_emitter(
    entity_id: EntityId,
    emitter_name: Opt<CName>,
    emitter_settings: Opt<EmitterSettings>,
) -> bool {
    if let Some(x) = LIFECYCLE.get() {
        if let Some(Some(x)) = x.try_write().as_deref_mut() {
            let (sender, receiver) = bounded(0);
            if x.try_send(Lifecycle::RegisterEmitter {
                entity_id,
                emitter_name,
                emitter_settings,
                sender,
            })
            .is_ok()
            {
                let handle = std::thread::spawn(move || match receiver.recv() {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!(
                            Audioware::env(),
                            "failed to get register emitter callback response: {}",
                            e
                        );
                        return false;
                    }
                });
                match handle.join() {
                    Ok(x) => return x,
                    Err(_) => {
                        log::error!(
                            Audioware::env(),
                            "failed to join register emitter callback thread"
                        );
                    }
                }
            } else {
                log::error!(Audioware::env(), "failed to notify register emitter");
            }
        }
    }
    false
}
