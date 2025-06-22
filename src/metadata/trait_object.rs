use super::traits::PointerRecomposition;
use crate::offset::Ptr;
use std::mem;
use std::ptr::{self, NonNull, Pointee};

/// A wrapper that enables trait objects to work seamlessly with `SelfRef`.
///
/// Rust's trait objects have complex internal structure (fat pointers with data + vtable),
/// making them incompatible with offset-based pointers. `TraitObject<T>` bridges this gap
/// by providing the metadata handling needed for `SelfRef` to work with trait objects.
///
/// Standard `SelfRef` works great with concrete types, but trait objects like `dyn Any`
/// or `dyn Debug` need special handling because they're "fat pointers" containing both
/// a data pointer and metadata (vtable). This wrapper makes that "just work".
///
/// # Example: Self-Referential Any Storage
///
/// ```rust
/// # #![feature(ptr_metadata)]
/// # fn main() {
/// use tether::{SelfRef, TraitObject};
/// use std::any::Any;
///
/// struct Container {
///     data: Vec<u8>,
///     any_ref: SelfRef<TraitObject<dyn Any>, i16>,
/// }
///
/// impl Container {
///     fn new(data: Vec<u8>) -> Self {
///         let mut container = Self {
///             data,
///             any_ref: SelfRef::null(),
///         };
///         
///         // Convert our data to a trait object and store it
///         let trait_obj = unsafe {
///             TraitObject::from_mut(&mut container.data as &mut dyn Any)
///         };
///         container.any_ref.set(trait_obj).unwrap();
///         
///         container
///     }
///     
///     fn get_any(&self) -> &dyn Any {
///         unsafe { self.any_ref.as_ref_unchecked().as_ref() }
///     }
/// }
///
/// // Works even after moving!
/// let container = Container::new(vec![1, 2, 3]);
/// let boxed = Box::new(container);
/// println!("Type: {:?}", boxed.get_any().type_id());
/// # }
/// ```
///
/// # Safety
///
/// This type is `#[repr(transparent)]` and should only be used with actual trait objects.
/// Using it with concrete types will lead to undefined behavior.
#[repr(transparent)]
pub struct TraitObject<T: ?Sized + Pointee<Metadata = ptr::DynMetadata<T>>>(T);

impl<T: ?Sized + Pointee<Metadata = ptr::DynMetadata<T>>> TraitObject<T> {
    /// Wraps an immutable trait object reference for use with `SelfRef`.
    ///
    /// This creates a `TraitObject` wrapper around your trait object, enabling
    /// it to be stored in a `SelfRef`. The wrapper is zero-cost and transparent.
    ///
    /// # Safety
    ///
    /// `T` must be an actual trait object (like `dyn Debug`, `dyn Any`, etc.).
    /// Using this with concrete types will cause undefined behavior.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![feature(ptr_metadata)]
    /// use tether::TraitObject;
    /// use std::fmt::Debug;
    ///
    /// let value = 42i32;
    /// let debug_obj: &dyn Debug = &value;
    /// let wrapped = unsafe { TraitObject::from_ref(debug_obj) };
    /// ```
    pub unsafe fn from_ref(t: &T) -> &Self {
        mem::transmute(t)
    }

    /// Wraps a mutable trait object reference for use with `SelfRef`.
    ///
    /// Like `from_ref`, but for mutable references. This is what you'll typically
    /// use when setting up self-referential structures.
    ///
    /// # Safety
    ///
    /// `T` must be an actual trait object (like `dyn Debug`, `dyn Any`, etc.).
    /// Using this with concrete types will cause undefined behavior.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![feature(ptr_metadata)]
    /// use tether::TraitObject;
    /// use std::fmt::Debug;
    ///
    /// let mut value = vec![1, 2, 3];
    /// let debug_obj: &mut dyn Debug = &mut value;
    /// let wrapped = unsafe { TraitObject::from_mut(debug_obj) };
    /// ```
    pub unsafe fn from_mut(t: &mut T) -> &mut Self {
        mem::transmute(t)
    }

    /// Unwraps back to the original trait object reference.
    ///
    /// Once you've retrieved your `TraitObject` from a `SelfRef`, use this
    /// to get back the original trait object for normal use.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![feature(ptr_metadata)]
    /// # use tether::{SelfRef, TraitObject};
    /// # use std::any::Any;
    /// # let mut data = vec![1u8, 2, 3];
    /// # let mut self_ref: SelfRef<TraitObject<dyn Any>, i16> = SelfRef::null();
    /// # let trait_obj = unsafe { TraitObject::from_mut(&mut data as &mut dyn Any) };
    /// # self_ref.set(trait_obj).unwrap();
    ///
    /// let retrieved = unsafe { self_ref.as_ref_unchecked() };
    /// let original: &dyn Any = retrieved.as_ref();
    /// ```
    pub fn as_ref(&self) -> &T {
        &self.0
    }

    /// Unwraps back to the original mutable trait object reference.
    ///
    /// Like `as_ref`, but returns a mutable reference to the trait object.
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

unsafe impl<T: ?Sized + Pointee<Metadata = ptr::DynMetadata<T>>> PointerRecomposition for TraitObject<T> {
    type Components = ptr::DynMetadata<T>;

    #[inline]
    fn decompose(this: &Self) -> Self::Components {
        ptr::metadata(this.as_ref() as *const T)
    }

    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, metadata: Self::Components) -> Ptr<Self> {
        let data_ptr = ptr?.as_ptr();
        let trait_obj_ptr = ptr::from_raw_parts(data_ptr as *const (), metadata) as *const T;
        let self_ptr = mem::transmute::<*const T, *const Self>(trait_obj_ptr);
        NonNull::new(self_ptr as *mut Self)
    }
}
