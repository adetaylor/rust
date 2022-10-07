#![feature(arbitrary_self_types)]
#![feature(receiver_trait)]

// run-pass

struct Foo {
    x: i32,
}

impl Foo {
    fn x(self: Bar<Self>) -> i32 {
        unsafe { self.0.as_ref() }.unwrap().x
    }
}

struct Bar<T>(*const T);

impl<T> core::ops::Receiver for Bar<T> {
    type Target = T;
}

fn main() {
    let foo = Foo { x: 12 };
    let bar = Bar(&foo as *const Foo);
    assert_eq!(12, bar.x());
}
