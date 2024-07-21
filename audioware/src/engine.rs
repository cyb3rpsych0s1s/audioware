use manager::Manager;
use scene::Scene;
use tracks::Tracks;

use crate::error::Error;

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
}
