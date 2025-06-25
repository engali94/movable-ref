use super::*;

struct SelfRefTest<T, U: ?Sized + PointerRecomposition> {
    t_ref: SelfRef<U, i8>,
    t: T,
}

fn id<T>(t: &mut T) -> &mut T {
    t
}

impl<T, U: ?Sized + PointerRecomposition> SelfRefTest<T, U> {
    pub fn new(t: T, f: fn(&mut T) -> &mut U) -> Self {
        let mut this = Self {
            t,
            t_ref: SelfRef::null(),
        };

        this.t_ref.set(f(&mut this.t)).unwrap();

        this
    }

    pub fn t(&self) -> &T {
        &self.t
    }

    pub fn t_mut(&mut self) -> &mut T {
        &mut self.t
    }

    pub fn t_ref(&self) -> &U {
        unsafe { self.t_ref.as_ref_unchecked() }
    }

    #[allow(unused)]
    pub fn t_ref_mut(&mut self) -> &mut U {
        unsafe { self.t_ref.as_mut_unchecked() }
    }
}

#[inline(never)]
fn block_opt<T>(x: T) -> T {
    x
}

#[test]
fn simple_test() {
    let mut s = SelfRefTest {
        t: "Hello World",
        t_ref: SelfRef::null(),
    };

    s.t_ref.set(&mut s.t).unwrap();

    assert_eq!(s.t(), s.t_ref());
    assert_eq!(*s.t(), "Hello World");
    assert_eq!(*s.t_ref(), "Hello World");
}

#[test]
fn simple_move() {
    let mut s = SelfRefTest {
        t: "Hello World",
        t_ref: SelfRef::null(),
    };

    s.t_ref.set(&mut s.t).unwrap();

    assert_eq!(s.t(), s.t_ref());
    assert_eq!(*s.t(), "Hello World");
    assert_eq!(*s.t_ref(), "Hello World");

    let s = block_opt(s);

    assert_eq!(s.t(), s.t_ref());
    assert_eq!(*s.t(), "Hello World");
    assert_eq!(*s.t_ref(), "Hello World");
}

#[test]
fn simple_move_after_init() {
    let s = SelfRefTest::new("Hello World", id);

    assert_eq!(s.t(), s.t_ref());
    assert_eq!(*s.t(), "Hello World");
    assert_eq!(*s.t_ref(), "Hello World");

    let s = block_opt(s);

    assert_eq!(s.t(), s.t_ref());
    assert_eq!(*s.t(), "Hello World");
    assert_eq!(*s.t_ref(), "Hello World");
}

#[test]
fn swap() {
    let mut s = SelfRefTest::new("Hello World", id);
    let mut x = SelfRefTest::new("Killer Move", id);

    assert_eq!(*s.t(), "Hello World");
    assert_eq!(*x.t(), "Killer Move");

    assert_eq!(*s.t_ref(), "Hello World");
    assert_eq!(*x.t_ref(), "Killer Move");

    std::mem::swap(&mut s, &mut x);

    assert_eq!(*s.t(), "Killer Move");
    assert_eq!(*x.t(), "Hello World");

    assert_eq!(*s.t_ref(), "Killer Move");
    assert_eq!(*x.t_ref(), "Hello World");
}

#[test]
fn aliasing() {
    let mut s = SelfRefTest::new("Hello World", id);

    assert_eq!(s.t(), s.t_ref());

    *s.t_mut() = "Killer Move";

    assert_eq!(*s.t(), "Killer Move");
    assert_eq!(*s.t_ref(), "Killer Move");
}

#[test]
fn sub_str() {
    #[inline(never)]
    fn get_move(s: SelfRefTest<[u8; 5], [u8]>) {
        assert_eq!(*s.t(), [0, 1, 2, 3, 4]);
        assert_eq!(*s.t_ref(), [2, 3, 4]);
    }

    let s = SelfRefTest::new([0, 1, 2, 3, 4], |x| &mut x[2..]);

    assert_eq!(*s.t(), [0, 1, 2, 3, 4]);
    assert_eq!(*s.t_ref(), [2, 3, 4]);

    get_move(s);
}

#[test]
fn check_copy() {
    fn is_copy<T: Copy>() {}

    #[allow(unused, path_statements)]
    fn check<T: ?Sized + PointerRecomposition, I: Offset>() {
        is_copy::<SelfRef<T, I>>;
    }
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::*;

    #[derive(Debug)]
    struct TestStruct {
        value: u32,
    }

    #[test]
    fn check_trait_object_simple() {
        let s = SelfRefTest::new(TestStruct { value: 42 }, |x| unsafe {
            TraitObject::from_mut(x as &mut dyn std::fmt::Debug)
        });

        assert_eq!(s.t().value, 42);

        #[cfg(feature = "std")]
        {
            let debug_str = format!("{:?}", s.t_ref().as_ref());
            assert!(debug_str.contains("42"));
        }
    }

    #[test]
    fn check_trait_object_after_move() {
        let s = SelfRefTest::new(TestStruct { value: 42 }, |x| unsafe {
            TraitObject::from_mut(x as &mut dyn std::fmt::Debug)
        });

        assert_eq!(s.t().value, 42);

        #[cfg(feature = "std")]
        {
            let debug_str = format!("{:?}", s.t_ref().as_ref());
            assert!(debug_str.contains("42"));
        }

        #[inline(never)]
        fn force_move<T>(t: T) -> T {
            t
        }

        let s = force_move(s);

        assert_eq!(s.t().value, 42);

        #[cfg(feature = "std")]
        {
            let debug_str = format!("{:?}", s.t_ref().as_ref());
            assert!(debug_str.contains("42"));
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn check_trait_object_after_move_heap() {
        let s = SelfRefTest::new(TestStruct { value: 42 }, |x| unsafe {
            TraitObject::from_mut(x as &mut dyn std::fmt::Debug)
        });

        assert_eq!(s.t().value, 42);

        #[cfg(feature = "std")]
        {
            let debug_str = format!("{:?}", s.t_ref().as_ref());
            assert!(debug_str.contains("42"));
        }

        let s = Box::new(s);

        assert_eq!(s.t().value, 42);

        #[cfg(feature = "std")]
        {
            let debug_str = format!("{:?}", s.t_ref().as_ref());
            assert!(debug_str.contains("42"));
        }
    }
}
