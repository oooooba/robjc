use std::cmp;
use std::convert;
use std::hash;
use std::mem;
use std::ops;
use std::ptr;

#[repr(transparent)]
#[derive(Debug)]
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

    pub fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
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

impl<T> ops::DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> cmp::PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> cmp::Eq for Ptr<T> {}

impl<T> hash::Hash for Ptr<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
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

    pub fn as_ref(&self) -> Option<&Ptr<T>> {
        self.0.as_ref()
    }
}

impl<T> convert::From<Option<Ptr<T>>> for NilablePtr<T> {
    fn from(ptr: Option<Ptr<T>>) -> Self {
        NilablePtr::wrap(ptr)
    }
}
