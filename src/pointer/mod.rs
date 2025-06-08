//! Pointer module - Core relative pointer functionality
//!
//! This module contains the main `SelfRef` type and all operations
//! related to relative pointer manipulation.

mod operations;
mod self_ref;
/// Module for handling unreachable code
pub mod unreachable;

pub use self_ref::SelfRef;
