// run-pass

#![feature(receiver_trait)]

use std::ptr::NonNull;

struct Foo(usize);

impl Foo {
  fn bar(self: NonNull<Self>) -> i32 { 3 }
  fn as_mut(self: &mut NonNull<Self>) -> i32 { 4 }
}

fn main() {
  let mut foo = Foo(3);
  let mut ptr = std::ptr::NonNull::new(&mut foo as *mut Foo).unwrap();
  // FIXME: Should call Foo::as_mut but warn about ambiguity
  // once we have implemented warnings
  assert_eq!(ptr.as_mut(), 4);
  assert_eq!(ptr.bar(), 3);
}
