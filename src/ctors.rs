use crate::Dyn;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
    pin::Pin,
    ptr::{drop_in_place, NonNull},
};

impl<'a, T: ?Sized> From<&'a mut T> for Dyn<'a, T> {
    fn from(r: &'a mut T) -> Self {
        Dyn {
            ptr: NonNull::from(r),
            drop: |_| (),
            marker: PhantomData,
        }
    }
}

impl<'a, T: ?Sized> Dyn<'a, T> {
    pub fn pinned<P>(p: Pin<P>) -> Pin<Self>
    where
        P: Deref<Target = T>,
        Self: From<P>,
    {
        unsafe { Pin::new_unchecked(Self::from(Pin::into_inner_unchecked(p))) }
    }
}

impl<'a, T: ?Sized> Dyn<'a, T> {
    /// # Safety
    ///
    /// This function runs the destructor of the `ManuallyDrop`'s contained
    /// value. As such, it has the same safety requirements as calling
    /// [`ManuallyDrop::drop`] directly.
    pub unsafe fn take(r: &'a mut ManuallyDrop<T>) -> Self {
        Dyn {
            ptr: NonNull::from(&mut *(r as *mut ManuallyDrop<T> as *mut T)),
            drop: |ptr| drop_in_place(ptr.as_ptr()),
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: ?Sized> From<Box<T>> for Dyn<'a, T> {
    fn from(b: Box<T>) -> Self {
        unsafe {
            Dyn {
                ptr: NonNull::new_unchecked(Box::into_raw(b)),
                drop: |ptr| drop(Box::from_raw(ptr.as_ptr())),
                marker: PhantomData,
            }
        }
    }
}
