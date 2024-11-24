use crate::AtomicPrimitive;
use crate::Atomicable;
use crate::__internal;
use std::sync::atomic::{
    AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize,
};

macro_rules! impl_primitive_atomicable {
    ($($ty: ty => $atomic_ty: ty;)*) => {
        $(
            impl Atomicable for $ty {
                type Atomic = $atomic_ty;
            }
            impl AtomicPrimitive for $atomic_ty {
                type Base = $ty;

                fn into_inner(self) -> Self::Base {
                    <$atomic_ty>::into_inner(self)
                }
            }
            impl __internal::Sealed for $atomic_ty {}
        )*
    };
}

impl_primitive_atomicable! {
    u8 => AtomicU8;
    u16 => AtomicU16;
    u32 => AtomicU32;
    u64 => AtomicU64;
    usize => AtomicUsize;
    i8 => AtomicI8;
    i16 => AtomicI16;
    i32 => AtomicI32;
    i64 => AtomicI64;
    isize => AtomicIsize;
}

impl<T> __internal::Sealed for AtomicPtr<T> {}
impl<T> AtomicPrimitive for AtomicPtr<T> {
    type Base = *mut T;

    fn into_inner(self) -> Self::Base {
        AtomicPtr::into_inner(self)
    }
}
impl<T> Atomicable for *mut T {
    type Atomic = AtomicPtr<T>;
}
