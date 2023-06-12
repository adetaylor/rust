/// Used for immutable dereferencing operations, like `*v`.
///
/// In addition to being used for explicit dereferencing operations with the
/// (unary) `*` operator in immutable contexts, `Deref` is also used implicitly
/// by the compiler in many circumstances. This mechanism is called
/// ['`Deref` coercion'][more]. In mutable contexts, [`DerefMut`] is used.
///
/// Implementing `Deref` for smart pointers makes accessing the data behind them
/// convenient, which is why they implement `Deref`. On the other hand, the
/// rules regarding `Deref` and [`DerefMut`] were designed specifically to
/// accommodate smart pointers. Because of this, **`Deref` should only be
/// implemented for smart pointers** to avoid confusion.
///
/// For similar reasons, **this trait should never fail**. Failure during
/// dereferencing can be extremely confusing when `Deref` is invoked implicitly.
///
/// # More on `Deref` coercion
///
/// If `T` implements `Deref<Target = U>`, and `x` is a value of type `T`, then:
///
/// * In immutable contexts, `*x` (where `T` is neither a reference nor a raw pointer)
///   is equivalent to `*Deref::deref(&x)`.
/// * Values of type `&T` are coerced to values of type `&U`
/// * `T` implicitly implements all the (immutable) methods of the type `U`.
///
/// For more details, visit [the chapter in *The Rust Programming Language*][book]
/// as well as the reference sections on [the dereference operator][ref-deref-op],
/// [method resolution] and [type coercions].
///
/// [book]: ../../book/ch15-02-deref.html
/// [more]: #more-on-deref-coercion
/// [ref-deref-op]: ../../reference/expressions/operator-expr.html#the-dereference-operator
/// [method resolution]: ../../reference/expressions/method-call-expr.html
/// [type coercions]: ../../reference/type-coercions.html
///
/// # Examples
///
/// A struct with a single field which is accessible by dereferencing the
/// struct.
///
/// ```
/// use std::ops::Deref;
///
/// struct DerefExample<T> {
///     value: T
/// }
///
/// impl<T> Deref for DerefExample<T> {
///     type Target = T;
///
///     fn deref(&self) -> &Self::Target {
///         &self.value
///     }
/// }
///
/// let x = DerefExample { value: 'a' };
/// assert_eq!('a', *x);
/// ```
#[lang = "deref"]
#[doc(alias = "*")]
#[doc(alias = "&*")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "Deref"]
pub trait Deref {
    /// The resulting type after dereferencing.
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "deref_target"]
    #[lang = "deref_target"]
    type Target: ?Sized;

    /// Dereferences the value.
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "deref_method"]
    fn deref(&self) -> &Self::Target;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Deref for &T {
    type Target = T;

    #[rustc_diagnostic_item = "noop_method_deref"]
    fn deref(&self) -> &T {
        *self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !DerefMut for &T {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Deref for &mut T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}

/// Used for mutable dereferencing operations, like in `*v = 1;`.
///
/// In addition to being used for explicit dereferencing operations with the
/// (unary) `*` operator in mutable contexts, `DerefMut` is also used implicitly
/// by the compiler in many circumstances. This mechanism is called
/// ['`Deref` coercion'][more]. In immutable contexts, [`Deref`] is used.
///
/// Implementing `DerefMut` for smart pointers makes mutating the data behind
/// them convenient, which is why they implement `DerefMut`. On the other hand,
/// the rules regarding [`Deref`] and `DerefMut` were designed specifically to
/// accommodate smart pointers. Because of this, **`DerefMut` should only be
/// implemented for smart pointers** to avoid confusion.
///
/// For similar reasons, **this trait should never fail**. Failure during
/// dereferencing can be extremely confusing when `DerefMut` is invoked
/// implicitly.
///
/// # More on `Deref` coercion
///
/// If `T` implements `DerefMut<Target = U>`, and `x` is a value of type `T`,
/// then:
///
/// * In mutable contexts, `*x` (where `T` is neither a reference nor a raw pointer)
///   is equivalent to `*DerefMut::deref_mut(&mut x)`.
/// * Values of type `&mut T` are coerced to values of type `&mut U`
/// * `T` implicitly implements all the (mutable) methods of the type `U`.
///
/// For more details, visit [the chapter in *The Rust Programming Language*][book]
/// as well as the reference sections on [the dereference operator][ref-deref-op],
/// [method resolution] and [type coercions].
///
/// [book]: ../../book/ch15-02-deref.html
/// [more]: #more-on-deref-coercion
/// [ref-deref-op]: ../../reference/expressions/operator-expr.html#the-dereference-operator
/// [method resolution]: ../../reference/expressions/method-call-expr.html
/// [type coercions]: ../../reference/type-coercions.html
///
/// # Examples
///
/// A struct with a single field which is modifiable by dereferencing the
/// struct.
///
/// ```
/// use std::ops::{Deref, DerefMut};
///
/// struct DerefMutExample<T> {
///     value: T
/// }
///
/// impl<T> Deref for DerefMutExample<T> {
///     type Target = T;
///
///     fn deref(&self) -> &Self::Target {
///         &self.value
///     }
/// }
///
/// impl<T> DerefMut for DerefMutExample<T> {
///     fn deref_mut(&mut self) -> &mut Self::Target {
///         &mut self.value
///     }
/// }
///
/// let mut x = DerefMutExample { value: 'a' };
/// *x = 'b';
/// assert_eq!('b', x.value);
/// ```
#[lang = "deref_mut"]
#[doc(alias = "*")]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait DerefMut: Deref {
    /// Mutably dereferences the value.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn deref_mut(&mut self) -> &mut Self::Target;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> DerefMut for &mut T {
    fn deref_mut(&mut self) -> &mut T {
        *self
    }
}

/// Indicates that a struct can be used as a method receiver.
/// That is, a type can use this type as a type of `self`, like this:
/// ```
/// use std::ops::Receiver;
///
/// struct SmartPointer<T>(T);
///
/// impl<T> Receiver for SmartPointer<T> {
///    type Target = T;
/// }
///
/// struct MyContainedType;
///
/// impl MyContainedType {
///   fn method(self: MySmartPointer<Self>) {
///     // ...
///   }
/// }
///
/// fn main() {
///   let ptr = SmartPointer(MyContainedType);
///   ptr.method();
/// }
/// ```
/// This trait is automatically implemented for any type which implements
/// [`Deref`], which includes stdlib pointer types like `Box<T>`,`Rc<T>`, `&T`,
/// and `Pin<P>`. For that reason, it's relatively rare to need to
/// implement this directly. You'll typically do this only if you need
/// to implement a smart pointer type which can't implement [`Deref`]; perhaps
/// because you're interfacing with another programming language and can't
/// guarantee that references comply with Rust's aliasing rules.
///
/// When looking for method candidates, Rust will explore a chain of possible
/// `Receiver`s, so for example each of the following methods work:
/// ```
/// use std::boxed::Box;
/// use std::rc::Rc;
///
/// // Both Box and Rc (indirectly) implement Receiver
///
/// struct MyContainedType;
///
/// fn main() {
///   let t = Rc::new(Box::new(MyContainedType));
///   t.method_a();
///   t.method_b();
///   t.method_c();
///   t.method_d();
/// }
///
/// impl MyContainedType {
///   fn method_a(self) {
///
///   }
///   fn method_b(&self) {
///
///   }
///   fn method_c(self: Box<Self>) {
///
///   }
///   fn method_d(self: Rc<Box<Self>>) {
///
///   }
/// }
/// ```
#[lang = "receiver"]
#[unstable(feature = "receiver_trait", issue = "none")]
pub trait Receiver {
    /// The target type on which the method may be called.
    #[cfg(not(bootstrap))]
    #[rustc_diagnostic_item = "receiver_target"]
    #[lang = "receiver_target"]
    #[unstable(feature = "receiver_trait", issue = "none")]
    type Target: ?Sized;
}

#[cfg(bootstrap)]
#[unstable(feature = "receiver_trait", issue = "none")]
impl<T: ?Sized> Receiver for &T {
    #[cfg(not(bootstrap))]
    type Target = T;
}

#[cfg(bootstrap)]
#[unstable(feature = "receiver_trait", issue = "none")]
impl<T: ?Sized> Receiver for &mut T {
    #[cfg(not(bootstrap))]
    type Target = T;
}

#[cfg(not(bootstrap))]
#[unstable(feature = "receiver_trait", issue = "none")]
impl<P: ?Sized, T: ?Sized> Receiver for P
where
    P: Deref<Target = T>,
{
    type Target = T;
}
