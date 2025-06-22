use crate::offset::Ptr;

/// The bridge that makes `SelfRef` work with any type, sized or unsized.
///
/// Rust has two kinds of types: sized ones like `i32` and `String` that have a known
/// size at compile time, and unsized ones like `[T]`, `str`, and trait objects that
/// need extra metadata to work with. This trait abstracts away that complexity,
/// letting `SelfRef` handle both seamlessly.
///
/// Most users never need to implement this trait directly - it's already implemented
/// for all the types you'd want to use. The magic happens behind the scenes when you
/// create a `SelfRef<[u8]>` or `SelfRef<TraitObject<dyn Debug>>`.
///
/// ```rust
/// use tether::SelfRef;
///
/// // Works with sized types (metadata = ())
/// let mut value = 42i32;
/// let mut ptr: SelfRef<i32, i16> = SelfRef::null();
/// ptr.set(&mut value).unwrap();
///
/// // Also works with slices (metadata = length)
/// let mut data = vec![1, 2, 3, 4, 5];
/// let mut slice_ptr: SelfRef<[i32], i16> = SelfRef::null();
/// slice_ptr.set(&mut data[1..4]).unwrap();  // Points to middle section
/// ```
///
/// The trait handles the complex pointer arithmetic needed to reconstruct fat pointers
/// from offset-based storage, making `SelfRef` truly universal across Rust's type system.
///
/// # Safety
///
/// Implementations must correctly reconstruct valid pointers. The `compose` method
/// is the critical piece - it takes a raw data pointer and metadata, then builds
/// a proper pointer to `Self`. Get this wrong and you'll have undefined behavior.
pub unsafe trait PointerRecomposition {
    /// The metadata type - `()` for sized types, `usize` for slices, vtables for trait objects.
    ///
    /// This is what gets stored alongside the offset to fully reconstruct the pointer later.
    /// For a `String`, this is just `()` since the size is in the struct itself.
    /// For a `[u8]`, this is the length. For trait objects, it's the vtable pointer.
    type Components: Copy + Eq;

    /// Extracts metadata from a live reference.
    ///
    /// When `SelfRef` stores a pointer, it calls this to capture whatever extra
    /// information is needed to reconstruct the reference later. Think of it as
    /// "what do I need to remember about this pointer besides its location?"
    fn decompose(this: &Self) -> Self::Components;

    /// Reconstructs a proper pointer from raw components.
    ///
    /// This is where the magic happens - takes a basic data pointer and the stored
    /// metadata, then builds back the original fat pointer. For slices, this means
    /// combining the data pointer with the length. For trait objects, it's data + vtable.
    unsafe fn recompose(ptr: Ptr<u8>, data: Self::Components) -> Ptr<Self>;
}
