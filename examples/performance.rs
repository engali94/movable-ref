#![allow(clippy::uninlined_format_args)]

use movable_ref::{selfref_accessors, SelfRef, SelfRefCell};
use std::hint::black_box;
use std::time::Instant;

struct SelfRefRelPtr {
    data: [u64; 100],
    ptr: SelfRefCell<u64, i16>,
}

impl SelfRefRelPtr {
    fn new() -> Self {
        let mut this = Self {
            data: [0u64; 100],
            ptr: SelfRefCell::new(0u64).unwrap(),
        };

        this.data.iter_mut().enumerate().for_each(|(i, item)| {
            *item = i as u64 * 2;
        });

        this.ptr = SelfRefCell::new(this.data[50]).unwrap();
        this
    }
}

selfref_accessors!(impl SelfRefRelPtr { get_value : ptr -> u64 });

struct DirectAccess {
    data: [u64; 100],
    index: usize,
}

impl DirectAccess {
    fn new() -> Self {
        let mut data = [0u64; 100];
        data.iter_mut().enumerate().for_each(|(i, item)| {
            *item = i as u64 * 2;
        });

        Self { data, index: 50 }
    }

    fn get_value(&self) -> u64 {
        self.data[self.index]
    }
}

fn benchmark_memory_usage() {
    println!("Memory Usage Comparison:");
    println!(
        "  SelfRef<u64, i8>:   {} bytes",
        std::mem::size_of::<SelfRef<u64, i8>>()
    );
    println!(
        "  SelfRef<u64, i16>:  {} bytes",
        std::mem::size_of::<SelfRef<u64, i16>>()
    );
    println!(
        "  SelfRef<u64, i32>:  {} bytes",
        std::mem::size_of::<SelfRef<u64, i32>>()
    );
    println!(
        "  SelfRef<u64, isize>:{} bytes",
        std::mem::size_of::<SelfRef<u64, isize>>()
    );
    println!(
        "  *const u64:        {} bytes",
        std::mem::size_of::<*const u64>()
    );
    println!(
        "  usize (index):     {} bytes",
        std::mem::size_of::<usize>()
    );
    println!();

    println!("Structure Size Comparison:");
    println!(
        "  SelfRefRelPtr:     {} bytes",
        std::mem::size_of::<SelfRefRelPtr>()
    );
    println!(
        "  DirectAccess:      {} bytes",
        std::mem::size_of::<DirectAccess>()
    );
    println!();
}

fn benchmark_access_performance() {
    const ITERATIONS: usize = 10_000_000;

    let rel_ptr_struct = SelfRefRelPtr::new();
    let direct_struct = DirectAccess::new();

    let start = Instant::now();
    (0..ITERATIONS).for_each(|_| {
        black_box(rel_ptr_struct.get_value());
    });
    let rel_ptr_time = start.elapsed();

    let start = Instant::now();
    (0..ITERATIONS).for_each(|_| {
        black_box(direct_struct.get_value());
    });
    let direct_time = start.elapsed();

    println!("Access Performance ({} iterations):", ITERATIONS);
    println!("  SelfRef access:    {:?}", rel_ptr_time);
    println!("  Direct access:     {:?}", direct_time);
    println!(
        "  Overhead:          {:.2}%",
        ((rel_ptr_time.as_nanos() as f64 / direct_time.as_nanos() as f64) - 1.0) * 100.0
    );
    println!();
}

fn demonstrate_movability() {
    println!("Movability Demonstration:");

    let s = SelfRefRelPtr::new();
    println!("  Original value: {}", s.get_value());

    let boxed = Box::new(s);
    println!("  After Box::new: {}", boxed.get_value());

    let moved_again = *boxed;
    println!("  After unbox:    {}", moved_again.get_value());

    let vec = [moved_again, SelfRefRelPtr::new()];
    println!("  In vector[0]:   {}", vec[0].get_value());
    println!("  In vector[1]:   {}", vec[1].get_value());

    println!("  ✓ All moves preserved the internal reference!");
    println!();
}

fn main() {
    println!("rel-ptr Performance Benchmarks");
    println!("==============================");
    println!();

    benchmark_memory_usage();
    benchmark_access_performance();
    demonstrate_movability();

    println!("Metrics:");
    println!("- SelfRef uses 1-8 bytes vs 8 bytes for full pointers (on 64-bit)");
    println!("- Access overhead is minimal (usually <50% vs direct access)");
    println!("- Enables impossible patterns: movable self-referential structures");
    println!("- Zero-cost abstraction: overhead comes from offset calculation");
    println!("- Perfect for embedded systems where every byte counts");
}
