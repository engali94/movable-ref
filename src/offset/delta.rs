use std::ptr::NonNull;

/// A nullable pointer, using `NonNull<T>`
pub type Ptr<T> = Option<NonNull<T>>;

/// Trait for types that can represent pointer differences.
///
/// Generalizes pointer arithmetic to integer types like `i8`, `i16`, `i32`.
/// Used internally by `SelfRef` for offset-based pointer storage.
///
/// # Safety
///
/// Implementations must maintain these invariants:
/// - `sub(a, a) == ZERO` for all pointers `a`
/// - `add(sub(a, b), b) == a` when `sub(a, b)` succeeds
/// - `add(ZERO, a) == a` for all pointers `a`
pub unsafe trait Offset: Copy + Eq {
    /// Error type returned when pointer difference cannot be represented.
    type Error;

    /// Computes the difference between two pointers.
    ///
    /// Returns `Err` if the difference cannot be represented in `Self`.
    fn sub(a: *mut u8, b: *mut u8) -> Result<Self, Self::Error>;

    /// Computes pointer difference without bounds checking.
    ///
    /// # Safety
    ///
    /// The difference between `a` and `b` must be representable in `Self`.
    unsafe fn sub_unchecked(a: *mut u8, b: *mut u8) -> Self;

    /// Adds the offset to a base pointer.
    ///
    /// # Safety
    ///
    /// The resulting pointer must be valid for the intended use.
    unsafe fn add(self, a: *const u8) -> *mut u8;
}

/// A `Delta` type that has a null/zero value.
///
/// # Safety
///
/// Must satisfy: `add(NULL, ptr) == ptr` for all pointers.
pub trait Nullable: Offset {
    /// The null/zero offset value.
    const NULL: Self;
}
