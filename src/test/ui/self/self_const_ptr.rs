// run-pass

struct Foo(u32);

impl Foo {
    fn contents(*const self) -> u32 {
        unsafe { self.as_ref() }.unwrap().0
    }
}

fn main() {
    let foo = Foo(3);
    assert_eq!(3, foo.contents());
}
