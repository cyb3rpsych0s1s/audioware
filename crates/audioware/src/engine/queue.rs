use std::{sync::OnceLock, thread::JoinHandle, time::Duration};

use bitflags::bitflags;
use crossbeam::channel::{Receiver, Sender, bounded, tick};
use kira::{
    AudioManagerSettings,
    backend::cpal::{CpalBackend, CpalBackendSettings},
};
use red4ext_rs::{
    SdkEnv,
    log::{self},
    types::CName,
};
use std::sync::{Mutex, RwLock};

use crate::{
    abi::{
        command::Command,
        is_in_foreground,
        lifecycle::{Board, Lifecycle, Session, System},
    },
    config::BufferSize,
    engine::{DilationUpdate, tweens::DILATION_EASE_OUT},
    error::Error,
    utils::{fails, lifecycle},
};

use super::Engine;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Flags: u8 {
        const LOADING = 1 << 0;
        const IN_MENU = 1 << 1;
        const IN_GAME = 1 << 2;
        const PAUSED  = 1 << 3;
        const FOCUSED = 1 << 4;
        const MUTE_IN_BACKGROUND = 1 << 5;
    }
}

impl Flags {
    fn should_sync(&self) -> bool {
        self.contains(Flags::IN_GAME)
            && !self.intersects(Flags::LOADING | Flags::IN_MENU | Flags::PAUSED)
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let yes = "+";
        let no = "-";
        write!(
            f,
            "[LOADING: {}, IN_MENU: {}, IN_GAME: {}, PAUSED: {}, SHOULD_SYNC: {}]",
            if self.contains(Self::LOADING) {
                yes
            } else {
                no
            },
            if self.contains(Self::IN_MENU) {
                yes
            } else {
                no
            },
            if self.contains(Self::IN_GAME) {
                yes
            } else {
                no
            },
            if self.contains(Self::PAUSED) { yes } else { no },
            if self.should_sync() { yes } else { no },
        )
    }
}

static THREAD: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();
static LIFECYCLE: OnceLock<RwLock<Option<Sender<Lifecycle>>>> = OnceLock::new();
static COMMAND: OnceLock<RwLock<Option<Sender<Command>>>> = OnceLock::new();

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
    let capacity = manager_settings.capacities.sub_track_capacity;
    Ok((Engine::try_new(manager_settings)?, capacity))
}

pub fn spawn(env: &SdkEnv) -> Result<(), Error> {
    lifecycle!("spawn plugin thread");
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
    crate::utils::lifecycle!("run engine thread");
    let s = |x| Duration::from_secs_f32(x);
    let ms = |x| Duration::from_millis(x);
    let reclamation = tick(s(if cfg!(debug_assertions) { 3. } else { 60. }));
    let synchronization = tick(ms(15));
    let mut state = Flags::LOADING | Flags::MUTE_IN_BACKGROUND;
    'game: loop {
        if state.contains(Flags::MUTE_IN_BACKGROUND) {
            if !is_in_foreground() {
                if state.contains(Flags::FOCUSED) {
                    state.set(Flags::FOCUSED, false);
                    engine.mute(true);
                }
            } else if !state.contains(Flags::FOCUSED) {
                state.set(Flags::FOCUSED, true);
                engine.mute(false);
            }
        }
        for l in rl.try_iter() {
            lifecycle!("> {l}");
            match l {
                Lifecycle::Terminate => {
                    engine.tracks.clear();
                    break 'game;
                }
                Lifecycle::ReportInitialization => engine.report_initialization(false),
                #[cfg(feature = "hot-reload")]
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
                Lifecycle::RegisterEmitter {
                    entity_id,
                    tag_name,
                    emitter_name,
                    emitter_settings,
                    sender,
                } => {
                    let registered = engine.register_emitter(
                        *entity_id,
                        *tag_name,
                        emitter_name,
                        emitter_settings.as_deref(),
                    );
                    let _ = sender.try_send(registered);
                }
                Lifecycle::UnregisterEmitter {
                    entity_id,
                    tag_name,
                    sender,
                } => {
                    let unregistered = engine.unregister_emitter(*entity_id, *tag_name);
                    let _ = sender.try_send(unregistered);
                }
                Lifecycle::OnEmitterDies { entity_id } => engine.on_emitter_dies(entity_id),
                Lifecycle::OnEmitterIncapacitated { entity_id } => {
                    engine.on_emitter_incapacitated(entity_id)
                }
                Lifecycle::OnEmitterDefeated { .. } => {}
                Lifecycle::SetVolume { setting, value } => engine.set_volume(setting, value),
                Lifecycle::SetMuteInBackground { value } => {
                    if value != state.contains(Flags::MUTE_IN_BACKGROUND) {
                        state.set(Flags::MUTE_IN_BACKGROUND, value);
                    }
                }
                Lifecycle::Session(Session::BeforeStart) => engine.reset(),
                Lifecycle::Session(Session::Start) => {
                    state.set(Flags::LOADING, true);
                }
                Lifecycle::Session(Session::End) => {}
                Lifecycle::Session(Session::BeforeEnd) => {
                    if state.contains(Flags::IN_GAME) {
                        state.set(Flags::IN_GAME, false);
                        engine.scene = None;
                        engine.tracks.clear();
                    }
                }
                Lifecycle::UIInGameNotificationRemove => {
                    if state.contains(Flags::LOADING) {
                        engine.tracks.stop(DILATION_EASE_OUT);
                    }
                }
                Lifecycle::Session(Session::Ready) => {
                    state.set(Flags::LOADING, false);
                    state.set(Flags::IN_MENU, false);
                    state.set(Flags::IN_GAME, true);
                }
                Lifecycle::Session(Session::Pause) => {
                    state.set(Flags::PAUSED, true);
                }
                Lifecycle::Session(Session::Resume) => {
                    state.set(Flags::PAUSED, false);
                }
                Lifecycle::SwitchToScenario(name) => {
                    if name == CName::new("MenuScenario_PauseMenu") {
                        engine.pause();
                    }
                    state.set(Flags::IN_MENU, true);
                }
                Lifecycle::EngagementScreen => {
                    state.set(Flags::IN_GAME, false);
                }
                Lifecycle::System(System::Attach) | Lifecycle::System(System::Detach) => {}
                Lifecycle::System(System::PlayerAttach) => {
                    if let Err(e) = engine.try_new_scene() {
                        lifecycle!("failed to create new scene: {e}");
                    }
                }
                Lifecycle::System(System::PlayerDetach) => engine.stop_scene_emitters_and_actors(),
                Lifecycle::Board(Board::UIMenu(opened)) => {
                    state.set(Flags::IN_MENU, opened);
                    if state.contains(Flags::IN_GAME) {
                        if opened {
                            engine.pause();
                        } else {
                            engine.resume();
                        }
                    }
                }
                Lifecycle::Board(Board::ReverbMix(value)) => engine.set_reverb_mix(value),
                Lifecycle::Board(Board::Preset(value)) => engine.set_preset(value),
            }
        }
        if state.should_sync()
            && (engine.any_emitter() || engine.any_actor())
            && synchronization.try_recv().is_ok()
        {
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
                } => engine.play::<kira::Tween>(event_name, entity_id, emitter_name, None, None),
                Command::Play {
                    event_name: sound_name,
                    entity_id,
                    emitter_name,
                    ext,
                    line_type,
                } => engine.play(sound_name, entity_id, emitter_name, ext, line_type),
                Command::PlayOnEmitter {
                    event_name,
                    entity_id,
                    tag_name,
                    ext,
                } => engine.play_on_emitter(event_name, *entity_id, *tag_name, ext),
                Command::PlayOverThePhone {
                    event_name,
                    emitter_name,
                    gender,
                } => engine.play_over_the_phone(event_name, emitter_name, gender),
                Command::PlaySceneDialog {
                    string_id,
                    entity_id,
                    is_player,
                    is_holocall,
                    is_rewind,
                    seek_time,
                } => engine.play_scene_dialog(
                    string_id,
                    entity_id,
                    is_player,
                    is_holocall,
                    is_rewind,
                    seek_time,
                ),
                Command::StopSceneDialog {
                    string_id,
                    fade_out,
                } => engine.stop_on_actor(string_id, fade_out),
                Command::StopOnEmitter {
                    event_name,
                    entity_id,
                    tag_name,
                    tween,
                } => engine.stop_on_emitter(event_name, *entity_id, *tag_name, tween),
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
                Command::Switch {
                    switch_name,
                    switch_value,
                    entity_id,
                    emitter_name,
                    switch_name_tween,
                    switch_value_settings,
                } => engine.switch(
                    switch_name,
                    switch_value,
                    entity_id,
                    emitter_name,
                    switch_name_tween,
                    switch_value_settings,
                ),
                Command::SwitchVanilla {
                    switch_name,
                    switch_value,
                    entity_id,
                    emitter_name,
                } => engine.switch::<kira::Tween>(
                    switch_name,
                    switch_value,
                    entity_id,
                    emitter_name,
                    None,
                    None,
                ),
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
            if let Some(x) = x.as_ref()
                && let Err(e) = x.try_send(lifecycle)
            {
                fails!("failed to notify plugin lifecycle: {e}");
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
            if let Some(x) = x.as_ref()
                && let Err(e) = x.try_send(command)
            {
                fails!("failed to send plugin command: {e}");
            }
        } else {
            fails!("plugin command channel is not initialized");
        }
    } else {
        fails!("plugin game loop is not initialized");
    }
}
