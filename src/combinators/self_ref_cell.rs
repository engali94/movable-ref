use crate::offset::Nullable;
use crate::{Offset, PointerRecomposition, SelfRef};

/// Container that provides safe access to a self-referenced value.
pub struct SelfRefCell<T: PointerRecomposition, I: Offset = isize> {
    value: T,
    ptr: SelfRef<T, I>,
}

impl<T: PointerRecomposition, I: Offset + Nullable> SelfRefCell<T, I> {
    /// Creates a new cell.
    ///
    /// # Parameters
    /// * `value` - Value to be owned by the cell and referenced internally.
    ///
    /// # Returns
    /// * `Result<Self, I::Error>` - `Ok` with an initialised cell, or the offset error when `I`
    ///   cannot encode the distance.
    pub fn new(value: T) -> Result<Self, I::Error> {
        let mut this = Self {
            value,
            ptr: SelfRef::null(),
        };
        this.ptr.set(&mut this.value)?;
        Ok(this)
    }

    /// Immutable access to the value.
    ///
    /// # Returns
    /// * `&T` - Shared reference to the stored value.
    pub fn get(&self) -> &T {
        self.try_get()
            .expect("SelfRefCell accessed before initialisation")
    }

    /// Immutable access to the value if the pointer has been initialised.
    ///
    /// # Returns
    /// * `Option<&T>` - Shared reference when the pointer is ready.
    #[inline]
    pub fn try_get(&self) -> Option<&T> {
        if !self.ptr.is_ready() {
            return None;
        }
        let base = self as *const _ as *const u8;
        Some(unsafe { self.ptr.get_ref_from_base_unchecked(base) })
    }

    /// Mutable access to the value.
    ///
    /// # Returns
    /// * `&mut T` - Exclusive reference to the stored value.
    pub fn get_mut(&mut self) -> &mut T {
        self.try_get_mut()
            .expect("SelfRefCell accessed before initialisation")
    }

    /// Mutable access to the value if the pointer has been initialised.
    ///
    /// # Returns
    /// * `Option<&mut T>` - Exclusive reference when the pointer is ready.
    #[inline]
    pub fn try_get_mut(&mut self) -> Option<&mut T> {
        if !self.ptr.is_ready() {
            return None;
        }
        let base = self as *mut _ as *mut u8;
        Some(unsafe { self.ptr.get_mut_from_base_unchecked(base) })
    }

    /// Consumes the cell and returns the value.
    ///
    /// # Returns
    /// * `T` - The owned value.
    pub fn into_inner(self) -> T {
        self.value
    }
}
