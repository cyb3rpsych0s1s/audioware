use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, OnceLock},
};

use kira::{
    manager::AudioManager,
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};
use lazy_static::lazy_static;

use super::{BoundedTrack, UnboundedTrack};

lazy_static! {
    pub(crate) static ref PLAYER: OnceLock<PlayerTracks> = OnceLock::default();
}

pub(crate) struct PlayerTracks {
    pub(crate) vocal: Arc<Mutex<BoundedTrack>>,
    pub(crate) mental: Arc<Mutex<UnboundedTrack>>,
    pub(crate) emissive: Arc<Mutex<UnboundedTrack>>,
    pub(crate) parent: TrackHandle,
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

impl PlayerTracks {
    pub(crate) fn setup(manager: &mut AudioManager, reverb: &TrackHandle) -> anyhow::Result<()> {
        let player = manager.add_sub_track(
            TrackBuilder::new().routes(TrackRoutes::new().with_route(reverb, 0.25)),
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
        let _ = PLAYER.set(PlayerTracks {
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
        });
        Ok(())
    }
}
