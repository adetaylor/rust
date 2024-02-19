// run-pass

#![feature(receiver_trait)]

use std::ptr::NonNull;

struct Foo(usize);

impl Foo {
  fn foo(self: &NonNull<Self>) -> i32 { 3 }
  fn bar(self: &mut NonNull<Self>) -> i32 { 3 }
  fn baz(self: NonNull<Self>) -> i32 { 3 }
  fn as_mut(self: &mut NonNull<Self>) -> i32 { 4 }
}

fn main() {
  let mut foo = Foo(3);
  let mut ptr = std::ptr::NonNull::new(&mut foo as *mut Foo).unwrap();
  assert_eq!(ptr.as_mut(), 4);
  //~^ ambiguous targets
  assert_eq!(ptr.foo(), 3);
  assert_eq!(ptr.bar(), 3);
  assert_eq!(ptr.baz(), 3);
}
