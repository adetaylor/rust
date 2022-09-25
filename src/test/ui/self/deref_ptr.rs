// run-pass

#![feature(arbitrary_self_types)]

struct Foo(u32);

impl Foo {
    fn contents(self: *const Foo) -> u32 {
        unsafe { self.as_ref() }.unwrap().0
    }
}

struct FooContainer<'a>(&'a Foo);

impl core::ops::DerefPtr for FooContainer<'_> {
    type Target = Foo;
    fn deref(&self) -> *const Self::Target {
        self.0 as *const Self::Target
    }
}

fn main() {
    let foo = Foo(3);
    let foo_container = FooContainer(&foo);
    assert_eq!(3, foo_container.contents());
}
