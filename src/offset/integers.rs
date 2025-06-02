use super::delta::{Offset, Nullable};
use crate::error::{IntegerOffsetError, IntegerOffsetErrorImpl};
use crate::pointer::unreachable::{UncheckedOptionExt, OVERFLOW_SUB};
use core::num::*;

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
                    (Self::min_value() as isize) > del ||
                    (Self::max_value() as isize) < del
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

macro_rules! impl_delta_nonzero {
    ($($type:ident $base:ident),* $(,)?) => {$(
        unsafe impl Offset for $type {
            type Error = IntegerOffsetError;

            fn sub(a: *mut u8, b: *mut u8) -> Result<Self, Self::Error> {
                let del = match isize::checked_sub(a as usize as _, b as usize as _) {
                    None => return Err(IntegerOffsetError(IntegerOffsetErrorImpl::Sub(a as usize, b as usize))),
                    Some(0) => return Err(IntegerOffsetError(IntegerOffsetErrorImpl::InvalidNonZero)),
                    Some(del) => del,
                };

                if std::mem::size_of::<Self>() < std::mem::size_of::<isize>() && (
                    ($base::min_value() as isize) > del ||
                    ($base::max_value() as isize) < del
                )
                {
                    Err(IntegerOffsetError(IntegerOffsetErrorImpl::Conversion(del)))
                } else {
                    // 0 case was checked in match before hand, so this is guarenteed ot be non zero
                    unsafe { Ok(Self::new_unchecked(del as _)) }
                }
            }

            unsafe fn sub_unchecked(a: *mut u8, b: *mut u8) -> Self {
                Self::new_unchecked(isize::checked_sub(a as usize as _, b as usize as _).unchecked_unwrap(OVERFLOW_SUB) as _)
            }

            unsafe fn add(self, a: *const u8) -> *mut u8 {
                <*mut u8>::offset(a as _, self.get() as isize) as *mut u8
            }
        }
    )*};
}

impl_delta_nonzero! { NonZeroI8 i8, NonZeroI16 i16, NonZeroI32 i32, NonZeroI64 i64, NonZeroI128 i128, NonZeroIsize isize } 
