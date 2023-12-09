use std::collections::VecDeque;

use either::Either;
use kira::{
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, PlaybackState},
    track::TrackHandle,
};

use crate::audio::StaticAudio;

pub(super) struct BoundedTrack {
    pub(super) track: TrackHandle,
    pub(super) current: Option<AnySound>,
    pub(super) queue: VecDeque<StaticAudio>,
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

pub(super) struct UnboundedTrack {
    pub(super) track: TrackHandle,
    pub(super) current: Vec<AnySound>,
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
pub(super) struct AnySound(Either<StaticSoundHandle, StreamingSoundHandle<anyhow::Error>>);

impl From<StaticSoundHandle> for AnySound {
    fn from(value: StaticSoundHandle) -> Self {
        Self(Either::Left(value))
    }
}

impl From<StreamingSoundHandle<anyhow::Error>> for AnySound {
    fn from(value: StreamingSoundHandle<anyhow::Error>) -> Self {
        Self(Either::Right(value))
    }
}

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
