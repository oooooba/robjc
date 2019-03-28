#[macro_use]
extern crate lazy_static;

mod category;
mod class;
mod context;
mod ivar;
mod message;
mod method;
mod module;
mod object;
pub mod runtime;
mod selector;
mod str_ptr;

use std::convert;
use std::mem;
use std::os::raw;
use std::ptr;

use class::ObjcClass;
use context::CONTEXT;
use ivar::ObjcIvar;
use method::{CodePtr, ObjcMethod};
use module::ObjcModule;
use object::ObjcObject;
use selector::ObjcSelector;

type UShort = i16;
type Int = i32;
type Long = i64;
type ULong = u64;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Bool(raw::c_uchar);

impl convert::From<bool> for Bool {
    fn from(b: bool) -> Self {
        if b {
            YES
        } else {
            NO
        }
    }
}
pub const YES: Bool = Bool(1u8);
pub const NO: Bool = Bool(0u8);

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Ptr<T>(ptr::NonNull<T>);

impl<T> Ptr<T> {
    pub unsafe fn new(ptr: *mut T) -> Ptr<T> {
        Ptr(ptr::NonNull::new_unchecked(ptr))
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

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NilablePtr<T>(Option<Ptr<T>>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Ivar<'a>(Option<&'a ObjcIvar>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Class<'a>(Option<&'a ObjcClass<'a>>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Id<'a>(Option<&'a ObjcObject<'a>>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Sel(NilablePtr<ObjcSelector>);

#[repr(transparent)]
#[derive(Clone)]
pub struct Imp<'a>(CodePtr<'a>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Method<'a>(Option<&'a ObjcMethod<'a>>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Module<'a>(ptr::NonNull<ObjcModule<'a>>);

#[no_mangle]
pub extern "C" fn __objc_exec_class(module: &'static ObjcModule) {
    let mut ctx = CONTEXT.write().unwrap();
    ctx.load_module(module);
}

#[cfg(test)]
mod tests {
    use super::str_ptr::StrPtr;
    use super::{Class, Id, Imp, Ivar, Method, Module, NilablePtr, Ptr, Sel};
    use std::mem;

    #[test]
    fn object_size() {
        assert_eq!(mem::size_of::<Ptr<()>>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<NilablePtr<()>>(), mem::size_of::<usize>());

        assert_eq!(mem::size_of::<Ivar>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Id>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Class>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Sel>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Imp>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Method>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Module>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<StrPtr>(), mem::size_of::<usize>());
    }
}
