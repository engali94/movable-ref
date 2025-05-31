
/**
 * If an integer's range is too small to store an offset, then
 * this error is generated
 */
#[derive(Debug)]
pub struct IntegerOffsetError(pub(crate) IntegerOffsetErrorImpl);

/// All types of errors, this is internal and so protected
/// behind a wrapper struct
#[derive(Debug)]
pub(crate) enum IntegerOffsetErrorImpl {
    /// Failed to convert isize to given integer type
    Conversion(isize),
    /// Failed to subtract the two usizes (overflowed isize)
    Sub(usize, usize),
    /// Got a zero when a non-zero value was expected (for `NonZero*`)
    InvalidNonZero
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for IntegerOffsetError {}

mod fmt {
    use super::*;
    use std::fmt;

    impl fmt::Display for IntegerOffsetError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                IntegerOffsetErrorImpl::Conversion(del) => write!(
                    f,
                    "Offset could not be stored (offset of {} is too large)",
                    del
                ),
                IntegerOffsetErrorImpl::Sub(a, b) => {
                    write!(f, "Difference is beween {} and {} overflows `isize`", a, b)
                },
                
                IntegerOffsetErrorImpl::InvalidNonZero => {
                    write!(f, "Difference was zero when a `NonZero*` type was specified")
                }
            }
        }
    }
}
