use std::{hash::Hash, num::NonZero, ops::Deref};

use kira::spatial::emitter::EmitterSettings;
use red4ext_rs::types::{CName, EntityId, GameInstance};

use crate::{error::ValidationError, get_player, AsEntity, Entity};

#[derive(Debug, Clone, Copy)]
pub struct TagName(Valid<CName>);

impl TagName {
    pub fn try_new(value: CName) -> Result<Self, ValidationError> {
        match value.as_str() {
            "" | "None" => Err(ValidationError::InvalidTagName),
            _ => Ok(Self(Valid(value))),
        }
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for TagName {
    type Target = CName;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TargetId(Valid<EntityId>);

impl TargetId {
    pub fn try_new(value: EntityId) -> Result<Self, ValidationError> {
        if !value.is_defined()
            || get_player(GameInstance::new())
                .cast::<Entity>()
                .expect("PlayerPuppet inherits from Entity")
                .get_entity_id()
                == value
        {
            return Err(ValidationError::InvalidTargetId);
        }
        Ok(Self(Valid(value)))
    }
}

impl Deref for TargetId {
    type Target = EntityId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TargetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct TargetFootprint(Valid<(EmitterSettings, NonZero<u64>)>);

impl TargetFootprint {
    pub fn try_new(value: (EmitterSettings, NonZero<u64>)) -> Result<Self, Vec<ValidationError>> {
        // value.0.validate()?;
        Ok(Self(Valid(value)))
    }
}

impl Deref for TargetFootprint {
    type Target = (EmitterSettings, NonZero<u64>);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Valid<T>(T);
impl<T> Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Hash for Valid<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<T> Clone for Valid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Valid(self.0.clone())
    }
}
impl<T> Copy for Valid<T> where T: Copy {}

pub struct PartiallyValid<T>(T);
impl<T> Deref for PartiallyValid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Hash for PartiallyValid<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<T> Clone for PartiallyValid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        PartiallyValid(self.0.clone())
    }
}
impl<T> Copy for PartiallyValid<T> where T: Copy {}
