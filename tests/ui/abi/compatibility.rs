//@ check-pass
//@ revisions: host
//@ revisions: i686
//@[i686] compile-flags: --target i686-unknown-linux-gnu
//@[i686] needs-llvm-components: x86
//@ revisions: x86-64
//@[x86-64] compile-flags: --target x86_64-unknown-linux-gnu
//@[x86-64] needs-llvm-components: x86
//@ revisions: x86-64-win
//@[x86-64-win] compile-flags: --target x86_64-pc-windows-msvc
//@[x86-64-win] needs-llvm-components: x86
//@ revisions: arm
//@[arm] compile-flags: --target arm-unknown-linux-gnueabi
//@[arm] needs-llvm-components: arm
//@ revisions: aarch64
//@[aarch64] compile-flags: --target aarch64-unknown-linux-gnu
//@[aarch64] needs-llvm-components: aarch64
//@ revisions: s390x
//@[s390x] compile-flags: --target s390x-unknown-linux-gnu
//@[s390x] needs-llvm-components: systemz
//@ revisions: mips
//@[mips] compile-flags: --target mips-unknown-linux-gnu
//@[mips] needs-llvm-components: mips
//@ revisions: mips64
//@[mips64] compile-flags: --target mips64-unknown-linux-gnuabi64
//@[mips64] needs-llvm-components: mips
//@ revisions: sparc
//@[sparc] compile-flags: --target sparc-unknown-linux-gnu
//@[sparc] needs-llvm-components: sparc
//@ revisions: sparc64
//@[sparc64] compile-flags: --target sparc64-unknown-linux-gnu
//@[sparc64] needs-llvm-components: sparc
//@ revisions: powerpc64
//@[powerpc64] compile-flags: --target powerpc64-unknown-linux-gnu
//@[powerpc64] needs-llvm-components: powerpc
//@ revisions: riscv
//@[riscv] compile-flags: --target riscv64gc-unknown-linux-gnu
//@[riscv] needs-llvm-components: riscv
//@ revisions: loongarch64
//@[loongarch64] compile-flags: --target loongarch64-unknown-linux-gnu
//@[loongarch64] needs-llvm-components: loongarch
//@[loongarch64] min-llvm-version: 18
//@ revisions: wasm
//@[wasm] compile-flags: --target wasm32-unknown-unknown
//@[wasm] needs-llvm-components: webassembly
//@ revisions: wasip1
//@[wasip1] compile-flags: --target wasm32-wasip1
//@[wasip1] needs-llvm-components: webassembly
//@ revisions: bpf
//@[bpf] compile-flags: --target bpfeb-unknown-none
//@[bpf] needs-llvm-components: bpf
//@ revisions: m68k
//@[m68k] compile-flags: --target m68k-unknown-linux-gnu
//@[m68k] needs-llvm-components: m68k
//@ revisions: csky
//@[csky] compile-flags: --target csky-unknown-linux-gnuabiv2
//@[csky] needs-llvm-components: csky
//@ revisions: nvptx64
//@[nvptx64] compile-flags: --target nvptx64-nvidia-cuda
//@[nvptx64] needs-llvm-components: nvptx
#![feature(rustc_attrs, unsized_fn_params, transparent_unions)]
#![cfg_attr(not(host), feature(no_core, lang_items), no_std, no_core)]
#![allow(unused, improper_ctypes_definitions, internal_features)]

// FIXME: some targets are broken in various ways.
// Hence there are `cfg` throughout this test to disable parts of it on those targets.
// sparc64: https://github.com/rust-lang/rust/issues/115336
// mips64: https://github.com/rust-lang/rust/issues/115404

#[cfg(host)]
use std::{
    any::Any, marker::PhantomData, mem::ManuallyDrop, num::NonZero, ptr::NonNull, rc::Rc, sync::Arc,
};

/// To work cross-target this test must be no_core.
/// This little prelude supplies what we need.
#[cfg(not(host))]
mod prelude {
    #[lang = "sized"]
    pub trait Sized {}

    #[lang = "hard_coded_receiver"]
    pub trait HardCodedReceiver {}
    impl<T: ?Sized> HardCodedReceiver for &T {}
    impl<T: ?Sized> HardCodedReceiver for &mut T {}

    #[lang = "copy"]
    pub trait Copy: Sized {}
    impl Copy for i32 {}
    impl Copy for f32 {}
    impl<T: ?Sized> Copy for &T {}
    impl<T: ?Sized> Copy for *const T {}
    impl<T: ?Sized> Copy for *mut T {}

    #[lang = "clone"]
    pub trait Clone: Sized {
        fn clone(&self) -> Self;
    }

    #[lang = "phantom_data"]
    pub struct PhantomData<T: ?Sized>;
    impl<T: ?Sized> Copy for PhantomData<T> {}

    #[lang = "unsafe_cell"]
    #[repr(transparent)]
    pub struct UnsafeCell<T: ?Sized> {
        value: T,
    }

    pub trait Any: 'static {}

    pub enum Option<T> {
        None,
        Some(T),
    }
    impl<T: Copy> Copy for Option<T> {}

    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    impl<T: Copy, E: Copy> Copy for Result<T, E> {}

    #[lang = "manually_drop"]
    #[repr(transparent)]
    pub struct ManuallyDrop<T: ?Sized> {
        value: T,
    }
    impl<T: Copy + ?Sized> Copy for ManuallyDrop<T> {}

    #[repr(transparent)]
    #[rustc_layout_scalar_valid_range_start(1)]
    #[rustc_nonnull_optimization_guaranteed]
    pub struct NonNull<T: ?Sized> {
        pointer: *const T,
    }
    impl<T: ?Sized> Copy for NonNull<T> {}

    #[repr(transparent)]
    #[rustc_layout_scalar_valid_range_start(1)]
    #[rustc_nonnull_optimization_guaranteed]
    pub struct NonZero<T>(T);

    // This just stands in for a non-trivial type.
    pub struct Vec<T> {
        ptr: NonNull<T>,
        cap: usize,
        len: usize,
    }

    pub struct Unique<T: ?Sized> {
        pub pointer: NonNull<T>,
        pub _marker: PhantomData<T>,
    }

    #[lang = "global_alloc_ty"]
    pub struct Global;

    #[lang = "owned_box"]
    pub struct Box<T: ?Sized, A = Global>(Unique<T>, A);

    #[repr(C)]
    struct RcBox<T: ?Sized> {
        strong: UnsafeCell<usize>,
        weak: UnsafeCell<usize>,
        value: T,
    }
    pub struct Rc<T: ?Sized, A = Global> {
        ptr: NonNull<RcBox<T>>,
        phantom: PhantomData<RcBox<T>>,
        alloc: A,
    }

    #[repr(C, align(8))]
    struct AtomicUsize(usize);
    #[repr(C)]
    struct ArcInner<T: ?Sized> {
        strong: AtomicUsize,
        weak: AtomicUsize,
        data: T,
    }
    pub struct Arc<T: ?Sized, A = Global> {
        ptr: NonNull<ArcInner<T>>,
        phantom: PhantomData<ArcInner<T>>,
        alloc: A,
    }
}
#[cfg(not(host))]
use prelude::*;

macro_rules! assert_abi_compatible {
    ($name:ident, $t1:ty, $t2:ty) => {
        mod $name {
            use super::*;
            // Declaring a `type` doesn't even check well-formedness, so we also declare a function.
            fn check_wf(_x: $t1, _y: $t2) {}
            // Test argument and return value, `Rust` and `C` ABIs.
            #[rustc_abi(assert_eq)]
            type TestRust = (fn($t1) -> $t1, fn($t2) -> $t2);
            #[rustc_abi(assert_eq)]
            type TestC = (extern "C" fn($t1) -> $t1, extern "C" fn($t2) -> $t2);
        }
    };
}

struct Zst;
impl Copy for Zst {}
impl Clone for Zst {
    fn clone(&self) -> Self {
        Zst
    }
}

#[repr(C)]
struct ReprC1<T: ?Sized>(T);
#[repr(C)]
struct ReprC2Int<T>(i32, T);
#[repr(C)]
struct ReprC2Float<T>(f32, T);
#[repr(C)]
struct ReprC4<T>(T, Vec<i32>, Zst, T);
#[repr(C)]
struct ReprC4Mixed<T>(T, f32, i32, T);
#[repr(C)]
enum ReprCEnum<T> {
    Variant1,
    Variant2(T),
}
#[repr(C)]
union ReprCUnion<T> {
    nothing: (),
    something: ManuallyDrop<T>,
}

macro_rules! test_abi_compatible {
    ($name:ident, $t1:ty, $t2:ty) => {
        mod $name {
            use super::*;
            assert_abi_compatible!(plain, $t1, $t2);
            // We also do some tests with differences in fields of `repr(C)` types.
            assert_abi_compatible!(repr_c_1, ReprC1<$t1>, ReprC1<$t2>);
            assert_abi_compatible!(repr_c_2_int, ReprC2Int<$t1>, ReprC2Int<$t2>);
            assert_abi_compatible!(repr_c_2_float, ReprC2Float<$t1>, ReprC2Float<$t2>);
            assert_abi_compatible!(repr_c_4, ReprC4<$t1>, ReprC4<$t2>);
            assert_abi_compatible!(repr_c_4mixed, ReprC4Mixed<$t1>, ReprC4Mixed<$t2>);
            assert_abi_compatible!(repr_c_enum, ReprCEnum<$t1>, ReprCEnum<$t2>);
            assert_abi_compatible!(repr_c_union, ReprCUnion<$t1>, ReprCUnion<$t2>);
        }
    };
}

// Compatibility of pointers.
test_abi_compatible!(ptr_mut, *const i32, *mut i32);
test_abi_compatible!(ptr_pointee, *const i32, *const Vec<i32>);
test_abi_compatible!(ref_mut, &i32, &mut i32);
test_abi_compatible!(ref_ptr, &i32, *const i32);
test_abi_compatible!(box_ptr, Box<i32>, *const i32);
test_abi_compatible!(nonnull_ptr, NonNull<i32>, *const i32);
test_abi_compatible!(fn_fn, fn(), fn(i32) -> i32);

// Compatibility of integer types.
test_abi_compatible!(char_uint, char, u32);
#[cfg(target_pointer_width = "32")]
test_abi_compatible!(isize_int, isize, i32);
#[cfg(target_pointer_width = "64")]
test_abi_compatible!(isize_int, isize, i64);

// Compatibility of 1-ZST.
test_abi_compatible!(zst_unit, Zst, ());
#[cfg(not(any(target_arch = "sparc64")))]
test_abi_compatible!(zst_array, Zst, [u8; 0]);
test_abi_compatible!(nonzero_int, NonZero<i32>, i32);

// `#[repr(C)]` enums should not change ABI based on individual variant inhabitedness.
// (However, this is *not* a guarantee. We only guarantee same layout, not same ABI.)
enum Void {}
test_abi_compatible!(repr_c_enum_void, ReprCEnum<Void>, ReprCEnum<ReprCUnion<Void>>);

// `DispatchFromDyn` relies on ABI compatibility.
// This is interesting since these types are not `repr(transparent)`. So this is not part of our
// public ABI guarantees, but is relied on by the compiler.
test_abi_compatible!(rc, Rc<i32>, *mut i32);
test_abi_compatible!(arc, Arc<i32>, *mut i32);

// `repr(transparent)` compatibility.
#[repr(transparent)]
struct Wrapper1<T: ?Sized>(T);
#[repr(transparent)]
struct Wrapper2<T: ?Sized>((), Zst, T);
#[repr(transparent)]
struct Wrapper3<T>(T, [u8; 0], PhantomData<u64>);
#[repr(transparent)]
union WrapperUnion<T> {
    nothing: (),
    something: ManuallyDrop<T>,
}

macro_rules! test_transparent {
    ($name:ident, $t:ty) => {
        mod $name {
            use super::*;
            test_abi_compatible!(wrap1, $t, Wrapper1<$t>);
            test_abi_compatible!(wrap2, $t, Wrapper2<$t>);
            test_abi_compatible!(wrap3, $t, Wrapper3<$t>);
            test_abi_compatible!(wrap4, $t, WrapperUnion<$t>);
        }
    };
}

test_transparent!(simple, i32);
test_transparent!(reference, &'static i32);
test_transparent!(zst, Zst);
test_transparent!(unit, ());
test_transparent!(enum_, Option<i32>);
test_transparent!(enum_niched, Option<&'static i32>);
#[cfg(not(any(target_arch = "mips64", target_arch = "sparc64")))]
mod tuples {
    use super::*;
    // mixing in some floats since they often get special treatment
    test_transparent!(pair, (i32, f32));
    // chosen to fit into 64bit
    test_transparent!(triple, (i8, i16, f32));
    // Pure-float types that are not ScalarPair seem to be tricky.
    test_transparent!(triple_f32, (f32, f32, f32));
    test_transparent!(triple_f64, (f64, f64, f64));
    // and also something that's larger than 2 pointers
    test_transparent!(tuple, (i32, f32, i64, f64));
}
// Some targets have special rules for arrays.
#[cfg(not(any(target_arch = "mips64", target_arch = "sparc64")))]
mod arrays {
    use super::*;
    test_transparent!(empty_array, [u32; 0]);
    test_transparent!(empty_1zst_array, [u8; 0]);
    test_transparent!(small_array, [i32; 2]); // chosen to fit into 64bit
    test_transparent!(large_array, [i32; 16]);
}

// Some tests with unsized types (not all wrappers are compatible with that).
macro_rules! test_transparent_unsized {
    ($name:ident, $t:ty) => {
        mod $name {
            use super::*;
            assert_abi_compatible!(wrap1, $t, Wrapper1<$t>);
            assert_abi_compatible!(wrap1_reprc, ReprC1<$t>, ReprC1<Wrapper1<$t>>);
            assert_abi_compatible!(wrap2, $t, Wrapper2<$t>);
            assert_abi_compatible!(wrap2_reprc, ReprC1<$t>, ReprC1<Wrapper2<$t>>);
        }
    };
}

#[cfg(not(any(target_arch = "mips64", target_arch = "sparc64")))]
mod unsized_ {
    use super::*;
    test_transparent_unsized!(str_, str);
    test_transparent_unsized!(slice, [u8]);
    test_transparent_unsized!(slice_with_prefix, (usize, [u8]));
    test_transparent_unsized!(dyn_trait, dyn Any);
}

// RFC 3391 <https://rust-lang.github.io/rfcs/3391-result_ffi_guarantees.html>.
macro_rules! test_nonnull {
    ($name:ident, $t:ty) => {
        mod $name {
            use super::*;
            test_abi_compatible!(option, Option<$t>, $t);
            test_abi_compatible!(result_err_unit, Result<$t, ()>, $t);
            test_abi_compatible!(result_ok_unit, Result<(), $t>, $t);
            test_abi_compatible!(result_err_zst, Result<$t, Zst>, $t);
            test_abi_compatible!(result_ok_zst, Result<Zst, $t>, $t);
            test_abi_compatible!(result_err_arr, Result<$t, [i8; 0]>, $t);
            test_abi_compatible!(result_ok_arr, Result<[i8; 0], $t>, $t);
        }
    }
}

test_nonnull!(ref_, &i32);
test_nonnull!(mut_, &mut i32);
test_nonnull!(ref_unsized, &[i32]);
test_nonnull!(mut_unsized, &mut [i32]);
test_nonnull!(fn_, fn());
test_nonnull!(nonnull, NonNull<i32>);
test_nonnull!(nonnull_unsized, NonNull<dyn Any>);
test_nonnull!(non_zero, NonZero<i32>);

fn main() {}
