# movable-ref

[![Crates.io](https://img.shields.io/crates/v/movable-ref.svg)](https://crates.io/crates/movable-ref)
[![Documentation](https://docs.rs/movable-ref/badge.svg)](https://docs.rs/movable-ref)
[![CI](https://github.com/engali94/movable-ref/workflows/CI/badge.svg)](https://github.com/engali94/movable-ref/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.70+-blue.svg)](https://github.com/engali94/movable-ref/actions)

Movable self-referential data for Rust without pinning or runtime bookkeeping.

## At a glance

- Store offsets instead of absolute pointers so your data can move freely across stack, heap, arenas, or embedded buffers.
- Works in `no_std` projects and can be tuned to an 8-bit offset for tightly packed layouts.
- Core API is explicit—helper macros are available but completely optional.
- Optional `debug-guards` feature adds runtime assertions while you are iterating; release builds stay lean.

## When to reach for `SelfRef`

- You need a self-referential struct that must move (e.g., push onto a `Vec`, relocate across buffers, or compact in place).
- You are targeting embedded or real-time systems where every byte matters and heap allocation is either expensive or unavailable.
- You want predictable behaviour and explicit control instead of macro-generated code or hidden reference counting.

## Quick start

Install the crate:

```toml
[dependencies]
movable-ref = "0.1.0"
```

Wrap a field in `SelfRefCell` to keep the unsafe details contained:

```rust
use movable_ref::SelfRefCell;

struct Message {
    body: SelfRefCell<String, i16>,
}

impl Message {
    fn new(body: String) -> Self {
        Self { body: SelfRefCell::new(body).expect("offset fits in i16") }
    }

    fn body(&self) -> &str {
        self.body.get()
    }

    fn body_mut(&mut self) -> &mut String {
        self.body.get_mut()
    }
}

let mut msg = Message::new("move me".into());
assert_eq!(msg.body(), "move me");

let mut together = Vec::new();
together.push(msg);          // moved to heap inside Vec
assert_eq!(together[0].body(), "move me");
```

Need more control? Use `SelfRef` directly:

```rust
use movable_ref::SelfRef;

struct Node {
    value: String,
    ptr: SelfRef<String, i16>,
}

impl Node {
    fn new(value: String) -> Self {
        let mut node = Self { value, ptr: SelfRef::null() };
        node.ptr.set(&mut node.value).expect("offset fits in i16");
        node
    }

    fn value(&self) -> &str {
        self.ptr.try_as_ref().expect("initialised" )
    }
}

let mut node = Node::new("hello".into());
let boxed = Box::new(node);
let mut list = Vec::new();
list.push(*boxed);            // node moves again
assert_eq!(list[0].value(), "hello");
```

## How it works

Rust normally stores raw pointers. Absolute addresses break the moment a struct moves. `SelfRef<T, I>` stores only the signed offset (`I`) between the pointer and the value it targets plus the metadata needed to rebuild fat pointers (`[T]`, `str`, trait objects).

When the owner moves, the relative distance stays the same, so recomputing the pointer after the move just works. Choose `I` to match the size of your container: `i8` covers ±127 bytes, `i16` covers ±32 KiB, `isize` covers most use cases.

## Safety model

`SelfRef` uses `unsafe` internally, so it is important to follow the invariants:

1. Initialise immediately: call `SelfRef::set` right after constructing the struct. The pointer stays unset otherwise.
2. Keep layout stable: do not reorder or remove the referenced field after initialisation.
3. Move the whole struct together: individual fields must not be detached from the container.

The crate provides layers to help you respect those rules:

- `SelfRefCell` hides the unsafe parts and gives you safe `try_get`/`try_get_mut` accessors.
- `SelfRef::try_as_ref` and `SelfRef::try_as_mut` return `Option` so you can handle uninitialised cases gracefully.
- `SelfRef::guard` returns an RAII guard that re-seals the pointer when you finish mutating the target.
- Enable the `debug-guards` feature during development to assert that recorded absolute pointers still match after moves.

Failure modes are documented in the crate root (`src/lib.rs`). Use the safe helpers whenever possible; unchecked calls are intended for tightly controlled internals.

## Benchmarks

The Criterion benchmarks live in `benches/performance.rs`.

| Operation | Direct | SelfRef | Pin<Box<T>> | Rc<RefCell<T>> |
|-----------|--------|---------|-------------|----------------|
| Access (ps) | 329 | **331** | 365 | 429 |
| Create (ns) | 19 | 38 | 46 | 40 |
| Move (ns) | 49 | **58** | N/A | 50 (clone) |

Memory usage per pointer:

```
SelfRef<T, i8>   : 1 byte (±127 bytes)
SelfRef<T, i16>  : 2 bytes (±32 KiB)
SelfRef<T, i32>  : 4 bytes (±2 GiB)
*const T         : 8 bytes
Rc<RefCell<T>>   : 8 bytes + heap allocation
```

`cargo bench` will rebuild these tables for your target.

## Tooling

| Task | Command |
|------|---------|
| Lint | `cargo clippy --all-targets -- -D warnings` |
| Format | `cargo fmt` |
| Tests | `cargo test` |
| Miri | `cargo +nightly miri test` (see full matrix below) |
| AddressSanitizer | `RUSTFLAGS="-Zsanitizer=address" ASAN_OPTIONS=detect_leaks=0 cargo +nightly test` |

Miri matrix:

```bash
cargo +nightly miri setup
cargo +nightly miri test
cargo +nightly miri test --no-default-features
cargo +nightly miri test --features nightly
cargo +nightly miri test --features debug-guards
```

## Comparison

| Approach | Moves? | Memory | Runtime cost | Notes |
|----------|--------|--------|--------------|-------|
| `SelfRef` | ✅ | 1–8 bytes | None | Works in `no_std`, flexible integer offsets |
| `Pin<Box<T>>` | ❌ | 8+ bytes | Allocation | Stable but data cannot move |
| `Rc<RefCell<T>>` | ➖ (clone) | 16+ bytes | Borrow checking + refcount | Allows interior mutability |
| `ouroboros` | ✅ | varies | None | Macro DSL, less manual control |

## License

MIT licensed. See `LICENSE-MIT` for details.
