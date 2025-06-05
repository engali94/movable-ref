//! Metadata handling for different types used in SelfRef.
//! 
//! This module provides metadata extraction and composition functionality for
//! both sized types and unsized types like slices and str.

/// Traits for metadata extraction and composition
pub mod traits;

/// Implementations of MetaData trait for various types
pub mod impls;

/// Trait object support for nightly Rust (requires ptr_metadata feature)
#[cfg(feature = "nightly")]
pub mod trait_object;

pub use traits::*;

#[cfg(feature = "nightly")]
pub use trait_object::*;
