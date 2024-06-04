use std::marker::PhantomData;

use audioware_sys::interop::entity::{Entity, ScriptedPuppet};
use audioware_sys::interop::game::get_game_instance;
use audioware_sys::interop::SafeDowncast;
use audioware_sys::interop::{entity::find_entity_by_id, gender::PlayerGender};
use either::Either;
use error::{BankRegistrySnafu, CannotFindEntitySnafu, Error};
use id::HandleId;
use kira::sound::{
    static_sound::{StaticSoundData, StaticSoundHandle},
    streaming::{StreamingSoundData, StreamingSoundHandle},
    FromFileError,
};
use kira::tween::Tween;
use manager::{audio_manager, maybe_statics, maybe_streams};
use red4ext_rs::types::{CName, EntityId, Ref};
use scene::Scene;
use snafu::{OptionExt, ResultExt};
use track::Tracks;

use crate::bank::{Banks, Id};
use crate::state::player::{gender, spoken_language};

mod destination;
mod effect;
pub mod error;
mod id;
mod manager;
mod scene;
mod track;
pub use manager::Manage;

pub struct Engine;
impl Engine {
    pub(crate) fn setup() -> Result<(), Error> {
        // SAFETY: initialization order matters
        Tracks::setup()?;
        Scene::setup()?;
        Ok(())
    }
    pub fn play(
        sound_name: &CName,
        entity_id: Option<&EntityId>,
        _emitter_name: Option<&CName>,
    ) -> Result<(), Error> {
        let locale = *spoken_language()
            .try_read()
            .map_err(crate::error::Error::from)?;
        let entity: Option<Ref<Entity>> = match entity_id {
            Some(entity_id) => Some(
                find_entity_by_id(get_game_instance(), entity_id.clone())
                    .into_ref()
                    .context(CannotFindEntitySnafu {
                        entity_id: entity_id.clone(),
                    })?,
            ),
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

        let id = Banks::exist(sound_name, &locale, gender.as_ref()).context(BankRegistrySnafu)?;
        let handle = Self::play_either(Banks::data(id))?;
        Self::store_either(id, entity_id, handle)?;
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

impl Manage for Engine {
    fn stop(&mut self, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, stop(tween));
        delegate_impl_manage!(maybe_streams, stop(tween));
    }

    fn stop_by_cname(&mut self, cname: &CName, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, stop_by_cname(cname, tween));
        delegate_impl_manage!(maybe_streams, stop_by_cname(cname, tween));
    }

    fn stop_by_cname_for_entity(
        &mut self,
        cname: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    ) {
        delegate_impl_manage!(
            maybe_statics,
            stop_by_cname_for_entity(cname, entity_id, tween)
        );
        delegate_impl_manage!(
            maybe_streams,
            stop_by_cname_for_entity(cname, entity_id, tween)
        );
    }

    fn pause(&mut self, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, pause(tween));
        delegate_impl_manage!(maybe_streams, pause(tween));
    }

    fn pause_by_cname(&mut self, cname: &CName, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, pause_by_cname(cname, tween));
        delegate_impl_manage!(maybe_streams, pause_by_cname(cname, tween));
    }

    fn pause_by_cname_for_entity(
        &mut self,
        cname: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    ) {
        delegate_impl_manage!(
            maybe_statics,
            pause_by_cname_for_entity(cname, entity_id, tween)
        );
        delegate_impl_manage!(
            maybe_streams,
            pause_by_cname_for_entity(cname, entity_id, tween)
        );
    }

    fn resume(&mut self, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, resume(tween));
        delegate_impl_manage!(maybe_streams, resume(tween));
    }

    fn resume_by_cname(&mut self, cname: &CName, tween: Option<Tween>) {
        delegate_impl_manage!(maybe_statics, resume_by_cname(cname, tween));
        delegate_impl_manage!(maybe_streams, resume_by_cname(cname, tween));
    }

    fn resume_by_cname_for_entity(
        &mut self,
        cname: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    ) {
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
