struct A {}

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
}

fn counter_get() -> usize {
    let mut val = COUNTER.lock().unwrap();
    *val += 1;
    *val
}

struct Fixture<T> {
    inner: Option<T>,
    counter: usize
}

impl<T> Fixture<T> {
    pub fn take(&mut self) -> T {
        self.inner.take().unwrap()
    }
}

impl<T:Sized> From<T> for Fixture<T> {
    fn from(inner: T) -> Self {
        let counter = counter_get();
        println!("setup [{}]", counter);
        Fixture { inner: Some(inner), counter }
    }
}

impl<T> Drop for Fixture<T> {
    fn drop(&mut self) {
        println!("teardown [{}]", self.counter)
    }
}

fn copy() -> usize {
    4
}

fn copy_val() -> Fixture<usize> {
    copy().into()
}

fn reference() -> &'static str {
    "ciao"
}

fn reference_val() -> Fixture<&'static str> {
    reference().into()
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
