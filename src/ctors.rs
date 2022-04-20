use crate::{Dyn, DynMut};
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
    pin::Pin,
    ptr::{drop_in_place, NonNull},
};

impl<'a, T: ?Sized> From<DynMut<'a, T>> for Dyn<'a, T> {
    fn from(dm: DynMut<'a, T>) -> Self {
        let dm = ManuallyDrop::new(dm);
        Dyn {
            ptr: dm.ptr,
            drop: dm.drop,
            marker: dm.marker,
        }
    }
}

impl<'a, T: ?Sized> From<&'a T> for Dyn<'a, T> {
    fn from(r: &'a T) -> Self {
        Dyn {
            ptr: NonNull::from(r),
            drop: |_| (),
            marker: PhantomData,
        }
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for DynMut<'a, T> {
    fn from(r: &'a mut T) -> Self {
        DynMut {
            ptr: NonNull::from(r),
            drop: |_| (),
            marker: PhantomData,
        }
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for Dyn<'a, T> {
    fn from(r: &'a mut T) -> Self {
        Dyn {
            ptr: NonNull::from(r),
            drop: |_| (),
            marker: PhantomData,
        }
    }
}

impl<'a, T: ?Sized> DynMut<'a, T> {
    pub fn pinned<P>(p: Pin<P>) -> Pin<Self>
    where
        P: Deref<Target = T>,
        Self: From<P>,
    {
        unsafe { Pin::new_unchecked(Self::from(Pin::into_inner_unchecked(p))) }
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

impl<'a, T: ?Sized> DynMut<'a, T> {
    /// # Safety
    ///
    /// This function runs the destructor of the `ManuallyDrop`'s contained
    /// value. As such, it has the same safety requirements as calling
    /// [`ManuallyDrop::drop`] directly.
    pub unsafe fn take(r: &'a mut ManuallyDrop<T>) -> Self {
        DynMut {
            ptr: NonNull::from(&mut *(r as *mut ManuallyDrop<T> as *mut T)),
            drop: |ptr| drop_in_place(ptr.as_ptr()),
            marker: PhantomData,
        }
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
impl<'a, T: ?Sized> From<Rc<T>> for Dyn<'a, T> {
    fn from(b: Rc<T>) -> Self {
        unsafe {
            Dyn {
                ptr: NonNull::new_unchecked(Rc::into_raw(b) as *mut T),
                drop: |ptr| drop(Rc::from_raw(ptr.as_ptr())),
                marker: PhantomData,
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: ?Sized> From<Arc<T>> for Dyn<'a, T> {
    fn from(b: Arc<T>) -> Self {
        unsafe {
            Dyn {
                ptr: NonNull::new_unchecked(Arc::into_raw(b) as *mut T),
                drop: |ptr| drop(Arc::from_raw(ptr.as_ptr())),
                marker: PhantomData,
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: ?Sized> From<Box<T>> for DynMut<'a, T> {
    fn from(b: Box<T>) -> Self {
        unsafe {
            DynMut {
                ptr: NonNull::new_unchecked(Box::into_raw(b)),
                drop: |ptr| drop(Box::from_raw(ptr.as_ptr())),
                marker: PhantomData,
            }
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
