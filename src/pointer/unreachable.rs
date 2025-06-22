pub(crate) const OVERFLOW_SUB: &str =
    "Attempted to subtract with overflow, this is UB in release mode!";

/// Adds an unchecked unwrap, this unwrap is UB if self is None
///
/// # Safety
///
/// This is UB if self is None and debug assertions are enabled
pub(crate) trait UncheckedOptionExt {
    type T;

    unsafe fn unchecked_unwrap(self, err: &str) -> Self::T;
}

impl<T> UncheckedOptionExt for Option<T> {
    type T = T;

    #[inline]
    #[allow(clippy::assertions_on_constants)]
    unsafe fn unchecked_unwrap(self, err: &str) -> T {
        match self {
            Some(value) => value,
            None if cfg!(debug_assertions) => panic!("{}", err),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
