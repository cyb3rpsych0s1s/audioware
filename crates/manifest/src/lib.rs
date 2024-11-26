//! # Manifest
//!
//! This crate contains definitions for Audioware manifests.
//!
//! The manifests describe which audio file must be loaded, alongside their user-defined settings.

mod de;
mod depot;
pub mod error;
mod types;
pub use de::*;
pub use depot::*;
pub use types::*;
