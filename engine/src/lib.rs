use audioware_core::{SpokenLocale, WrittenLocale};
use audioware_sys::interop::entity::Entity;
use audioware_sys::interop::gender::PlayerGender;
use destination::output_destination;
use effect::IMMEDIATELY;
use either::Either;
use id::{EmitterId, HandleId};
use kira::track::TrackBuilder;
use kira::tween::Tween;
use kira::OutputDestination;
use kira::{
    sound::{
        static_sound::{StaticSoundData, StaticSoundHandle},
        streaming::{StreamingSoundData, StreamingSoundHandle},
        FromFileError,
    },
    Volume,
};
use manager::{audio_manager, maybe_statics, maybe_streams};
use modulator::Parameter;
use red4ext_rs::types::{CName, EntityId, Ref};
use scene::{maybe_scene_entities, Scene};
use snafu::ResultExt;
use track::{maybe_custom_tracks, maybe_tracks, TrackName, Tracks};

use audioware_bank::{Banks, Id};

mod destination;
pub mod effect;
mod error;
pub use error::*;
mod id;
mod manager;
mod modulator;
mod scene;
pub mod track;
pub use manager::Manage;
mod state;
pub use modulator::GlobalParameter;
pub use modulator::GlobalParameters;
pub use modulator::VolumeModulator;
pub use state::game::*;
pub use state::player::*;

pub struct Engine;
impl Engine {
    pub fn setup() -> Result<(), Error> {
        let mut manager = audio_manager().lock()?;
        // SAFETY: initialization order matters
        Tracks::setup(&mut manager)?;
        Scene::setup(&mut manager)?;
        Self::update_game_state(State::Load);
        Ok(())
    }

    pub fn another_play(
        sound_name: &CName,
        destination: Either<CName, EntityId>,
        emitter_name: Option<&CName>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn play(
        sound_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
    ) -> Result<(), Error> {
        let (gender, spoken, _) = Self::get_player_states(entity_id)?;
        let id = Banks::exist(sound_name, &spoken, gender.as_ref()).context(BankRegistrySnafu)?;
        let destination = output_destination(entity_id, emitter_name, false)?;
        let data = Banks::data(id).map_either(
            |x| x.output_destination(destination),
            |x| x.output_destination(destination),
        );
        let handle = Self::play_either(data)?;
        Self::store_either(id, entity_id, handle)?;
        Ok(())
    }

    pub fn play_on_emitter(
        sound_name: &CName,
        entity_id: &EntityId,
        _emitter_name: &CName,
    ) -> Result<(), Error> {
        let (gender, spoken, _) = Self::get_player_states(Some(entity_id))?;
        let id = Banks::exist(sound_name, &spoken, gender.as_ref()).context(BankRegistrySnafu)?;
        let data = Banks::data(id);
        let emitters = maybe_scene_entities()?;
        if let Some(emitter) = emitters.get(&entity_id.into()) {
            let mut manager = audio_manager()
                .try_lock()
                .map_err(|e| Error::Internal { source: e.into() })?;
            let handle = match data {
                Either::Left(x) => {
                    let handle = manager.play(x.output_destination(emitter))?;
                    Either::Left(handle)
                }
                Either::Right(x) => {
                    let handle = manager.play(x.output_destination(emitter))?;
                    Either::Right(handle)
                }
            };
            Self::store_either(id, Some(entity_id), handle)?;
        }
        Ok(())
    }

    pub fn play_on_track(
        sound_name: &CName,
        track_name: &TrackName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
        tween: Option<Tween>,
    ) -> Result<(), Error> {
        let output_destination = match track_name {
            x if x == &CName::new("Audioware:V:vocal") => {
                Some(OutputDestination::from(&maybe_tracks()?.v.vocal))
            }
            x if x == &CName::new("Audioware:V:environmental") => {
                Some(OutputDestination::from(&maybe_tracks()?.v.environmental))
            }
            x => {
                let tracks = maybe_custom_tracks().try_lock()?;
                tracks.get(x).map(OutputDestination::from)
            }
        };
        todo!()
    }

    fn play_either(
        data: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> Result<Either<StaticSoundHandle, StreamingSoundHandle<FromFileError>>, Error> {
        let mut manager = audio_manager()
            .try_lock()
            .map_err(audioware_core::Error::from)?;
        match data {
            Either::Left(data) => Ok(Either::Left(manager.play(data)?)),
            Either::Right(data) => Ok(Either::Right(manager.play(data)?)),
        }
    }
    fn store_either(
        id: &Id,
        owner: Option<&EntityId>,
        handle: Either<StaticSoundHandle, StreamingSoundHandle<FromFileError>>,
    ) -> Result<(), Error> {
        match handle {
            Either::Left(handle) => {
                let mut statics = maybe_statics()?;
                statics.insert(HandleId::new(id, owner.cloned()), handle);
            }
            Either::Right(handle) => {
                let mut streams = maybe_streams()?;
                streams.insert(HandleId::new(id, owner.cloned()), handle);
            }
        }
        Ok(())
    }

    /// on specific state changes sounds will also be paused, resumed or stopped.
    pub fn update_game_state(state: State) {
        let previous = State::set(state);
        #[cfg(debug_assertions)]
        if previous != state {
            red4ext_rs::info!("updated game state from {previous} to {state}");
        }
        match (previous, state) {
            (State::InGame, State::InMenu | State::InPause) => {
                Self::pause(None);
            }
            (State::InMenu | State::InPause, State::InGame) => {
                Self::resume(None);
            }
            (_, State::Unload | State::End) => {
                Self::stop(None);
            }
            _ => {}
        }
    }

    pub fn update_volume(value: f32) -> Result<bool, Error> {
        if value < 0. {
            return Err(Error::InvalidModulatorValue {
                value,
                reason: "modulator value must be greater than 0.0",
            });
        }
        if value > 100. {
            return Err(Error::InvalidModulatorValue {
                value,
                reason: "modulator value must be lower or equals to 100.0",
            });
        }
        VolumeModulator::update(Volume::Amplitude(value as f64), IMMEDIATELY)?;
        red4ext_rs::info!("update frequencies modulator: {value}");
        Ok(true)
    }

    fn get_player_states(
        entity_id: Option<&EntityId>,
    ) -> Result<(Option<PlayerGender>, SpokenLocale, WrittenLocale), Error> {
        let spoken = *spoken_language()
            .try_read()
            .map_err(audioware_core::Error::from)?;
        let written = *written_language()
            .try_read()
            .map_err(audioware_core::Error::from)?;
        let entity: Option<Ref<Entity>> = match entity_id {
            Some(entity_id) => Some((&EmitterId::from(entity_id)).try_into()?),
            None => None,
        };
        let gender: Option<PlayerGender> = match entity {
            Some(ref entity) => {
                if entity.is_player() {
                    Some(*gender().try_read().map_err(audioware_core::Error::from)?)
                } else {
                    let gender = entity.get_template_gender();
                    match gender {
                        x if x == CName::new("Female") => Some(PlayerGender::Female),
                        x if x == CName::new("Male") => Some(PlayerGender::Male),
                        _ => None,
                    }
                }
            }
            None => None,
        };
        Ok((gender, spoken, written))
    }
    pub fn add_sub_track(name: &TrackName) -> Result<(), Error> {
        let mut manager = audio_manager().try_lock().map_err(Error::from)?;
        let mut tracks = maybe_custom_tracks().try_lock().map_err(Error::from)?;
        let handle = manager.add_sub_track(TrackBuilder::new())?;
        tracks.insert(name.clone(), handle);
        Ok(())
    }
    pub fn remove_sub_track(name: &TrackName) -> Result<(), Error> {
        let mut tracks = maybe_custom_tracks().try_lock().map_err(Error::from)?;
        tracks.remove(name);
        Ok(())
    }
}

macro_rules! delegate_impl_manage {
    ($storage:ident, $action:ident( $( $args:tt )* )) => {
        match $storage() {
            Ok(mut x) => x.$action( $( $args )* ),
            Err(e) => {
                ::red4ext_rs::error!("{e}");
            },
        };
    };
}

impl Engine {
    pub fn stop(tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, stop(tween));
        delegate_impl_manage!(maybe_streams, stop(tween));
    }

    pub fn stop_by_cname(cname: &CName, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, stop_by_cname(cname, tween));
        delegate_impl_manage!(maybe_streams, stop_by_cname(cname, tween));
    }

    pub fn stop_by_cname_for_entity(cname: &CName, entity_id: &EntityId, tween: Option<Tween>) {
        delegate_impl_manage!(
            maybe_statics,
            stop_by_cname_for_entity(cname, entity_id, tween)
        );
        delegate_impl_manage!(
            maybe_streams,
            stop_by_cname_for_entity(cname, entity_id, tween)
        );
    }

    pub fn pause(tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, pause(tween));
        delegate_impl_manage!(maybe_streams, pause(tween));
    }

    pub fn pause_by_cname(cname: &CName, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, pause_by_cname(cname, tween));
        delegate_impl_manage!(maybe_streams, pause_by_cname(cname, tween));
    }

    pub fn pause_by_cname_for_entity(cname: &CName, entity_id: &EntityId, tween: Option<Tween>) {
        delegate_impl_manage!(
            maybe_statics,
            pause_by_cname_for_entity(cname, entity_id, tween)
        );
        delegate_impl_manage!(
            maybe_streams,
            pause_by_cname_for_entity(cname, entity_id, tween)
        );
    }

    pub fn resume(tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, resume(tween));
        delegate_impl_manage!(maybe_streams, resume(tween));
    }

    pub fn resume_by_cname(cname: &CName, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, resume_by_cname(cname, tween));
        delegate_impl_manage!(maybe_streams, resume_by_cname(cname, tween));
    }

    pub fn resume_by_cname_for_entity(cname: &CName, entity_id: &EntityId, tween: Option<Tween>) {
        delegate_impl_manage!(
            maybe_statics,
            resume_by_cname_for_entity(cname, entity_id, tween)
        );
        delegate_impl_manage!(
            maybe_streams,
            resume_by_cname_for_entity(cname, entity_id, tween)
        );
    }
}
