use crate::AtomicType;
use crate::Atomicable;
use std::ops::Deref;

pub struct Atomic<T: Atomicable> {
    value: T::Atomic,
}

impl<T: Atomicable> Deref for Atomic<T> {
    type Target = T::Atomic;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Atomicable> From<T> for Atomic<T> {
    fn from(value: T) -> Self {
        Self {
            value: T::Atomic::from(value),
        }
    }
}

impl<T: Atomicable> AtomicType for Atomic<T> {
    type Base = T;
    type Primitive = <T::Atomic as AtomicType>::Primitive;

    fn into_inner(self) -> Self::Base {
        self.value.into_inner()
    }
}

impl<T: Atomicable> Atomic<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: T::Atomic::from(value),
        }
    }
    pub fn into_inner(self) -> T {
        T::Atomic::into_inner(self.value)
    }
}
