use crate::{Offset, PointerRecomposition, SelfRef};
use crate::offset::Nullable;

/// Container that provides safe access to a self-referenced value.
pub struct SelfRefCell<T: PointerRecomposition, I: Offset = isize> {
    value: T,
    ptr: SelfRef<T, I>,
}

impl<T: PointerRecomposition, I: Offset + Nullable> SelfRefCell<T, I> {
/// Creates a new cell.
pub fn new(value: T) -> Result<Self, I::Error> {
        let mut this = Self { value, ptr: SelfRef::null() };
        this.ptr.set(&mut this.value)?;
        Ok(this)
    }

/// Immutable access to the value.
pub fn get(&self) -> &T {
        let base = self as *const _ as *const u8;
        unsafe { self.ptr.get_ref_from_base_unchecked(base) }
    }

/// Mutable access to the value.
pub fn get_mut(&mut self) -> &mut T {
        let base = self as *mut _ as *mut u8;
        unsafe { self.ptr.get_mut_from_base_unchecked(base) }
    }

/// Consumes the cell and returns the value.
pub fn into_inner(self) -> T {
        self.value
    }
}