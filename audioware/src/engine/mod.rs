use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex, OnceLock},
};

use anyhow::Context;

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::PlaybackState,
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle},
};
use lazy_static::lazy_static;
use red4ext_rs::types::EntityId;

use crate::{
    audio::{SoundId, StaticAudio},
    banks::BANKS,
};

mod collector;
mod manager;
mod player;
pub mod state;
mod track;
mod wrapper;

use player::PLAYER;

use self::{
    collector::Collector,
    player::PlayerTracks,
    state::{State, STATE},
    track::StatefulSound,
};

lazy_static! {
    static ref AUDIO: OnceLock<Mutex<Audioware>> = OnceLock::default();
}

pub(super) struct Audioware {
    manager: Arc<Mutex<AudioManager<DefaultBackend>>>,
    reverb: Arc<TrackHandle>,
    player: PlayerTracks,
    collector: Collector,
}

fn collect() {}

impl Audioware {
    pub(crate) fn update_engine_state(state: State) {
        if let Some(audioware) = AUDIO.get().and_then(|x| x.try_lock().ok()) {
            audioware.collector.unpark();
            STATE.store(state as u8, std::sync::atomic::Ordering::SeqCst);
        }
    }
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
        let audioware = Audioware {
            player: PlayerTracks::setup(&mut manager, &reverb)?,
            manager: Arc::new(Mutex::new(manager)),
            collector: Collector::new(Arc::new(collect)),
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
                                        vocal_guard.current = Some(sound.into());
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
