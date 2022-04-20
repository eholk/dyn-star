#![no_std]

use core::{marker::PhantomData, ptr::NonNull};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub struct DynMut<'a, T: ?Sized> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) drop: unsafe fn(NonNull<T>),
    pub(crate) marker: PhantomData<&'a mut T>,
}

pub struct Dyn<'a, T: ?Sized> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) drop: unsafe fn(NonNull<T>),
    pub(crate) marker: PhantomData<&'a mut T>,
}

mod ctors;
mod traits;
