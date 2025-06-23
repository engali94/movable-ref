# movable-ref

[![Crates.io](https://img.shields.io/crates/v/movable-ref.svg)](https://crates.io/crates/movable-ref)
[![Documentation](https://docs.rs/movable-ref/badge.svg)](https://docs.rs/movable-ref)
[![CI](https://github.com/engali94/movable-ref/workflows/CI/badge.svg)](https://github.com/engali94/movable-ref/actions)
[![Ubuntu](https://img.shields.io/github/actions/workflow/status/engali94/movable-ref/ci.yml?branch=main&label=Ubuntu&logo=ubuntu)](https://github.com/engali94/movable-ref/actions)
[![macOS](https://img.shields.io/github/actions/workflow/status/engali94/movable-ref/ci.yml?branch=main&label=macOS&logo=apple)](https://github.com/engali94/movable-ref/actions)
[![Windows](https://img.shields.io/github/actions/workflow/status/engali94/movable-ref/ci.yml?branch=main&label=Windows&logo=windows)](https://github.com/engali94/movable-ref/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.70+-blue.svg)](https://github.com/engali94/movable-ref/actions)

A Rust library for **offset based pointers** that enable movable self-referential data structures.


## The Problem

Standard Rust cannot create self-referential structures that can be moved in memory:

```rust
// This is impossible in safe Rust
struct SelfRef<'a> {
    data: String,
    ptr: &'a str,  // Cannot reference self.data
}
```

Existing solutions have many limitations:
- `Pin<Box<T>>`: Requires heap allocation, prevents movement.
- `Rc<RefCell<T>>`: Runtime overhead, not `Send`/`Sync`
- `ouroboros`: Complex macros, limited flexibility.

## The Solution

Offset pointers store **offsets** instead of absolute addresses, enabling self-referential structures that remain valid when moved:

```rust
use movable_ref::SelfRef;

struct Node {
    value: String,
    self_ref: SelfRef<String, i16>,  // 2-byte offset instead of 8-byte pointer
}

impl Node {
    fn new(value: String) -> Self {
        let mut node = Self {
            value,
            self_ref: SelfRef::null(),
        };
        node.self_ref.set(&mut node.value).unwrap();
        node
    }
    
    fn get_value(&self) -> &str {
        unsafe { self.self_ref.as_ref_unchecked() }
    }
}

// This works! The structure can be moved anywhere
let node = Node::new("Hello".to_string());
let boxed = Box::new(node);         // ✓ Moves to heap
let mut vec = Vec::new();
vec.push(*boxed);                   // ✓ Moves again
println!("{}", vec[0].get_value()); // ✓ Still works!
```

## Why tether?

Offset pointers solve a fundamental limitation in Rust: creating efficient, movable self-referential data structures. While other solutions exist, tether provides:

1. **Embedded Systems Friendly**: Can run in very memory constrained devices. 
2. **Movement freedom**: Structures work on stack, heap, or anywhere unlike `Pin`
3. **True zero-cost abstraction**: Zero to Minimal runtime overhead
4. **Memory efficiency**: 1-8 bytes vs 8+ bytes for alternatives  
5. **Simplicity**: Straightforward API without complex macros

Perfect for performance-critical applications, embedded systems, and anywhere you need self-referential structures that can move.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
movable-ref = "0.1.0"
```

## Basic Usage

```rust
use movable_ref::SelfRef;

// 1. Create structure with null pointer
let mut data = MyStruct {
    value: "Hello".to_string(),
    ptr: SelfRef::null(),
};

// 2. Set the relative pointer
data.ptr.set(&mut data.value).unwrap();

// 3. Dereference the pointer
let reference: &str = unsafe { data.ptr.as_ref_unchecked() };
```

## Features

- **`no_std`**: Works in embedded environments
- **`nightly`**: Trait object support with nightly Rust

```toml
[dependencies]
movable-ref = { version = "0.1.0", features = ["no_std"] }
```
## Performance Benchmarks

Run `cargo bench` to see these results on your machine:

### 🚀 **Access Speed** (lower = faster)
```
Direct Access:   329ps  (baseline)
SelfRef:         331ps  ⭐ FASTEST
Pin<Box<T>>:     365ps  (+10% slower)
Rc<RefCell<T>>:  429ps  (+30% slower)
```

### 💾 **Memory Efficiency**
```
SelfRef<T, i8>:   1 byte  (±127 byte range)
SelfRef<T, i16>:  2 bytes (±32KB range)  
SelfRef<T, i32>:  4 bytes (±2GB range)
*const T:         8 bytes (full address space)
Rc<RefCell<T>>:   8 bytes + heap allocation
```

### ⚡ **Creation Speed**
```
Direct:           19ns   (baseline)
SelfRef:          38ns   (+100% but still fastest)
Rc<RefCell<T>>:   40ns   
Pin<Box<T>>:      46ns   
```

### 🔄 **Move Semantics**
```
Direct move:      49ns   
Rc<RefCell<T>>:   50ns   (clone, not true move)
SelfRef move:     58ns   ⭐ __TRUE MOVE SEMANTICS__
Pin<Box<T>>:      N/A    (cannot move!)
```

**Key Takeaways:**
- ✅ **Zero-cost abstraction**: SelfRef access is as fast as direct access
- ✅ **Memory efficient**: 1-8 bytes vs 8+ bytes for alternatives
- ✅ **True movability**: Unlike Pin<Box<T>>, SelfRef can actually move
- ✅ **No runtime overhead**: No borrow checking like Rc<RefCell<T>>


## Comparison with Alternatives

| Solution | Move Cost | Memory | Runtime Cost | Complexity |
|----------|-----------|---------|--------------|------------|
| `SelfRef` | **Zero** | **1-8 bytes** | **Zero** | Low |
| `Pin<Box<T>>` | Impossible | 8+ bytes | Allocation | Medium |
| `Rc<RefCell<T>>` | Cheap | 16+ bytes | Reference counting | High |
| `ouroboros` | Zero | Varies | Zero | High |

## Safety

- ⚠️ Uses `unsafe` for pointer dereferencing
- ✅ Safe when structure layout doesn't change after pointer setup
- ✅ Safe for moving entire structures
- ✅ Extensively tested with Miri

## Examples

Run the examples to see tether in action:

```bash
# Basic usage
cargo run --example basic_usage

# Performance benchmarks
cargo run --example performance
```

## License

Licensed under [MIT license](LICENSE-MIT).
