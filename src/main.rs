use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

struct A {}

lazy_static! {
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
}

fn counter_get() -> usize {
    let mut val = COUNTER.lock().unwrap();
    *val += 1;
    *val
}

struct FixtureGuard {
    counter: usize,
    parents: Vec<FixtureGuard>,
}

impl Drop for FixtureGuard {
    fn drop(&mut self) {
        println!("teardown [{}]", self.counter)
    }
}

impl FixtureGuard {
    fn new(counter: usize) -> Self {
        FixtureGuard { counter, parents: vec![] }
    }
}

impl Default for FixtureGuard {
    fn default() -> Self {
        Self::new(counter_get())
    }
}

struct Fixture<T> {
    inner: Option<T>,
    guard: FixtureGuard,
}

impl<T> Fixture<T> {
    fn new(inner: T, guard: FixtureGuard) -> Self {
        println!("setup [{}]", guard.counter);
        Fixture { inner: Some(inner), guard }
    }
    pub fn take(&mut self) -> T {
        self.inner.take().unwrap()
    }

    fn push<S>(mut self, parent: Fixture<S>) -> Self {
        self.guard.parents.push(parent.guard);
        self
    }
}

impl<T: Sized> From<T> for Fixture<T> {
    fn from(inner: T) -> Self {
        Fixture::new(inner, Default::default())
    }
}

fn copy() -> usize {
    4
}

fn copy_val() -> Fixture<usize> {
    copy().into()
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

fn key_val() -> Fixture<i32> {
    key().into()
}

fn reference(key: i32) -> &'static str {
    HASH.get(&key).unwrap_or(&"__NOTHING__")
}

fn reference_val() -> Fixture<&'static str> {
    let mut key_fixture = key_val();
    let key = key_fixture.take();

    Fixture::from(reference(key)).push(key_fixture)
}

fn moved() -> A {
    A {}
}

fn moved_val() -> Fixture<A> {
    moved().into()
}

#[test]
fn prova() {
    fn prova(copy: usize, reference: &str, _moved: A) {
        assert_eq!(copy, reference.len());
        println!("Prova done");
    }

    let mut copy_fixture = copy_val();
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
