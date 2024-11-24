mod atomic;
mod primitive_atomicable;

pub use atomic::Atomic;
pub use atomics_macro::atomic_enum;

pub trait Atomicable {
    type Atomic: AtomicType<Base = Self>;
}

pub trait AtomicType: Send + Sync + From<Self::Base> {
    type Base;
    type Primitive: AtomicPrimitive;

    fn into_inner(self) -> Self::Base;
}

impl<T: AtomicPrimitive> AtomicType for T {
    type Base = <T as AtomicPrimitive>::Base;
    type Primitive = Self;

    fn into_inner(self) -> Self::Base {
        <Self as AtomicPrimitive>::into_inner(self)
    }
}

pub trait AtomicPrimitive: __internal::Sealed + Send + Sync + From<Self::Base> {
    type Base;

    fn into_inner(self) -> Self::Base;
}

mod __internal {
    pub trait Sealed {}
}
