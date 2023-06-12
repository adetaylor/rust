// check-pass

#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;

struct Struct { }

struct Wrap<T, P>(T, PhantomData<P>);

impl<T, P> Deref for Wrap<T, P> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl Struct {
    // Test using multiple `&Self`:

    fn wrap_ref_Self_ref_Self(self: Wrap<&Self, &Self>, f: &u8) -> &u8 {
        f
    }

    fn box_wrap_ref_Self_ref_Self(self: Box<Wrap<&Self, &Self>>, f: &u32) -> &u32 {
        f
    }

    fn pin_wrap_ref_Self_ref_Self(self: Pin<Wrap<&Self, &Self>>, f: &u32) -> &u32 {
        f
    }

    fn box_box_wrap_ref_Self_ref_Self(self: Box<Box<Wrap<&Self, &Self>>>, f: &u32) -> &u32 {
        f
    }

    fn box_pin_wrap_ref_Self_ref_Self(self: Box<Pin<Wrap<&Self, &Self>>>, f: &u32) -> &u32 {
        f
    }
}

fn main() { }
