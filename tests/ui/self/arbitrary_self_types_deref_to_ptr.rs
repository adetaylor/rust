// run-pass

struct Foo;

struct Ptr<T: ?Sized>(*const T);

impl<T: ?Sized> std::ops::Deref for Ptr<T>{
    // Checks we can call methods on Ptr<T> even if
    // Ptr: Deref<Target = *const T>
    // rather than Ptr: Deref<Target = T>
    type Target = *const T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Foo {
    fn bar(self: &Ptr<Self>) -> i32 { 3 }
}

trait SomeTrait {
    fn baz(self: &Ptr<Self>) -> i32;
}

impl SomeTrait for Foo {
    fn baz(self: &Ptr<Self>) -> i32 { 4 }
}

fn main() {
    let foo = Box::new(Foo);
    let ptr = Ptr(Box::into_raw(foo));
    assert_eq!(3, ptr.bar());
    assert_eq!(4, ptr.baz());
    unsafe { drop(Box::from_raw(ptr.0 as *mut Foo)); }
}
