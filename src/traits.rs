use crate::{Dyn, DynMut};
use core::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

impl<T: ?Sized> Drop for DynMut<'_, T> {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.ptr) }
    }
}

impl<T: ?Sized> Drop for Dyn<'_, T> {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.ptr) }
    }
}

impl<T: ?Sized> Deref for DynMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> Deref for Dyn<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for DynMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<A, B> PartialEq<&'_ B> for DynMut<'_, A>
where
    A: ?Sized + PartialEq<B>,
    B: ?Sized,
{
    fn eq(&self, other: &&'_ B) -> bool {
        **self == **other
    }
}

impl<A, B> PartialEq<&'_ B> for Dyn<'_, A>
where
    A: ?Sized + PartialEq<B>,
    B: ?Sized,
{
    fn eq(&self, other: &&'_ B) -> bool {
        **self == **other
    }
}

impl<A, B> PartialEq<&'_ mut B> for DynMut<'_, A>
where
    A: ?Sized + PartialEq<B>,
    B: ?Sized,
{
    fn eq(&self, other: &&'_ mut B) -> bool {
        **self == **other
    }
}

impl<A, B> PartialEq<&'_ mut B> for Dyn<'_, A>
where
    A: ?Sized + PartialEq<B>,
    B: ?Sized,
{
    fn eq(&self, other: &&'_ mut B) -> bool {
        **self == **other
    }
}

impl<A, B> PartialOrd<&'_ B> for DynMut<'_, A>
where
    A: ?Sized + PartialOrd<B>,
    B: ?Sized,
{
    fn partial_cmp(&self, other: &&'_ B) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<A, B> PartialOrd<&'_ B> for Dyn<'_, A>
where
    A: ?Sized + PartialOrd<B>,
    B: ?Sized,
{
    fn partial_cmp(&self, other: &&'_ B) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<A, B> PartialOrd<&'_ mut B> for DynMut<'_, A>
where
    A: ?Sized + PartialOrd<B>,
    B: ?Sized,
{
    fn partial_cmp(&self, other: &&'_ mut B) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<A, B> PartialOrd<&'_ mut B> for Dyn<'_, A>
where
    A: ?Sized + PartialOrd<B>,
    B: ?Sized,
{
    fn partial_cmp(&self, other: &&'_ mut B) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}
