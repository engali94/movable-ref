#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "nightly", feature(ptr_metadata))]
#![allow(clippy::needless_doctest_main)]
#![forbid(missing_docs)]
#![deny(unused_must_use)]

/*!
# tether

`tether` is a library for offset-based pointers, which can be used to create
movable self-referential types. Ituses an offset and its current location to
calculate where it points to.

## Safety

See the `SelfRef` type documentation for safety information.

## Features

### `no_std`

This crate is `no_std` compatible. Enable the `no_std` feature to use without the standard library.

## Example

Consider the memory segment below:

`[.., 0x3a, 0x10, 0x02, 0xe4, 0x2b ..]`

Where `0x3a` has address `0xff304050` (32-bit system)
and `0x2b` has address `0xff304054`.

If we have a 1-byte relative pointer (`SelfRef<_, i8>`)
at address `0xff304052`, then that relative pointer points to
`0x2b` because its address `0xff304052` plus its
offset `0x02` equals `0xff304054`.

Three key properties emerge:
1) It only took 1 byte to point to another value
2) A relative pointer can only access nearby memory
3) If both the relative pointer and pointee move together,
   the relative pointer remains valid

The third property enables movable self-referential structures.

The type `SelfRef<T, I>` is a relative pointer where `T` is the target type
and `I` is the offset storage type. In practice, you can ignore `I`
(defaulted to `isize`) as it covers most use cases. For size optimization,
use any type implementing `Delta`: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.

The tradeoff: smaller offset types reduce addressable range.
`isize` covers at least half of addressable memory. For self-referential
structures, choose an offset type whose range exceeds your structure size:
`std::mem::size_of::<YourStruct>() <= I::MAX`.

Note: Unsized types require additional considerations.

## Self-Referential Type Example

```rust
# fn main() {
# use movable_ref::SelfRef;
struct SelfRefStruct {
    value: (String, u32),
    ptr: SelfRef<String, i8>
}

impl SelfRefStruct {
    pub fn new(s: String, i: u32) -> Self {
        let mut this = Self {
            value: (s, i),
            ptr: SelfRef::null()
        };

        this.ptr.set(&mut this.value.0).unwrap();
        this
    }

    pub fn fst(&self) -> &str {
        unsafe { self.ptr.as_ref_unchecked() }
    }

    pub fn snd(&self) -> u32 {
        self.value.1
    }
}

let s = SelfRefStruct::new("Hello World".into(), 10);

assert_eq!(s.fst(), "Hello World");
assert_eq!(s.snd(), 10);

let s = Box::new(s); // Force a move - relative pointers work on the heap

assert_eq!(s.fst(), "Hello World");
assert_eq!(s.snd(), 10);
# }
```

## Pattern Analysis

The example demonstrates the standard pattern for safe movable self-referential types:

**Structure Definition**: Contains data and a relative pointer. No lifetimes are used
as they would either prevent movement or create unresolvable constraints.

**Initialization Pattern**: Create the object with `SelfRef::null()`, then immediately
set the pointer using `SelfRef::set()`. Unwrapping provides immediate feedback
if the offset range is insufficient.

**Movement Safety**: Once set, the structure can be moved safely because relative
pointers maintain their offset relationship regardless of absolute position.

**Access Safety**: `SelfRef::as_ref_unchecked()` is safe when the pointer cannot
be invalidated - which occurs when direct pointer modification is impossible
and field offsets remain constant after initialization.
*/

#[cfg(feature = "no_std")]
extern crate core as std;

#[cfg(test)]
mod tests;

mod error;
mod metadata;
mod offset;
mod pointer;

pub use self::error::*;
pub use self::metadata::*;
pub use self::offset::*;
pub use self::pointer::*;
