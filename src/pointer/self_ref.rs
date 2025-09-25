//! SelfRef type definition
//!
//! This module contains the main `SelfRef` type that represents a relative pointer.

use crate::metadata::PointerRecomposition;
use crate::offset::{Nullable, Offset, Ptr};
use crate::pointer::unreachable::UncheckedOptionExt as _;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use std::ptr::NonNull;

type GuardPayload<T> = Option<NonNull<T>>;

#[inline]
fn guard_payload_from<T: ?Sized>(target: Option<NonNull<T>>) -> GuardPayload<T> {
    #[cfg(feature = "debug-guards")]
    {
        target
    }
    #[cfg(not(feature = "debug-guards"))]
    {
        let _ = target;
        None
    }
}

#[inline]
fn guard_payload_empty<T: ?Sized>() -> GuardPayload<T> {
    guard_payload_from::<T>(None)
}

#[inline]
fn guard_extract_target<T: ?Sized>(payload: GuardPayload<T>) -> Option<NonNull<T>> {
    #[cfg(feature = "debug-guards")]
    {
        payload
    }
    #[cfg(not(feature = "debug-guards"))]
    {
        let _ = payload;
        None
    }
}

#[inline]
fn guard_assert_target<T: ?Sized>(payload: GuardPayload<T>, target: *mut u8) {
    #[cfg(feature = "debug-guards")]
    {
        if let Some(expected) = payload {
            debug_assert_eq!(expected.as_ptr() as *mut u8, target);
        }
    }
    #[cfg(not(feature = "debug-guards"))]
    {
        let _ = (payload, target);
    }
}

enum RefState<T: ?Sized> {
    Unset,
    Ready(GuardPayload<T>),
}

impl<T: ?Sized> Copy for RefState<T> {}

impl<T: ?Sized> Clone for RefState<T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// It is always safe to cast between a
/// `Option<NonNull<T>>` and a `*mut T`
/// because they are the exact same in memory
#[inline(always)]
fn nn_to_ptr<T: ?Sized>(nn: Ptr<T>) -> *mut T {
    unsafe { core::mem::transmute(nn) }
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
    RefState<T>,
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
        match (self.components_if_ready(), other.components_if_ready()) {
            (None, None) => true,
            (Some(lhs), Some(rhs)) => self.0 == other.0 && lhs == rhs,
            _ => false,
        }
    }
}

impl<T: ?Sized + PointerRecomposition, I: Nullable> SelfRef<T, I> {
    /// Creates an unset relative pointer.
    ///
    /// This is the starting point for most `SelfRef` usage - create a null pointer,
    /// then use `set()` to point it at your target data.
    ///
    /// # Returns
    /// * `SelfRef<T, I>` - Pointer that must be initialised before use.
    #[inline(always)]
    pub fn null() -> Self {
        Self(I::NULL, MaybeUninit::uninit(), PhantomData, RefState::Unset)
    }

    /// Checks if the pointer is unset.
    ///
    /// # Returns
    /// * `bool` - `true` when the pointer has not been initialised.
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0 == I::NULL
    }
}

impl<T: ?Sized + PointerRecomposition, I: Offset> SelfRef<T, I> {
    /// Returns `true` once the pointer metadata has been populated.
    ///
    /// # Returns
    /// * `bool` - `true` when initialisation has completed.
    #[inline]
    pub fn is_ready(&self) -> bool {
        matches!(self.3, RefState::Ready(_))
    }

    /// Provides the stored metadata when the pointer is initialised.
    ///
    /// # Returns
    /// * `Option<T::Components>` - Metadata captured during initialisation.
    #[inline]
    pub fn components_if_ready(&self) -> Option<T::Components> {
        match self.3 {
            RefState::Ready(_) => Some(unsafe { self.components_unchecked() }),
            RefState::Unset => None,
        }
    }

    #[inline]
    unsafe fn components_unchecked(&self) -> T::Components {
        *self.1.assume_init_ref()
    }

    /// Returns the raw distance recorded for this pointer.
    ///
    /// # Returns
    /// * `I` - Offset measured from this pointer to the target.
    #[inline]
    pub fn offset(&self) -> I {
        self.0
    }

    /// Reconstructs a relative pointer from previously captured parts.
    ///
    /// # Parameters
    /// * `offset` - Relative distance between pointer and target when captured.
    /// * `components` - Metadata produced by [`PointerRecomposition::decompose`].
    ///
    /// # Returns
    /// * `SelfRef<T, I>` - Pointer ready to be used at the current location.
    #[inline]
    pub fn from_parts(offset: I, components: T::Components) -> Self {
        Self(
            offset,
            MaybeUninit::new(components),
            PhantomData,
            RefState::Ready(guard_payload_empty::<T>()),
        )
    }

    /// Reconstructs a relative pointer and optionally tracks a known absolute target.
    ///
    /// The recorded pointer is only meaningful while the container remains at the
    /// same address; moves invalidate the stored absolute pointer and trigger debug
    /// assertions when the pointer is dereferenced.
    ///
    /// # Parameters
    /// * `offset` - Relative distance between pointer and target when captured.
    /// * `components` - Metadata produced by [`PointerRecomposition::decompose`].
    /// * `target` - Optional absolute pointer retained for debug verification.
    ///
    /// # Returns
    /// * `SelfRef<T, I>` - Pointer configured with optional debug metadata.
    #[inline]
    pub fn from_parts_with_target(
        offset: I,
        components: T::Components,
        target: Option<NonNull<T>>,
    ) -> Self {
        Self(
            offset,
            MaybeUninit::new(components),
            PhantomData,
            RefState::Ready(guard_payload_from::<T>(target)),
        )
    }

    /// Returns the stored offset and metadata when initialised.
    ///
    /// # Returns
    /// * `Option<(I, T::Components)>` - Offset and metadata if the pointer is ready.
    #[inline]
    pub fn parts_if_ready(&self) -> Option<(I, T::Components)> {
        self.components_if_ready()
            .map(|components| (self.0, components))
    }

    /// Returns offset, metadata, and any recorded absolute pointer when initialised.
    ///
    /// # Returns
    /// * `Option<(I, T::Components, Option<NonNull<T>>)>` - Captured parts used for reconstruction
    ///   along with the optional debug target.
    #[inline]
    pub fn parts_with_target_if_ready(&self) -> Option<(I, T::Components, Option<NonNull<T>>)> {
        self.components_if_ready().map(|components| match self.3 {
            RefState::Ready(payload) => (self.0, components, guard_extract_target::<T>(payload)),
            RefState::Unset => unreachable!(),
        })
    }

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
    ///
    /// # Parameters
    /// * `value` - Target to be referenced by the pointer.
    ///
    /// # Returns
    /// * `Result<(), I::Error>` - `Ok` when the offset fits in `I`, otherwise the conversion error.
    #[inline]
    pub fn set(&mut self, value: &mut T) -> Result<(), I::Error> {
        self.0 = I::sub(value as *mut T as _, self as *mut Self as _)?;
        self.1 = MaybeUninit::new(T::decompose(value));
        self.3 = RefState::Ready(guard_payload_empty::<T>());

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
    ///
    /// # Parameters
    /// * `value` - Raw pointer to the target value.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, value: *mut T) {
        debug_assert!(!value.is_null());
        self.0 = I::sub_unchecked(value as _, self as *mut Self as _);
        self.1 = MaybeUninit::new(T::decompose(&*value));
        self.3 = RefState::Ready(guard_payload_empty::<T>());
    }

    /// Reconstructs the target pointer without null checking.
    ///
    /// # Safety
    ///
    /// The pointer must have been successfully set and the relative positions
    /// of the pointer and target must not have changed since setting.
    ///
    /// # Returns
    /// * `*mut T` - Raw pointer to the target.
    #[inline]
    unsafe fn as_raw_unchecked_impl(&mut self) -> *mut T {
        debug_assert!(self.is_ready());
        let base = self as *mut Self as *const u8;
        let target = self.0.add(base);
        let components = unsafe { self.components_unchecked() };
        nn_to_ptr(T::recompose(NonNull::new(target), components))
    }

    /// Reconstructs the target as a mutable raw pointer.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`.
    ///
    /// # Returns
    /// * `*mut T` - Raw pointer to the target.
    #[inline]
    pub unsafe fn as_raw_unchecked(&mut self) -> *mut T {
        self.as_raw_unchecked_impl()
    }

    /// Reconstructs the target as a `NonNull` pointer.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`.
    ///
    /// # Returns
    /// * `NonNull<T>` - Guaranteed non-null pointer to the target.
    #[inline]
    pub unsafe fn as_non_null_unchecked(&mut self) -> NonNull<T> {
        debug_assert!(self.is_ready());
        let base = self as *mut Self as *const u8;
        let target = self.0.add(base);
        let components = unsafe { self.components_unchecked() };
        if let RefState::Ready(payload) = self.3 {
            guard_assert_target::<T>(payload, target);
        }
        T::recompose(NonNull::new(target), components)
            .unchecked_unwrap("Tried to use an unset relative pointer, this is UB in release mode!")
    }

    /// Reconstructs the target as an immutable reference.
    ///
    /// This is the most common way to access your self-referenced data.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`. Standard reference aliasing rules apply.
    ///
    /// # Returns
    /// * `&T` - Shared reference to the target.
    #[inline]
    pub unsafe fn as_ref_unchecked(&mut self) -> &T {
        &*self.as_raw_unchecked_impl()
    }

    /// Reconstructs a shared reference using a container base pointer.
    ///
    /// # Safety
    ///
    /// * `base` must be the start address of the object that currently contains `self`.
    /// * The pointer must have been established with `set` and the relative positions must
    ///   remain unchanged.
    /// * No mutable reference to the target may exist for the lifetime of the returned reference.
    ///
    /// # Parameters
    /// * `base` - Address of the owning container currently holding the pointer.
    ///
    /// # Returns
    /// * `&'a T` - Shared reference resolved relative to `base`.
    #[inline]
    pub unsafe fn get_ref_from_base_unchecked<'a>(&self, base: *const u8) -> &'a T {
        debug_assert!(self.is_ready());
        let self_ptr = self as *const Self as *const u8;
        let d_self = self_ptr.offset_from(base);
        let at_self = base.wrapping_offset(d_self);
        let components = unsafe { self.components_unchecked() };
        let target = self.0.add(at_self);
        if let RefState::Ready(payload) = self.3 {
            guard_assert_target::<T>(payload, target);
        }
        let p = nn_to_ptr(T::recompose(NonNull::new(target), components));
        &*p
    }

    /// Reconstructs a mutable reference using a container base pointer.
    ///
    /// # Safety
    ///
    /// * `base` must point to the start of the object that currently contains `self`.
    /// * The pointer must have been initialised with `set` and the relative positions must
    ///   remain unchanged.
    /// * The caller must guarantee unique access to the target for the lifetime of the
    ///   returned reference.
    ///
    /// # Parameters
    /// * `base` - Address of the owning container currently holding the pointer.
    ///
    /// # Returns
    /// * `&'a mut T` - Exclusive reference resolved relative to `base`.
    #[inline]
    pub unsafe fn get_mut_from_base_unchecked<'a>(&self, base: *mut u8) -> &'a mut T {
        debug_assert!(self.is_ready());
        let base_ptr = base.cast_const();
        let self_ptr = self as *const Self as *const u8;
        let d_self = self_ptr.offset_from(base_ptr);
        let at_self = base_ptr.wrapping_offset(d_self);
        let components = unsafe { self.components_unchecked() };
        let target = self.0.add(at_self);
        if let RefState::Ready(payload) = self.3 {
            guard_assert_target::<T>(payload, target);
        }
        let p = nn_to_ptr(T::recompose(NonNull::new(target), components));
        &mut *p
    }

    /// Reconstructs the target as a mutable reference.
    ///
    /// # Safety
    ///
    /// Same as `as_raw_unchecked_impl`. Standard reference aliasing rules apply.
    ///
    /// # Returns
    /// * `&mut T` - Exclusive reference to the target.
    #[inline]
    pub unsafe fn as_mut_unchecked(&mut self) -> &mut T {
        &mut *self.as_raw_unchecked()
    }
}

impl<T: ?Sized + PointerRecomposition, I: Nullable> SelfRef<T, I> {
    /// Reconstructs the target as a raw pointer, returning null if unset.
    ///
    /// # Safety
    ///
    /// If the pointer was set, the relative positions must not have changed.
    /// For most pointer types this is safe, but may be undefined behavior
    /// for some exotic pointer representations.
    ///
    /// # Returns
    /// * `*mut T` - Raw pointer to the target or null when unset.
    #[inline]
    pub unsafe fn as_raw(&mut self) -> *mut T {
        nn_to_ptr(self.as_non_null())
    }

    /// Reconstructs the target as a `NonNull` pointer, returning `None` if unset.
    ///
    /// # Safety
    ///
    /// If the pointer was set, the relative positions must not have changed.
    ///
    /// # Returns
    /// * `Option<NonNull<T>>` - Non-null pointer when initialised.
    #[inline]
    pub unsafe fn as_non_null(&mut self) -> Ptr<T> {
        if !self.is_ready() {
            return None;
        }
        let base = self as *mut Self as *const u8;
        let target = self.0.add(base);
        let components = unsafe { self.components_unchecked() };
        if let RefState::Ready(payload) = self.3 {
            guard_assert_target::<T>(payload, target);
        }
        T::recompose(NonNull::new(target), components)
    }

    /// Reconstructs the target as an immutable reference, returning `None` if unset.
    ///
    /// # Safety
    ///
    /// Standard reference aliasing rules apply. If the pointer was set,
    /// the relative positions must not have changed.
    ///
    /// # Returns
    /// * `Option<&T>` - Shared reference when initialised.
    #[inline]
    pub unsafe fn as_ref(&mut self) -> Option<&T> {
        self.as_non_null().map(|ptr| unsafe { &*ptr.as_ptr() })
    }

    /// Reconstructs the target as a mutable reference, returning `None` if unset.
    ///
    /// # Safety
    ///
    /// Standard reference aliasing rules apply. If the pointer was set,
    /// the relative positions must not have changed.
    ///
    /// # Returns
    /// * `Option<&mut T>` - Exclusive reference when initialised.
    #[inline]
    pub unsafe fn as_mut(&mut self) -> Option<&mut T> {
        self.as_non_null()
            .map(|mut_ptr| unsafe { &mut *mut_ptr.as_ptr() })
    }
}
