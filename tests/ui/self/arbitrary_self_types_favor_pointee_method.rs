// run-pass

#![feature(receiver_trait)]
#![allow(dead_code)]

struct Foo;
struct Ptr<T>(T);

impl<T> std::ops::Receiver for Ptr<T> {
    type Target = T;
}

impl Foo {
    fn method_mutref(self: &mut Ptr<Self>) -> u32 { 1 }
    fn method_mutref2(self: &Ptr<Self>) -> u32 { 1 }
    fn method_mutref3(self: Ptr<Self>) -> u32 { 1 }
    fn method_ref(self: &Ptr<Self>) -> u32 { 1 }
    fn method_ref2(self: Ptr<Self>) -> u32 { 1 }
    fn method_ref3(self: &mut Ptr<Self>) -> u32 { 1 }
    fn method_value(self: Ptr<Self>) -> u32 { 1 }
    fn method_value2(self: &Ptr<Self>) -> u32 { 1 }
    fn method_value3(self: &mut Ptr<Self>) -> u32 { 1 }
}

// Imagine these are added later
impl<T> Ptr<T> {
    fn method_mutref(&mut self) -> u32 { 2 }
    fn method_mutref2(&mut self) -> u32 { 2 }
    fn method_mutref3(&mut self) -> u32 { 2 }
    fn method_ref(&self) -> u32 { 2 }
    fn method_ref2(&self) -> u32 { 2 }
    fn method_ref3(&self) -> u32 { 2 }
    fn method_value(self) -> u32 { 2 }
    fn method_value2(self) -> u32 { 2 }
    fn method_value3(self) -> u32 { 2 }
}

fn main() {
    let mut a = Ptr(Foo);
    assert_eq!(a.method_mutref(), 1);
    //~^ ambiguous target
    let a = Ptr(Foo);
    assert_eq!(a.method_mutref2(), 1);
    let a = Ptr(Foo);
    assert_eq!(a.method_mutref3(), 1);

    let a = Ptr(Foo);
    assert_eq!(a.method_ref(), 1);
    //~^ ambiguous target
    let a = Ptr(Foo);
    assert_eq!(a.method_ref2(), 1);
    let mut a = Ptr(Foo);
    assert_eq!(a.method_ref3(), 1);
    //~^ ambiguous target

    let a = Ptr(Foo);
    assert_eq!(a.method_value(), 1);
    //~^ ambiguous target
    let a = Ptr(Foo);
    assert_eq!(a.method_value2(), 1);
    //~^ ambiguous target
    let mut a = Ptr(Foo);
    assert_eq!(a.method_value3(), 1);
    //~^ ambiguous target
}
