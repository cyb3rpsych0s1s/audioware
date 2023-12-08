use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    sync::{Arc, Mutex, OnceLock},
};

use either::Either;

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, PlaybackState, Sound,
    },
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle, TrackRoutes},
};
use lazy_static::lazy_static;
use red4ext_rs::types::EntityId;

use crate::{
    audio::{SoundId, StaticAudio},
    banks::BANKS,
    Audioware,
};

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

struct PlayerTracks {
    vocal: Arc<Mutex<BoundedTrack>>,
    mental: Arc<Mutex<UnboundedTrack>>,
    emissive: Arc<Mutex<UnboundedTrack>>,
    parent: TrackHandle,
}

impl std::fmt::Debug for PlayerTracks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerTracks")
            .field("vocal", &self.vocal)
            .field("mental", &self.mental)
            .field("emissive", &self.emissive)
            .field("parent", &self.parent.id())
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

lazy_static! {
    static ref AUDIO: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
    static ref PLAYER: OnceLock<PlayerTracks> = OnceLock::default();
    // static ref TRACKS: Arc<Mutex<HashMap<String, TrackHandle>>> =
    //     Arc::new(Mutex::new(HashMap::default()));
    // static ref SCENE: Arc<Mutex<Option<SpatialSceneHandle>>> = Arc::new(Mutex::new(None));
}

impl Audioware {
    fn setup() -> anyhow::Result<AudioManager> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let reverb = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().mix(1.0));
            builder
        })?;
        // let ambience = manager.add_sub_track(
        //     TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.5)),
        // )?;
        let player = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.25)),
        )?;
        let vocal = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&player, 1.)),
        )?;
        let mental = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&player, 1.)),
        )?;
        let emissive = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(&player, 1.)),
        )?;
        PLAYER
            .set(PlayerTracks {
                vocal: Arc::new(Mutex::new(BoundedTrack {
                    track: vocal,
                    current: None,
                    queue: VecDeque::with_capacity(3),
                })),
                mental: Arc::new(Mutex::new(UnboundedTrack {
                    track: mental,
                    current: Vec::with_capacity(64),
                })),
                emissive: Arc::new(Mutex::new(UnboundedTrack {
                    track: emissive,
                    current: Vec::with_capacity(64),
                })),
                parent: player,
            })
            .unwrap();
        // let environment = manager.add_spatial_scene(SpatialSceneSettings::default())?;

        // if let Ok(mut guard) = TRACKS.clone().borrow_mut().try_lock() {
        //     guard.insert("ambience".to_string(), ambience);
        //     guard.insert("player".to_string(), player);
        //     guard.insert("player:vocal".to_string(), vocal);
        //     guard.insert("player:emissive".to_string(), emissive);
        //     guard.insert("player:mental".to_string(), mental);
        // }
        // if let Ok(mut guard) = SCENE.clone().borrow_mut().try_lock() {
        //     *guard = Some(environment);
        // }
        Ok(manager)
    }
    pub(crate) fn create() -> anyhow::Result<()> {
        match Self::setup() {
            Ok(manager) => match AUDIO.clone().borrow_mut().try_lock() {
                Ok(mut guard) => {
                    *guard = Self(Some(manager));
                    Ok(())
                }
                Err(_) => anyhow::bail!("unable to store audioware's handle"),
            },
            Err(_) => anyhow::bail!("unable to setup audioware's audio engine"),
        }
    }
    pub(crate) fn destroy() -> anyhow::Result<()> {
        match AUDIO.clone().borrow_mut().try_lock() {
            Ok(mut guard) => {
                *guard = Self(None);
                Ok(())
            }
            Err(_) => anyhow::bail!("unable to destroy audioware"),
        }
    }
    fn get_audio(key: impl Into<SoundId>) -> Option<StaticAudio> {
        BANKS
            .clone()
            .try_lock()
            .ok()
            .and_then(|x| x.get_sound(key))
            .map(|x| x.audio())
    }
    pub(crate) fn play(&mut self, sound: impl Into<SoundId>) {
        if let Some(audio) = Self::get_audio(sound) {
            if let Some(tracks) = PLAYER.get() {
                if let Ok(mut vocal_guard) = tracks.vocal.clone().try_lock() {
                    if let Some(sound) = &vocal_guard.current {
                        if sound.playing() {
                            vocal_guard.queue.push_back(audio);
                        } else if let Ok(mut audio_guard) = AUDIO.clone().borrow_mut().try_lock() {
                            if let Some(manager) = audio_guard.0.as_mut() {
                                let data = audio.data.unwrap();
                                if let Ok(mut guard) = tracks.vocal.clone().try_lock() {
                                    data.settings.output_destination(&guard.track);
                                    if let Ok(sound) = manager.play(data) {
                                        guard.current = Some(AnySound(Either::Left(sound)));
                                    }
                                }
                            }
                        }
                    }
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
