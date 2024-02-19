// run-pass

#![feature(receiver_trait)]
#![allow(dead_code)]

struct Foo;
struct InnerPtr<T>(T);
struct OuterPtr<T>(T);

impl<T> std::ops::Receiver for InnerPtr<T> {
    type Target = T;
}

impl<T> std::ops::Receiver for OuterPtr<T> {
    type Target = T;
}

impl Foo {
    fn m(self: &OuterPtr<InnerPtr<Self>>) -> u32 { 1 }
}

impl<T> InnerPtr<T> {
    fn m(self: &OuterPtr<Self>) -> u32 { 2 }
}

impl<T> OuterPtr<T> {
    fn m(self) -> u32 { 3 }
}

fn main() {
    let  a = OuterPtr(InnerPtr(Foo));
    assert_eq!(a.m(), 1);
    //~^ ambiguous target
}
