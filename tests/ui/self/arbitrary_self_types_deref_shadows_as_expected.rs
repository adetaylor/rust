// run-pass

pub struct A;

impl A {
    pub fn f(&self) -> i32 { 1 }
}

pub struct B(A);

impl core::ops::Deref for B {
    type Target = A;
    fn deref(&self) -> &A {
        &self.0
    }
}

impl B {
    pub fn f(&self) -> i32 {
        2
    }
}

fn main() {
    assert_eq!(B(A).f(), 2);
}
