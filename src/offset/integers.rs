use super::delta::{Nullable, Offset};
use crate::error::{IntegerOffsetError, IntegerOffsetErrorImpl};
use crate::pointer::unreachable::{UncheckedOptionExt, OVERFLOW_SUB};

macro_rules! impl_delta_zeroable {
    ($($type:ty),* $(,)?) => {$(
        unsafe impl Offset for $type {
            type Error = IntegerOffsetError;

            fn sub(a: *mut u8, b: *mut u8) -> Result<Self, Self::Error> {
                let del = match isize::checked_sub(a as usize as _, b as usize as _) {
                    Some(del) => del,
                    None => return Err(IntegerOffsetError(IntegerOffsetErrorImpl::Sub(a as usize, b as usize)))
                };

                if std::mem::size_of::<Self>() < std::mem::size_of::<isize>() && (
                    (Self::MIN as isize) > del ||
                    (Self::MAX as isize) < del
                )
                {
                    Err(IntegerOffsetError(IntegerOffsetErrorImpl::Conversion(del)))
                } else {
                    Ok(del as _)
                }
            }

            unsafe fn sub_unchecked(a: *mut u8, b: *mut u8) -> Self {
                isize::checked_sub(a as usize as _, b as usize as _).unchecked_unwrap(OVERFLOW_SUB) as _
            }

            unsafe fn add(self, a: *const u8) -> *mut u8 {
                <*const u8>::offset(a, self as isize) as *mut u8
            }
        }

        impl Nullable for $type {
            const NULL: Self = 0;
        }
    )*};
}

impl_delta_zeroable! { i8, i16, i32, i64, i128, isize }
