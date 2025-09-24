//! SelfRef type definition
//!
//! This module contains the main `SelfRef` type that represents a relative pointer.

use crate::metadata::PointerRecomposition;
use crate::offset::{Nullable, Offset, Ptr};
use crate::pointer::unreachable::UncheckedOptionExt as _;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use std::ptr::NonNull;

/// It is always safe to cast between a
/// `Option<NonNull<T>>` and a `*mut T`
/// because they are the exact same in memory
#[inline(always)]
fn nn_to_ptr<T: ?Sized>(nn: Ptr<T>) -> *mut T {
    unsafe { std::mem::transmute(nn) }
}

/// A pointer that stores offsets instead of addresses, enabling movable self-referential structures.
///
/// Unlike regular pointers that become invalid when data moves, `SelfRef` stores the relative
/// distance to its target. This offset remains valid regardless of where the containing structure
/// is moved in memory - stack, heap, or anywhere else.
///
/// The magic happens through the offset type `I`: use `i8` for tiny 1-byte pointers with ±127 byte
/// range, `i16` for 2-byte pointers with ±32KB range, or larger types for bigger structures.
///
/// ```rust
/// use movable_ref::SelfRef;
///
/// struct Node {
///     value: String,
///     self_ref: SelfRef<String, i16>,  // 2 bytes instead of 8
/// }
///
/// impl Node {
///     fn new(value: String) -> Self {
///         let mut node = Self {
///             value,
///             self_ref: SelfRef::null(),
///         };
///         node.self_ref.set(&mut node.value).unwrap();
///         node
///     }
/// }
///
/// // Works everywhere - stack, heap, vectors
/// let mut node = Node::new("test".into());
/// let boxed = Box::new(node);
/// let mut vec = vec![*boxed];
/// let base = &vec[0] as *const _ as *const u8;
/// let value = unsafe { vec[0].self_ref.get_ref_from_base_unchecked(base) };
/// ```
///
/// # Safety Considerations
///
/// `SelfRef` uses `unsafe` internally but provides safe setup methods. The main safety requirement
/// is that once set, the relative positions of the pointer and target must not change. Moving
/// the entire structure is always safe - it's only internal layout changes that cause issues.
///
/// Special care needed with packed structs: field reordering during drops can invalidate offsets.
pub struct SelfRef<T: ?Sized + PointerRecomposition, I: Offset = isize>(
    I,
    MaybeUninit<T::Components>,
    PhantomData<*mut T>,
    #[cfg(miri)] MaybeUninit<Ptr<T>>,
);

// Ergonomics and ptr like impls

impl<T: ?Sized + PointerRecomposition, I: Offset> Copy for SelfRef<T, I> {}
impl<T: ?Sized + PointerRecomposition, I: Offset> Clone for SelfRef<T, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + PointerRecomposition, I: Offset> Eq for SelfRef<T, I> {}
impl<T: ?Sized + PointerRecomposition, I: Offset> PartialEq for SelfRef<T, I> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

/// Convert an offset into a `SelfRef`
impl<T: ?Sized + PointerRecomposition, I: Offset> From<I> for SelfRef<T, I> {
    fn from(i: I) -> Self {
        Self(
            i,
            MaybeUninit::uninit(),
            PhantomData,
            #[cfg(miri)]
            MaybeUninit::uninit(),
        )
    }
}

impl<T: ?Sized + PointerRecomposition, I: Nullable> SelfRef<T, I> {
    /// Creates an unset relative pointer.
    ///
    /// This is the starting point for most `SelfRef` usage - create a null pointer,
    /// then use `set()` to point it at your target data.
    #[inline(always)]
    pub fn null() -> Self {
        Self(
            I::NULL,
            MaybeUninit::uninit(),
            PhantomData,
            #[cfg(miri)]
            MaybeUninit::uninit(),
        )
    }

    /// Checks if the pointer is unset.
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0 == I::NULL
    }
}

impl<T: ?Sized + PointerRecomposition, I: Offset> SelfRef<T, I> {
    /// Sets the pointer to target the given value.
    ///
    /// Computes the offset from this `SelfRef`'s location to the target value.
    /// Returns an error if the distance is too large for the offset type `I`.
    ///
    /// This is the safe way to establish the self-reference - it validates that
    /// the offset fits before storing it.
    ///
    /// ```rust
    /// use movable_ref::SelfRef;
    /// let mut data = "hello".to_string();
    /// let mut ptr: SelfRef<String, i16> = SelfRef::null();
    /// ptr.set(&mut data).unwrap();  // Now points to data
    /// ```
    #[inline]
    pub fn set(&mut self, value: &mut T) -> Result<(), I::Error> {
        self.0 = I::sub(value as *mut T as _, self as *mut Self as _)?;
        self.1 = MaybeUninit::new(T::decompose(value));
        #[cfg(miri)]
        {
            self.3 = MaybeUninit::new(Some(NonNull::from(value)));
        }

        Ok(())
    }

    /// Sets the pointer without bounds checking.
    ///
    /// Like `set()` but assumes the offset will fit in type `I`. Used when you've
    /// already validated the distance or are reconstructing a known-good pointer.
    ///
    /// # Safety
    ///
    /// The offset between `value` and `self` must be representable in `I`.
    /// `value` must not be null.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, value: *mut T) {
        self.0 = I::sub_unchecked(value as _, self as *mut Self as _);
        self.1 = MaybeUninit::new(T::decompose(&*value));
        #[cfg(miri)]
        {
            self.3 = MaybeUninit::new(NonNull::new(value).map(|p| p));
        }
    }

    /// Reconstructs the target pointer without null checking.
    ///
    /// # Safety
    ///
    /// The pointer must have been successfully set and the relative positions
    /// of the pointer and target must not have changed since setting.
    #[inline]
    unsafe fn as_raw_unchecked_impl(&mut self) -> *mut T {
        #[cfg(miri)]
        {
            let base = self as *mut Self as *const u8;
            let addr = self.0.add(base).addr();
            let exposed = addr as usize as *mut u8;
            return nn_to_ptr(T::recompose(NonNull::new(exposed), self.1.assume_init()));
        }
        nn_to_ptr(T::recompose(
            NonNull::new(self.0.add(self as *mut Self as *mut u8)),
            self.1.assume_init(),
        ))
    }

    /// Reconstructs the target as a mutable raw pointer.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`.
    #[inline]
    pub unsafe fn as_raw_unchecked(&mut self) -> *mut T {
        self.as_raw_unchecked_impl()
    }

    /// Reconstructs the target as a `NonNull` pointer.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`.
    #[inline]
    pub unsafe fn as_non_null_unchecked(&mut self) -> NonNull<T> {
        #[cfg(miri)]
        {
            let base = self as *mut Self as *const u8;
            let addr = self.0.add(base).addr();
            let exposed = addr as usize as *mut u8;
            return T::recompose(NonNull::new(exposed), self.1.assume_init()).unchecked_unwrap(
                "Tried to use an unset relative pointer, this is UB in release mode!",
            );
        }
        T::recompose(
            NonNull::new(self.0.add(self as *mut Self as *mut u8)),
            self.1.assume_init(),
        )
        .unchecked_unwrap("Tried to use an unset relative pointer, this is UB in release mode!")
    }

    /// Reconstructs the target as an immutable reference.
    ///
    /// This is the most common way to access your self-referenced data.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`. Standard reference aliasing rules apply.
    #[inline]
    pub unsafe fn as_ref_unchecked(&mut self) -> &T {
        &*self.as_raw_unchecked_impl()
    }

    /// Reconstructs a shared reference using a container base pointer.
    #[inline]
    pub unsafe fn get_ref_from_base_unchecked<'a>(&self, base: *const u8) -> &'a T {
        let base_ptr = base as *const u8;
        let self_ptr = self as *const Self as *const u8;
        let d_self = self_ptr.offset_from(base_ptr);
        let at_self = base_ptr.wrapping_offset(d_self);
        let p = nn_to_ptr(T::recompose(
            NonNull::new(self.0.add(at_self)),
            self.1.assume_init(),
        ));
        &*p
    }

    /// Reconstructs a mutable reference using a container base pointer.
    #[inline]
    pub unsafe fn get_mut_from_base_unchecked<'a>(&self, base: *mut u8) -> &'a mut T {
        let base_ptr = base as *const u8;
        let self_ptr = self as *const Self as *const u8;
        let d_self = self_ptr.offset_from(base_ptr);
        let at_self = base_ptr.wrapping_offset(d_self);
        let p = nn_to_ptr(T::recompose(
            NonNull::new(self.0.add(at_self)),
            self.1.assume_init(),
        ));
        &mut *p
    }

    /// Reconstructs the target as a mutable reference.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`. Standard reference aliasing rules apply.
    #[inline]
    pub unsafe fn as_mut_unchecked(&mut self) -> &mut T {
        &mut *self.as_raw_unchecked()
    }
}

macro_rules! as_non_null_impl {
    ($self:ident) => {
        if $self.is_null() {
            None
        } else {
            T::recompose(
                NonNull::new($self.0.add($self as *const Self as *const u8)),
                $self.1.assume_init(),
            )
        }
    };
}

impl<T: ?Sized + PointerRecomposition, I: Nullable> SelfRef<T, I> {
    /// Reconstructs the target as a raw pointer, returning null if unset.
    ///
    /// # Safety
    ///
    /// If the pointer was set, the relative positions must not have changed.
    /// For most pointer types this is safe, but may be undefined behavior
    /// for some exotic pointer representations.
    #[inline]
    pub unsafe fn as_raw(&mut self) -> *mut T {
        nn_to_ptr(self.as_non_null())
    }

    /// Reconstructs the target as a `NonNull` pointer, returning `None` if unset.
    ///
    /// # Safety
    ///
    /// If the pointer was set, the relative positions must not have changed.
    #[inline]
    pub unsafe fn as_non_null(&mut self) -> Ptr<T> {
        #[cfg(miri)]
        {
            let base = self as *mut Self as *const u8;
            let addr = self.0.add(base).addr();
            let exposed = addr as usize as *mut u8;
            return T::recompose(NonNull::new(exposed), self.1.assume_init());
        }
        as_non_null_impl!(self)
    }

    /// Reconstructs the target as an immutable reference, returning `None` if unset.
    ///
    /// # Safety
    ///
    /// Standard reference aliasing rules apply. If the pointer was set,
    /// the relative positions must not have changed.
    #[inline]
    pub unsafe fn as_ref(&mut self) -> Option<&T> {
        Some(&*as_non_null_impl!(self)?.as_ptr())
    }

    /// Reconstructs the target as a mutable reference, returning `None` if unset.
    ///
    /// # Safety
    ///
    /// Standard reference aliasing rules apply. If the pointer was set,
    /// the relative positions must not have changed.
    #[inline]
    pub unsafe fn as_mut(&mut self) -> Option<&mut T> {
        Some(&mut *self.as_non_null()?.as_ptr())
    }
}
