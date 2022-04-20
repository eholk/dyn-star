#![no_std]

use core::{marker::PhantomData, ptr::NonNull};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub struct Dyn<'a, T: ?Sized> {
    ptr: NonNull<T>,
    drop: unsafe fn(NonNull<T>),
    marker: PhantomData<&'a mut T>,
}

mod ctors;
mod traits;
