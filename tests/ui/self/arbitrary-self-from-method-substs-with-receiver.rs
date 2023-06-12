#![feature(arbitrary_self_types)]

use std::ops::{Receiver, Deref};

struct SmartPtr<'a, T: ?Sized>(&'a T);

impl<'a, T: ?Sized> Deref for SmartPtr<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: ?Sized> Clone for SmartPtr<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'a, T: ?Sized> Copy for SmartPtr<'a, T> {
}

struct Foo(u32);
impl Foo {
    fn a<R: Receiver<Target=Self>>(self: R) -> u32 {
        // No way to access the obj if we're just generic
        // over Receiver
        2
    }
    fn b<R: Deref<Target=Self>>(self: R) -> u32 {
        // but this works, so test that it continues to do so.
        self.0
    }
    fn c(self: impl Receiver<Target=Self>) -> u32 {
        3
    }
    fn d(self: impl Deref<Target=Self>) -> u32 {
        self.0
    }
}

fn main() {
    let foo = Foo(1);
    assert_eq!((&foo).a::<&Foo>(), 2);
    assert_eq!((&foo).b::<&Foo>(), 1);
    assert_eq!((&foo).a(), 2);
    assert_eq!((&foo).b(), 1);
    assert_eq!((&foo).c(), 3);
    assert_eq!((&foo).d(), 1);
    assert_eq!(foo.a::<&Foo>(), 2);
    //~^ ERROR mismatched types
    assert_eq!(foo.b::<&Foo>(), 1);
    //~^ ERROR mismatched types
    let smart_ptr = SmartPtr(&foo);
    assert_eq!(smart_ptr.a(), 2);
    assert_eq!(smart_ptr.b(), 1);
    assert_eq!(smart_ptr.c(), 3);
    assert_eq!(smart_ptr.d(), 1);
    assert_eq!(smart_ptr.a::<&Foo>(), 2);
    //~^ ERROR mismatched types
    assert_eq!(smart_ptr.b::<&Foo>(), 1);
    //~^ ERROR mismatched types
}
