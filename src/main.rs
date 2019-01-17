use std::collections::HashMap;

use lazy_static::lazy_static;

trait TearDown {
    fn tear_down(&self);
}

#[derive(Default)]
struct EmptyGuard {}
impl TearDown for EmptyGuard {
    fn tear_down(&self) {
        println!("Empty Tear Down")
    }
}

impl<A: TearDown, B: TearDown> TearDown for (A, B) {
    fn tear_down(&self) {
        self.0.tear_down();
        self.1.tear_down();
    }
}

struct Fixture<T, G: TearDown> {
    inner: Option<T>,
    guard: Option<G>,
}

impl<T, G: TearDown> Fixture<T, G> {
    fn new(inner: T, guard: G) -> Self {
        Fixture { inner: Some(inner), guard: Some(guard) }
    }
    pub fn take(&mut self) -> T {
        self.inner.take().unwrap()
    }
    pub fn guard(&mut self) -> G {
        self.guard.take().unwrap()
    }
    pub fn compose<OTHER: TearDown>(mut self, guard: OTHER) -> Fixture<T, (G, OTHER)> {
        Fixture::new(self.take(), (self.guard(), guard))
    }
}

impl<T, G: TearDown> Drop for Fixture<T, G> {
    fn drop(&mut self) {
        self.guard.take().map(|g| g.tear_down());
    }
}

impl<T> From<T> for Fixture<T, EmptyGuard> {
    fn from(inner: T) -> Self {
        Fixture::new(inner, Default::default())
    }
}

fn copy() -> usize {
    4
}

fn copy_val() -> Fixture<usize, impl TearDown> {
    println!("setup copy");
    struct G {}
    impl TearDown for G {
        fn tear_down(&self) {
            println!("teardown copy");
        }
    }

    Fixture::new(copy(), G {})
}

lazy_static! {
    static ref HASH: HashMap<i32, &'static str> = {
        let mut h = HashMap::new();
        h.insert(3, "ciao");
        h
    };
}

fn key() -> i32 {
    3
}

fn key_val() -> Fixture<i32, impl TearDown> {
    println!("setup key");
    struct G {}
    impl TearDown for G {
        fn tear_down(&self) {
            println!("teardown key");
        }
    }

    Fixture::new(key(), G {})
}

fn reference(key: i32) -> &'static str {
    HASH.get(&key).unwrap_or(&"__NOTHING__")
}

fn reference_val() -> Fixture<&'static str, impl TearDown> {
    let mut key_fixture = key_val();
    let key = key_fixture.take();

    println!("setup reference");
    struct G {}
    impl TearDown for G {
        fn tear_down(&self) {
            println!("teardown reference");
        }
    }

    Fixture::new(reference(key), G {})
        .compose(key_fixture.guard())
}

struct A {}

fn moved() -> A {
    A {}
}

fn moved_val() -> Fixture<A, impl TearDown> {
    println!("setup moved");
    struct G {}
    impl TearDown for G {
        fn tear_down(&self) {
            println!("teardown moved");
        }
    }
    Fixture::new(moved(), G {})
}

#[test]
fn prova() {
    fn prova(copy: usize, reference: &str, _moved: A) {
        assert_eq!(copy, reference.len());
        println!("Prova done");
    }

    // Example no fixture
    let mut copy_fixture: Fixture<usize, _> = copy().into();
    let copy = copy_fixture.take();
    let mut reference_fixture = reference_val();
    let reference = reference_fixture.take();
    let mut moved_fixture = moved_val();
    let moved = moved_fixture.take();

    prova(copy, reference, moved)
}


fn main() {
    println!("Hello, world!");
}
