use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, Mutex, OnceLock,
    },
    thread::JoinHandle,
    time::Duration,
};

use anyhow::Context;
use either::Either;

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, PlaybackState},
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle},
};
use lazy_static::lazy_static;
use red4ext_rs::types::EntityId;

use crate::{
    audio::{SoundId, StaticAudio},
    banks::BANKS,
};

mod manager;
mod player;
mod wrapper;

use player::PLAYER;

use self::player::PlayerTracks;

lazy_static! {
    static ref AUDIO: OnceLock<Mutex<Audioware>> = OnceLock::default();
}

#[repr(u8)]
enum State {
    Load = 0,
    Menu = 1,
    InGame = 2,
    Unload = 3,
}

pub(super) struct Audioware {
    manager: Arc<Mutex<AudioManager<DefaultBackend>>>,
    state: Arc<AtomicU8>,
    reverb: Arc<TrackHandle>,
    player: PlayerTracks,
    collector: Collector,
}

struct Collector(JoinHandle<()>);

impl Collector {
    fn new(flag: Arc<AtomicU8>, callback: Arc<fn() -> ()>) -> Self {
        Self(std::thread::spawn(move || 'thread: loop {
            match flag.load(Ordering::Relaxed) {
                a if a == State::Load as u8 || a == State::Menu as u8 => {
                    std::thread::park();
                }
                a if a == State::InGame as u8 => {
                    (callback.clone())();
                    std::thread::sleep(Duration::from_millis(250));
                }
                _ => {
                    break 'thread;
                }
            }
        }))
    }
}

struct BoundedTrack {
    track: TrackHandle,
    current: Option<AnySound>,
    queue: VecDeque<StaticAudio>,
}

impl std::fmt::Debug for BoundedTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundedTrack")
            .field("track", &self.track.id())
            .field("current", &self.current)
            .field("queue", &self.queue)
            .finish()
    }
}

struct UnboundedTrack {
    track: TrackHandle,
    current: Vec<AnySound>,
}

impl std::fmt::Debug for UnboundedTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnboundedTrack")
            .field("track", &self.track.id())
            .field("current", &self.current)
            .finish()
    }
}

#[repr(transparent)]
pub struct AnySound(Either<StaticSoundHandle, StreamingSoundHandle<anyhow::Error>>);

impl std::fmt::Debug for AnySound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Either::Left(_) => f.debug_tuple("AnySound:StaticSoundHandle").finish(),
            Either::Right(_) => f.debug_tuple("AnySound:StreamingSoundHandle").finish(),
        }
    }
}

pub trait StatefulSound {
    fn state(&self) -> PlaybackState;
    fn playing(&self) -> bool {
        self.state() != PlaybackState::Stopped
    }
}

impl StatefulSound for StaticSoundHandle {
    fn state(&self) -> PlaybackState {
        self.state()
    }
}

impl<E> StatefulSound for StreamingSoundHandle<E> {
    fn state(&self) -> PlaybackState {
        self.state()
    }
}

impl AnySound {
    pub fn get(&self) -> &dyn StatefulSound {
        match &self.0 {
            Either::Left(left) => left,
            Either::Right(right) => right,
        }
    }
}

impl StatefulSound for AnySound {
    fn state(&self) -> PlaybackState {
        self.get().state()
    }
}

fn collect() {}

impl Audioware {
    pub(crate) fn setup() -> anyhow::Result<()> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .context("unable to initialize audio backend")?;
        let reverb = manager
            .add_sub_track({
                let mut builder = TrackBuilder::new();
                builder.add_effect(ReverbBuilder::new().mix(1.0));
                builder
            })
            .context("unable to initialize reverb track")?;
        let reverb = Arc::new(reverb);
        let state = Arc::new(AtomicU8::new(State::Load as u8));
        let audioware = Audioware {
            player: PlayerTracks::setup(&mut manager, &reverb)?,
            manager: Arc::new(Mutex::new(manager)),
            collector: Collector::new(state.clone(), Arc::new(collect)),
            state,
            reverb,
        };
        let _ = AUDIO.set(Mutex::new(audioware));
        Ok(())
    }
    fn get_audio(key: impl Into<SoundId>) -> Option<StaticAudio> {
        BANKS
            .clone()
            .try_lock()
            .ok()
            .and_then(|x| x.get_sound(key))
            .map(|x| x.audio())
    }
    pub(crate) fn play(sound: impl Into<SoundId>) {
        if let Some(audio) = Self::get_audio(sound) {
            if let Some(tracks) = PLAYER.get() {
                if let Ok(mut vocal_guard) = tracks.vocal.clone().try_lock() {
                    match &vocal_guard.current {
                        Some(sound) if sound.state() != PlaybackState::Stopped => {
                            vocal_guard.queue.push_back(audio);
                        }
                        _ => {
                            if let Some(audioware_guard) =
                                AUDIO.get().and_then(|x| x.try_lock().ok())
                            {
                                if let Ok(mut manager) =
                                    audioware_guard.manager.clone().borrow_mut().try_lock()
                                {
                                    let data = audio.data.unwrap();
                                    data.settings.output_destination(&vocal_guard.track);
                                    if let Ok(sound) = manager.play(data) {
                                        vocal_guard.current = Some(AnySound(Either::Left(sound)));
                                    }
                                }
                            }
                            // if let Ok(mut audio_guard) = AUDIO.clone().borrow_mut().try_lock() {
                            //     if let Some(manager) = audio_guard.0.as_mut() {
                            //         let data = audio.data.unwrap();
                            //         data.settings.output_destination(&vocal_guard.track);
                            //         if let Ok(sound) = manager.play(data) {
                            //             vocal_guard.current = Some(AnySound(Either::Left(sound)));
                            //         }
                            //     }
                            // }
                        }
                    };
                }
            }
        }
    }
}

pub trait IsPlayer {
    fn is_player(&self) -> bool;
}

impl IsPlayer for EntityId {
    fn is_player(&self) -> bool {
        self == &EntityId::from(1)
    }
}
