//@ check-pass

struct Foo<T>(T);

impl<'a> Foo<&'a ()> {
    fn call_me(self: &Self) -> &() { self.0 }
    fn call_me2(self: &Foo<&'a ()>) -> &() { self.0 }
}

fn main() { }
