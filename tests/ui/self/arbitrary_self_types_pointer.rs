#![feature(receiver_trait)]

struct A;

impl A {
    fn m(self: *const Self) {}
    //~^ ERROR: invalid `self` parameter type
}

trait B {
    fn bm(self: *const Self) {}
    //~^ ERROR: invalid `self` parameter type
}

fn main() {
    let a = A;
    let ptr = &a as *const A;
    ptr.m();
}
