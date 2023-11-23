// run-pass

#![feature(receiver_trait)]
#![allow(dead_code)]

struct Foo;
struct Ptr<T>(T);

impl<T> std::ops::Receiver for Ptr<T> {
    type Target = T;
}

impl<T> Ptr<T> {
    fn method_ref(&self) -> u32 { 2 }
    fn method_ref2(&self) -> u32 { 2 }
    fn method_value(self) -> u32 { 2 }
}

impl Foo {
    fn method_ref(self: Ptr<Self>) -> u32 { 1 }
    fn method_ref2(self: &Ptr<Self>) -> u32 { 1 }
    fn method_value(self: Ptr<Self>) -> u32 { 1 }
}

fn main() {
    let a = Ptr(Foo);
    assert_eq!(a.method_ref(), 1);
    let a = Ptr(Foo);
    assert_eq!(a.method_ref2(), 1);
    let a = Ptr(Foo);
    assert_eq!(a.method_value(), 1);
}
