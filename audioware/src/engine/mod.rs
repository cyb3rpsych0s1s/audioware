use audioware_sys::interop::entity::Entity;
use audioware_sys::interop::game::ScriptedPuppet;
use audioware_sys::interop::gender::PlayerGender;
use audioware_sys::interop::locale::Locale;
use audioware_sys::interop::SafeDowncast;
use destination::output_destination;
use effect::IMMEDIATELY;
use either::Either;
use error::{BankRegistrySnafu, Error};
use id::{HandleId, SoundEntityId};
use kira::sound::{
    static_sound::{StaticSoundData, StaticSoundHandle},
    streaming::{StreamingSoundData, StreamingSoundHandle},
    FromFileError,
};
use kira::tween::Tween;
use manager::{audio_manager, audio_modulator, maybe_statics, maybe_streams};
use red4ext_rs::types::{CName, EntityId, Ref};
use scene::{maybe_scene_entities, Scene};
use snafu::{OptionExt, ResultExt};
use track::Tracks;

use crate::bank::{Banks, Id};
use crate::state::game::State;
use crate::state::player::{gender, spoken_language, written_language};

mod destination;
pub mod effect;
pub mod error;
mod id;
mod manager;
mod scene;
pub mod track;
pub use manager::Manage;

pub struct Engine;
impl Engine {
    pub(crate) fn setup() -> Result<(), Error> {
        // SAFETY: initialization order matters
        Tracks::setup()?;
        Scene::setup()?;
        Self::update_game_state(crate::state::game::State::Load);
        Ok(())
    }
    pub fn play(
        sound_name: &CName,
        entity_id: Option<&EntityId>,
        emitter_name: Option<&CName>,
    ) -> Result<(), Error> {
        let (gender, locale, _) = Self::get_player_states(entity_id)?;
        let id = Banks::exist(sound_name, &locale, gender.as_ref()).context(BankRegistrySnafu)?;
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
        let (gender, locale, _) = Self::get_player_states(Some(entity_id))?;
        let id = Banks::exist(sound_name, &locale, gender.as_ref()).context(BankRegistrySnafu)?;
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

    fn play_either(
        data: Either<StaticSoundData, StreamingSoundData<FromFileError>>,
    ) -> Result<Either<StaticSoundHandle, StreamingSoundHandle<FromFileError>>, Error> {
        let mut manager = audio_manager()
            .try_lock()
            .map_err(crate::error::Error::from)?;
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
        let previous = crate::state::game::State::set(state);
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

    pub fn update_modulator(value: f32) -> Result<bool, Error> {
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
        let mut modulator = audio_modulator()
            .try_lock()
            .map_err(crate::error::Error::from)?;
        modulator.set(value as f64, IMMEDIATELY);
        red4ext_rs::info!("update frequencies modulator: {value}");
        Ok(true)
    }

    fn get_player_states(
        entity_id: Option<&EntityId>,
    ) -> Result<(Option<PlayerGender>, Locale, Locale), Error> {
        let spoken = *spoken_language()
            .try_read()
            .map_err(crate::error::Error::from)?;
        let written = *written_language()
            .try_read()
            .map_err(crate::error::Error::from)?;
        let entity: Option<Ref<Entity>> = match entity_id {
            Some(entity_id) => Some((&SoundEntityId::from(entity_id)).try_into()?),
            None => None,
        };
        let gender: Option<PlayerGender> = match entity {
            Some(ref entity) => {
                if entity.is_player() {
                    Some(*gender().try_read().map_err(crate::error::Error::from)?)
                } else {
                    red4ext_rs::warn!("before entering safe downcast");
                    match SafeDowncast::<ScriptedPuppet>::maybe_downcast(entity) {
                        Some(puppet) if puppet.get_gender() == CName::new("female") => {
                            Some(PlayerGender::Female)
                        }
                        Some(puppet) if puppet.get_gender() == CName::new("male") => {
                            Some(PlayerGender::Male)
                        }
                        _ => None,
                    }
                }
            }
            None => None,
        };
        Ok((gender, spoken, written))
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
