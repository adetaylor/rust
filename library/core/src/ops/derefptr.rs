/// Like [`Deref`] but dereferences to a raw pointer instead of a reference.
///
/// This should be used sparingly, for cases where a reference may not be
/// created to the target.
#[cfg_attr(not(bootstrap), lang = "deref_ptr")]
#[unstable(feature = "arbitrary_self_types", issue = "44874")]
pub trait DerefPtr {
    /// The resulting type after dereferencing.
    #[cfg_attr(not(bootstrap), lang = "deref_ptr_target")]
    type Target: ?Sized;

    /// Dereferences the value.
    #[must_use]
    fn deref(&self) -> *const Self::Target;
}

/// Like [`DerefMut`] but dereferences to a mutable raw pointer.
#[cfg_attr(not(bootstrap), lang = "deref_ptr_mut")]
#[unstable(feature = "arbitrary_self_types", issue = "44874")]
pub trait DerefPtrMut: DerefPtr {
    /// Dereferences the value.
    fn deref_mut(&mut self) -> *mut Self::Target;
}
