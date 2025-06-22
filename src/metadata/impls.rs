use super::traits::PointerRecomposition;
use crate::offset::Ptr;
use std::ptr::NonNull;

unsafe impl<T: ?Sized> PointerRecomposition for &T {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl<T: ?Sized> PointerRecomposition for &mut T {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

unsafe impl PointerRecomposition for u8 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for u16 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for u32 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for u64 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for u128 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for usize {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

unsafe impl PointerRecomposition for i8 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for i16 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for i32 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for i64 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for i128 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for isize {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

unsafe impl PointerRecomposition for f32 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for f64 {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

unsafe impl PointerRecomposition for bool {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for char {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

// Common container types
unsafe impl<T> PointerRecomposition for Option<T> {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl<T, E> PointerRecomposition for Result<T, E> {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl<T> PointerRecomposition for Vec<T> {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl PointerRecomposition for String {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

unsafe impl PointerRecomposition for () {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl<A> PointerRecomposition for (A,) {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl<A, B> PointerRecomposition for (A, B) {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}
unsafe impl<A, B, C> PointerRecomposition for (A, B, C) {
    type Components = ();
    #[inline]
    fn decompose(_: &Self) -> Self::Components {}
    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, (): Self::Components) -> Ptr<Self> {
        ptr.map(NonNull::cast)
    }
}

unsafe impl<T> PointerRecomposition for [T] {
    type Components = usize;

    #[inline]
    fn decompose(this: &Self) -> Self::Components {
        this.len()
    }

    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, data: Self::Components) -> Ptr<Self> {
        Some(NonNull::from(std::slice::from_raw_parts_mut(
            ptr?.as_ptr() as *mut T,
            data,
        )))
    }
}

unsafe impl PointerRecomposition for str {
    type Components = usize;

    #[inline]
    fn decompose(this: &Self) -> Self::Components {
        this.len()
    }

    #[inline]
    unsafe fn recompose(ptr: Ptr<u8>, data: Self::Components) -> Ptr<Self> {
        Some(NonNull::from(std::str::from_utf8_unchecked_mut(
            std::slice::from_raw_parts_mut(ptr?.as_ptr(), data),
        )))
    }
}
