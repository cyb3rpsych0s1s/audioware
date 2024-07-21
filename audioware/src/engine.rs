use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, ScnDialogLineType, SpokenLocale, WrittenLocale};
use manager::Manager;
use red4ext_rs::types::{CName, EntityId};
use scene::Scene;
use tracks::Tracks;

use crate::{error::Error, states::State};

mod eq;
mod id;
mod manager;
pub mod modulators;
mod scene;
mod tracks;

pub struct Engine;

impl Engine {
    pub fn setup() -> Result<(), Error> {
        // SAFETY: initialization order matters
        let mut manager = Manager::try_lock()?;
        Tracks::setup(&mut manager)?;
        Scene::setup(&mut manager)?;
        Ok(())
    }
    /// play sound
    pub fn play(
        sound_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
    ) -> Result<(), Error> {
        let manager = Manager::try_lock()?;
        let spoken = SpokenLocale::get();
        let written = WrittenLocale::get();
        let gender = PlayerGender::get();
        let id = Banks::exist(&sound_name, &spoken, gender.as_ref());
        // let mut data = match Banks::data(&sound_name) {
        //     Ok(data) => data,
        //     Err(_) => {
        //         // #[rustfmt::skip]
        //         // red4ext_rs::warn!("{}", RegistryError::NotFound { id: sound_name.clone() });
        //         // return;
        //     }
        // };
        Ok(())
    }
}
