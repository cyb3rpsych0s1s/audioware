use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use either::Either;

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, PlaybackState, Sound,
    },
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle},
};
use lazy_static::lazy_static;
use red4ext_rs::types::EntityId;

use crate::{
    audio::{SoundId, StaticAudio},
    banks::BANKS,
    Audioware,
};

mod player;

use player::PLAYER;

use self::player::PlayerTracks;

lazy_static! {
    static ref AUDIO: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
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
        PlayerTracks::setup(&mut manager, &reverb)
            .context("unable to initialize player soundtracks")?;
        *AUDIO
            .clone()
            .borrow_mut()
            .try_lock()
            .map_err(|_| anyhow::anyhow!("setup: unable to reach audio backend"))? =
            Self(Some(manager));
        Ok(())
    }
    pub(crate) fn teardown() -> anyhow::Result<()> {
        AUDIO
            .clone()
            .borrow_mut()
            .try_lock()
            .map_err(|_| anyhow::anyhow!("teardown: unable to reach audio backend"))?
            .unload();
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
                            if let Ok(mut audio_guard) = AUDIO.clone().borrow_mut().try_lock() {
                                if let Some(manager) = audio_guard.0.as_mut() {
                                    let data = audio.data.unwrap();
                                    data.settings.output_destination(&vocal_guard.track);
                                    if let Ok(sound) = manager.play(data) {
                                        vocal_guard.current = Some(AnySound(Either::Left(sound)));
                                    }
                                }
                            }
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
