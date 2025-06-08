//! Pointer operations
//!
//! This module contains the operations for the `SelfRef` type.

use super::self_ref::SelfRef;
use crate::metadata::PointerRecomposition;
use crate::offset::Offset;
use std::fmt::*;

impl<T: ?Sized + PointerRecomposition, I: Debug + Offset> Pointer for SelfRef<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:p}", self as *const Self)
    }
}

impl<T: ?Sized + PointerRecomposition, I: Debug + Offset> Debug for SelfRef<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("SelfRef")
            .field("ptr", &(self as *const Self))
            .finish()
    }
}
