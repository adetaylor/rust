// run-pass

#![feature(receiver_trait)]

use std::rc::{Rc, Weak};

struct Foo(usize);

impl Foo {
  fn foo(self: Weak<Self>) -> i32 { 3 }
  fn bar(self: &Weak<Self>) -> i32 { 4 }
  fn baz(self: &mut Weak<Self>) -> i32 { 5 }

  fn as_ptr(self: &Weak<Self>) -> i32 { 6 }
}

fn main() {
  let foo = Rc::new(Foo(3));
  let mut foo_weak = Rc::downgrade(&foo);

  assert_eq!(foo_weak.as_ptr(), 6);
  //~^ ambiguous targets

  assert_eq!(foo_weak.bar(), 4);
  assert_eq!(foo_weak.baz(), 5);
  assert_eq!(foo_weak.foo(), 3);
}
