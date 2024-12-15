use audioware_manifest::PlayerGender;
use dashmap::DashSet;
use kira::{
    sound::{
        static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError,
        PlaybackState,
    },
    spatial::emitter::{EmitterDistances, EmitterHandle},
};
use red4ext_rs::types::{CName, EntityId, GameInstance};

use crate::{
    engine::tweens::IMMEDIATELY,
    error::{Error, SceneError},
    AIActionHelper, AsEntity, AsGameInstance, AsTimeDilatable, GameObject, TimeDilatable, Vector4,
};

use super::super::{dilation::Dilation, AsEntityExt};

#[derive(Debug)]
pub struct Emitter {
    pub handles: Handles,
    pub handle: EmitterHandle,
    pub sharers: DashSet<CName>,
    pub dilation: Dilation,
    pub last_known_position: Vector4,
    pub busy: bool,
    pub persist_until_sounds_finishes: bool,
    pub marked_for_death: bool,
    #[allow(dead_code, reason = "todo")]
    pub gender: Option<PlayerGender>,
}

impl Emitter {
    pub fn store_static(
        &mut self,
        event_name: CName,
        handle: StaticSoundHandle,
        affected_by_time_dilation: bool,
    ) {
        self.handles.statics.push(Handle {
            event_name,
            handle,
            affected_by_time_dilation,
        });
    }
    pub fn store_stream(
        &mut self,
        event_name: CName,
        handle: StreamingSoundHandle<FromFileError>,
        affected_by_time_dilation: bool,
    ) {
        self.handles.streams.push(Handle {
            event_name,
            handle,
            affected_by_time_dilation,
        });
    }
    pub fn any_playing_handle(&self) -> bool {
        self.handles
            .statics
            .iter()
            .any(|x| x.handle.state() == PlaybackState::Playing)
            || self
                .handles
                .streams
                .iter()
                .any(|x| x.handle.state() == PlaybackState::Playing)
    }
}

impl AsRef<EmitterHandle> for Emitter {
    fn as_ref(&self) -> &EmitterHandle {
        &self.handle
    }
}

#[derive(Debug)]
pub struct Handle<T> {
    pub event_name: CName,
    pub handle: T,
    pub affected_by_time_dilation: bool,
}

#[derive(Debug, Default)]
pub struct Handles {
    pub statics: Vec<Handle<StaticSoundHandle>>,
    pub streams: Vec<Handle<StreamingSoundHandle<FromFileError>>>,
}

impl Drop for Handles {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.iter_mut().for_each(|x| {
            x.handle.stop(IMMEDIATELY);
        });
    }
}

impl Emitter {
    pub fn infos(entity_id: EntityId) -> Result<(Vector4, bool), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let busy = entity
            .clone()
            .cast::<GameObject>()
            .map(AIActionHelper::is_in_workspot)
            .unwrap_or(false);
        let position = entity.get_world_position();
        Ok((position, busy))
    }

    #[allow(clippy::type_complexity)]
    pub fn full_infos(
        entity_id: EntityId,
    ) -> Result<
        (
            Option<PlayerGender>,
            Vector4,
            bool,
            Option<f32>,
            Option<EmitterDistances>,
        ),
        Error,
    > {
        let (position, busy) = Self::infos(entity_id)?;
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let gender = entity.get_gender();
        let distances = entity.get_emitter_distances();
        if !entity.is_a::<TimeDilatable>() {
            return Ok((gender, position, busy, None, distances));
        }
        let dilation = entity
            .clone()
            .cast::<TimeDilatable>()
            .as_ref()
            .map(AsTimeDilatable::get_time_dilation_value);
        Ok((gender, position, busy, dilation, distances))
    }
}
