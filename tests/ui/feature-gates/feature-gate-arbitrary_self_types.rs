struct Foo;

impl Foo {
    fn foo(self: *const Self) {}
    //~^ ERROR A raw pointer `*const Foo` cannot be used as the type of `self` without
}

trait Bar {
    fn bar(self: *const Self);
    //~^ ERROR A raw pointer `*const Self` cannot be used as the type of `self` without
}

impl Bar for () {
    fn bar(self: *const Self) {}
    //~^ ERROR A raw pointer `*const ()` cannot be used as the type of `self` without
}

fn main() {}
