// run-pass

#![feature(receiver_trait)]

pub struct A;

impl A {
    pub fn f(self: &B<Self>) -> i32 { 1 }
}

pub struct B<T>(T);

impl<T> core::ops::Receiver for B<T> {
    type Target = A;
}

impl<T> B<T> {
    pub fn f(&self) -> i32 {
        2
    }
}

fn main() {
    assert_eq!(B(A).f(), 1);
}
