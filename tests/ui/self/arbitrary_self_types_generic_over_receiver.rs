#![feature(arbitrary_self_types)]

use std::ops::{Receiver, Deref};

struct Foo(u32);
impl Foo {
    fn a(self: impl Receiver<Target=Self>) -> u32 {
        3
    }
    fn b(self: impl Deref<Target=Self>) -> u32 {
        self.0
    }
}

fn main() {
    let foo = Foo(1);
    foo.a();
    //~^ ERROR the trait bound
    foo.b();
    //~^ ERROR the trait bound
}
