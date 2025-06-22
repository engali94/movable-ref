use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use tether::SelfRef;

// ============================================================================
// Test Data Structures (Small, Fast)
// ============================================================================

struct SelfRefStruct {
    data: [u64; 100], 
    ptr: SelfRef<u64, i16>,
}

impl SelfRefStruct {
    fn new() -> Self {
        let mut this = Self {
            data: [42; 100],
            ptr: SelfRef::null(),
        };
        this.ptr.set(&mut this.data[50]).unwrap();
        this
    }

    fn get_value(&self) -> u64 {
        unsafe { *self.ptr.as_ref_unchecked() }
    }
}

struct PinnedStruct {
    data: [u64; 100],
    ptr: *const u64,
}

impl PinnedStruct {
    fn new() -> Pin<Box<Self>> {
        let data = [42; 100];
        let ptr = &data[50] as *const u64;
        Box::pin(Self { data, ptr })
    }

    fn get_value(&self) -> u64 {
        unsafe { *self.ptr }
    }
}

#[derive(Clone)]
struct RcRefCellStruct {
    _data: [u64; 100],
    ptr: Rc<RefCell<u64>>,
}

impl RcRefCellStruct {
    fn new() -> Self {
        Self {
            _data: [42; 100],
            ptr: Rc::new(RefCell::new(42)),
        }
    }

    fn get_value(&self) -> u64 {
        *self.ptr.borrow()
    }
}

struct DirectStruct {
    data: [u64; 100],
    index: usize,
}

impl DirectStruct {
    fn new() -> Self {
        Self {
            data: [42; 100],
            index: 50,
        }
    }

    fn get_value(&self) -> u64 {
        self.data[self.index]
    }
}

// ============================================================================

fn bench_memory_footprint(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_footprint");
    group.sample_size(10);
    
    group.bench_function("SelfRef<u64, i8>", |b| {
        b.iter(|| black_box(std::mem::size_of::<SelfRef<u64, i8>>()))
    });
    
    group.bench_function("SelfRef<u64, i16>", |b| {
        b.iter(|| black_box(std::mem::size_of::<SelfRef<u64, i16>>()))
    });
    
    group.bench_function("*const u64", |b| {
        b.iter(|| black_box(std::mem::size_of::<*const u64>()))
    });
    
    group.bench_function("Rc<RefCell<u64>>", |b| {
        b.iter(|| black_box(std::mem::size_of::<Rc<RefCell<u64>>>()))
    });
    
    group.finish();
}

fn bench_access_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("access_speed");
    group.sample_size(50);
    
    let self_ref = SelfRefStruct::new();
    let pinned = PinnedStruct::new();
    let rc_refcell = RcRefCellStruct::new();
    let direct = DirectStruct::new();
    
    group.bench_function("SelfRef", |b| {
        b.iter(|| black_box(self_ref.get_value()))
    });
    
    group.bench_function("Pin<Box<T>>", |b| {
        b.iter(|| black_box(pinned.get_value()))
    });
    
    group.bench_function("Rc<RefCell<T>>", |b| {
        b.iter(|| black_box(rc_refcell.get_value()))
    });
    
    group.bench_function("Direct Access", |b| {
        b.iter(|| black_box(direct.get_value()))
    });
    
    group.finish();
}

fn bench_creation_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("creation_speed");
    group.sample_size(30);
    
    group.bench_function("SelfRef", |b| {
        b.iter(|| black_box(SelfRefStruct::new()))
    });
    
    group.bench_function("Pin<Box<T>>", |b| {
        b.iter(|| black_box(PinnedStruct::new()))
    });
    
    group.bench_function("Rc<RefCell<T>>", |b| {
        b.iter(|| black_box(RcRefCellStruct::new()))
    });
    
    group.bench_function("Direct", |b| {
        b.iter(|| black_box(DirectStruct::new()))
    });
    
    group.finish();
}

fn bench_move_semantics(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_semantics");
        group.sample_size(30);
    
    group.bench_function("SelfRef move", |b| {
        b.iter(|| {
            let s = SelfRefStruct::new();
            let moved = Box::new(s);
            black_box(moved.get_value())
        })
    });
    
    group.bench_function("Rc<RefCell<T>> clone", |b| {
        b.iter(|| {
            let s = RcRefCellStruct::new();
            let cloned = s.clone();
            black_box(cloned.get_value())
        })
    });
    
    group.bench_function("Direct move", |b| {
        b.iter(|| {
            let s = DirectStruct::new();
            let moved = Box::new(s);
            black_box(moved.get_value())
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_memory_footprint,
    bench_access_speed,
    bench_creation_speed,
    bench_move_semantics
);
criterion_main!(benches); 
