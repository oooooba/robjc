use std::convert;
use std::mem;
use std::ops;
use std::ptr;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Ptr<T>(ptr::NonNull<T>);

impl<T> Ptr<T> {
    pub unsafe fn new(ptr: *const T) -> Ptr<T> {
        Ptr(ptr::NonNull::new_unchecked(ptr as *mut T))
    }

    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    pub fn as_ref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        unsafe { mem::transmute_copy(self) }
    }
}

impl<T> ops::Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NilablePtr<T>(Option<Ptr<T>>);

impl<T> NilablePtr<T> {
    #[allow(dead_code)]
    pub fn new(ptr: Ptr<T>) -> NilablePtr<T> {
        NilablePtr(Some(ptr))
    }

    fn wrap(ptr: Option<Ptr<T>>) -> NilablePtr<T> {
        NilablePtr(ptr)
    }

    pub fn nil() -> NilablePtr<T> {
        NilablePtr(None)
    }

    pub fn as_ref(&self) -> Option<Ptr<T>> {
        self.0.clone()
    }
}

impl<T> convert::From<Option<Ptr<T>>> for NilablePtr<T> {
    fn from(ptr: Option<Ptr<T>>) -> Self {
        NilablePtr::wrap(ptr)
    }
}
