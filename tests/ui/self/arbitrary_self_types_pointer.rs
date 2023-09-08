// run-pass

#![feature(receiver_trait)]

struct A;

impl A {
    fn m(self: *const Self) {}
}

trait B {
    fn bm(self: *const Self) {}
}

fn main() {
    let a = A;
    let ptr = &a as *const A;
    ptr.m();
}
